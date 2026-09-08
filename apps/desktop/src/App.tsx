import { useEffect, useMemo, useRef, useState } from 'react';
import { GraphCanvas, type LayoutName } from './components/GraphCanvas';
import { Inspector } from './components/Inspector';
import { Sidebar, type NodeKindFilter, type NodeFilterKind } from './components/Sidebar';
import {
    chooseProjectFolder,
    composeProjectLayers,
    desktopRuntimeAvailable,
    listenForProjectDrop,
    openProjectSource,
    scanProject,
    updateProjectConfiguration,
} from './lib/desktop';
import { demoScan } from './lib/demoGraph';
import type {
    ArchitectureGraph,
    ArchitectureLayer,
    DesktopLayerGroup,
    GraphSelection,
    ProjectConfiguration,
    ProjectScan,
    SourceLocation,
} from './types';

const defaultFilters: NodeKindFilter = {
    architecture: true,
    reference: true,
    subreference: true,
};

function defaultLayout(configuration: ProjectConfiguration): LayoutName {
    if (
        configuration.default_view === 'directed'
        || configuration.default_view === 'organic'
        || configuration.default_view === 'sticky'
    ) {
        return configuration.default_view;
    }
    return 'directed';
}

function defaultLayerGroups(layers: ArchitectureLayer[]): DesktopLayerGroup[] {
    return layers.map((layer) => ({
        id: `layer:${layer.id}`,
        name: layer.displayName,
        enabled: true,
        layerIds: [layer.id],
    }));
}

function isRecord(value: unknown): value is Record<string, unknown> {
    return Boolean(value) && typeof value === 'object' && !Array.isArray(value);
}

function layerGroupsFromConfiguration(
    configuration: ProjectConfiguration,
    layers: ArchitectureLayer[],
): DesktopLayerGroup[] {
    const desktop = configuration.view_settings.desktop;
    if (!isRecord(desktop) || !Array.isArray(desktop.layer_groups)) {
        return defaultLayerGroups(layers);
    }

    const knownLayers = new Set(layers.map((layer) => layer.id));
    const seenLayers = new Set<string>();
    const seenGroupIds = new Set<string>();
    const groups: DesktopLayerGroup[] = [];

    for (const raw of desktop.layer_groups) {
        if (!isRecord(raw) || typeof raw.id !== 'string' || !Array.isArray(raw.layer_ids)) continue;
        const layerIds = raw.layer_ids.filter((layerId): layerId is string => {
            if (typeof layerId !== 'string' || !knownLayers.has(layerId) || seenLayers.has(layerId)) {
                return false;
            }
            seenLayers.add(layerId);
            return true;
        });
        if (layerIds.length === 0) continue;
        const baseId = raw.id.trim() || `group:${groups.length + 1}`;
        let id = baseId;
        let suffix = 2;
        while (seenGroupIds.has(id)) {
            id = `${baseId}:${suffix}`;
            suffix += 1;
        }
        seenGroupIds.add(id);
        groups.push({
            id,
            name: typeof raw.name === 'string' && raw.name.trim() ? raw.name : layerIds.join(' + '),
            enabled: typeof raw.enabled === 'boolean' ? raw.enabled : true,
            layerIds,
        });
    }

    for (const layer of layers) {
        if (seenLayers.has(layer.id)) continue;
        const baseId = `layer:${layer.id}`;
        let id = baseId;
        let suffix = 2;
        while (seenGroupIds.has(id)) {
            id = `${baseId}:${suffix}`;
            suffix += 1;
        }
        seenGroupIds.add(id);
        groups.push({
            id,
            name: layer.displayName,
            enabled: true,
            layerIds: [layer.id],
        });
    }

    return groups.length > 0 ? groups : defaultLayerGroups(layers);
}

function configurationWithDesktopSession(
    configuration: ProjectConfiguration,
    groups: DesktopLayerGroup[],
    layout: LayoutName,
): ProjectConfiguration {
    const currentDesktop = isRecord(configuration.view_settings.desktop)
        ? configuration.view_settings.desktop
        : {};
    return {
        ...configuration,
        default_view: layout,
        view_settings: {
            ...configuration.view_settings,
            desktop: {
                ...currentDesktop,
                layer_groups: groups.map((group) => ({
                    id: group.id,
                    name: group.name,
                    enabled: group.enabled,
                    layer_ids: group.layerIds,
                })),
            },
        },
    };
}

export default function App() {
    const [project, setProject] = useState<ProjectScan | null>(null);
    const [graph, setGraph] = useState<ArchitectureGraph | null>(null);
    const [layerGroups, setLayerGroups] = useState<DesktopLayerGroup[]>([]);
    const [layout, setLayout] = useState<LayoutName>('directed');
    const [selection, setSelection] = useState<GraphSelection>(null);
    const [filters, setFilters] = useState(defaultFilters);
    const [search, setSearch] = useState('');
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [dropHover, setDropHover] = useState(false);
    const [sourceOpenError, setSourceOpenError] = useState<string | null>(null);
    const [configurationSaving, setConfigurationSaving] = useState(false);
    const searchRef = useRef<HTMLInputElement | null>(null);
    const compositionRevision = useRef(0);
    const saveQueue = useRef<Promise<void>>(Promise.resolve());
    const desktop = useMemo(() => desktopRuntimeAvailable(), []);

    const composeGroups = async (scan: ProjectScan, groups: DesktopLayerGroup[]) => {
        const revision = ++compositionRevision.current;
        if (!desktop) {
            if (revision === compositionRevision.current) setGraph(scan.graph);
            return;
        }

        try {
            const composed = await composeProjectLayers(
                scan.graph.project,
                scan.layers,
                scan.diagnostics,
                groups
                    .filter((group) => group.enabled)
                    .map(({ id, name, layerIds }) => ({ id, name, layerIds })),
            );
            if (revision === compositionRevision.current) setGraph(composed);
        } catch (reason) {
            if (revision === compositionRevision.current) {
                setError(reason instanceof Error ? reason.message : String(reason));
            }
        }
    };

    const loadScan = (scan: ProjectScan) => {
        const groups = layerGroupsFromConfiguration(scan.configuration, scan.layers);
        const nextLayout = defaultLayout(scan.configuration);
        setProject(scan);
        setGraph(scan.graph);
        setLayerGroups(groups);
        setLayout(nextLayout);
        setSelection(null);
        void composeGroups(scan, groups);
    };

    const loadRoot = async (root: string) => {
        setLoading(true);
        setError(null);
        setSelection(null);
        try {
            loadScan(await scanProject(root));
        } catch (reason) {
            setError(reason instanceof Error ? reason.message : String(reason));
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        if (!desktop) return;
        let disposed = false;
        let unlisten: (() => void) | undefined;
        void listenForProjectDrop(
            (paths) => {
                if (paths[0]) void loadRoot(paths[0]);
            },
            setDropHover,
        ).then((result) => {
            if (disposed) result();
            else unlisten = result;
        });
        return () => {
            disposed = true;
            unlisten?.();
        };
    }, [desktop]);

    useEffect(() => {
        const onKeyDown = (event: KeyboardEvent) => {
            if (event.key === '/' && !event.metaKey && !event.ctrlKey && !event.altKey) {
                const target = event.target as HTMLElement | null;
                if (target && ['INPUT', 'TEXTAREA'].includes(target.tagName)) return;
                event.preventDefault();
                searchRef.current?.focus();
            } else if (event.key === 'Escape') {
                setSelection(null);
                searchRef.current?.blur();
            }
        };
        window.addEventListener('keydown', onKeyDown);
        return () => window.removeEventListener('keydown', onKeyDown);
    }, []);

    const persistSession = (
        groups: DesktopLayerGroup[],
        nextLayout: LayoutName,
        createConfiguration = false,
    ): Promise<void> => {
        if (!desktop || !project || (!project.configurationExists && !createConfiguration)) {
            return Promise.resolve();
        }

        setConfigurationSaving(true);
        const root = project.graph.project.root;
        const configuration = configurationWithDesktopSession(
            project.configuration,
            groups,
            nextLayout,
        );

        const work = saveQueue.current
            .catch(() => undefined)
            .then(async () => {
                const written = await updateProjectConfiguration(root, configuration);
                setProject((current) => current && current.graph.project.root === root
                    ? { ...current, configuration: written, configurationExists: true }
                    : current);
            });
        saveQueue.current = work;
        void work
            .catch((reason) => {
                setError(reason instanceof Error ? reason.message : String(reason));
            })
            .finally(() => {
                if (saveQueue.current === work) setConfigurationSaving(false);
            });
        return work;
    };

    const changeLayerGroups = (groups: DesktopLayerGroup[]) => {
        setLayerGroups(groups);
        setSelection(null);
        if (project) void composeGroups(project, groups);
        void persistSession(groups, layout);
    };

    const changeLayout = (nextLayout: LayoutName) => {
        setLayout(nextLayout);
        void persistSession(layerGroups, nextLayout);
    };

    const createConfiguration = () => {
        void persistSession(layerGroups, layout, true);
    };

    const openSource = (source: SourceLocation) => {
        if (!desktop || !graph) return;
        setSourceOpenError(null);
        void openProjectSource(graph.project.root, source).catch((reason) => {
            const message = reason instanceof Error ? reason.message : String(reason);
            console.error('Could not open source declaration', reason);
            setSourceOpenError(message);
        });
    };

    const openFolder = async () => {
        if (!desktop) return;
        const root = await chooseProjectFolder();
        if (root) await loadRoot(root);
    };

    const refreshProject = async () => {
        if (!desktop || !project) return;

        setLoading(true);
        setError(null);
        setSelection(null);

        try {
            const scan = await scanProject(project.graph.project.root);

            // A session-aware project should reload its persisted configuration.
            if (scan.configurationExists) {
                loadScan(scan);
                return;
            }

            // Session-blind projects keep their current desktop arrangement.
            const sessionConfiguration = configurationWithDesktopSession(
                scan.configuration,
                layerGroups,
                layout,
            );
            const groups = layerGroupsFromConfiguration(
                sessionConfiguration,
                scan.layers,
            );

            setProject(scan);
            setGraph(scan.graph);
            setLayerGroups(groups);
            void composeGroups(scan, groups);
        } catch (reason) {
            setError(reason instanceof Error ? reason.message : String(reason));
        } finally {
            setLoading(false);
        }
    };

    if (!project || !graph) {
        return (
            <main className={`launch-screen ${dropHover ? 'drop-hover' : ''}`}>
                <div className="launch-card">
                    <div className="brand-mark">AG</div>
                    <h1>ArchGraph</h1>
                    <p>Explore explicit architectural references in a project.</p>
                    {desktop ? (
                        <div className="drop-zone">
                            <strong>{loading ? 'Scanning project…' : 'Drop a project folder'}</strong>
                            <span>or</span>
                            <button disabled={loading} onClick={openFolder}>Open Folder</button>
                        </div>
                    ) : (
                        <div className="drop-zone browser-preview">
                            <strong>Browser preview</strong>
                            <p>Folder scanning is available in the Tauri desktop runtime.</p>
                            <button onClick={() => loadScan(demoScan)}>Load Demo Graph</button>
                        </div>
                    )}
                    {error ? <div className="error-box">{error}</div> : null}
                </div>
            </main>
        );
    }

    return (
        <main className={`app-shell ${dropHover ? 'drop-hover' : ''}`}>
            <header className="topbar">
                <div className="brand"><span className="brand-mark small">AG</span><strong>ArchGraph</strong></div>
                    <button
                        className="project-button"
                        disabled={loading}
                        onClick={desktop ? openFolder : () => {
                            setProject(null);
                            setGraph(null);
                        }}
                    >
                        {desktop ? 'Open Project' : 'Back'}
                    </button>

                    {desktop ? (
                        <button
                            className="project-button"
                            disabled={loading}
                            onClick={refreshProject}
                        >
                            {loading ? 'Refreshing…' : 'Refresh Project'}
                        </button>
                    ) : null}
                <div className="search-wrap">
                    <span>⌕</span>
                    <input
                        ref={searchRef}
                        value={search}
                        onChange={(event) => setSearch(event.target.value)}
                        placeholder="Search nodes  /"
                    />
                </div>
            </header>
            <div className="workspace">
                <Sidebar
                    graph={graph}
                    layers={project.layers}
                    layerGroups={layerGroups}
                    configurationExists={project.configurationExists}
                    canCreateConfiguration={desktop}
                    configurationSaving={configurationSaving}
                    filters={filters}
                    onFilterChange={(kind: NodeFilterKind, enabled) => setFilters((current) => ({ ...current, [kind]: enabled }))}
                    onLayerGroupsChange={changeLayerGroups}
                    onCreateConfiguration={createConfiguration}
                />
                <GraphCanvas
                    key={graph.project.root}
                    graph={graph}
                    configuration={project.configuration}
                    filters={filters}
                    search={search}
                    selection={selection}
                    layout={layout}
                    onLayoutChange={changeLayout}
                    onSelectionChange={setSelection}
                    onOpenSource={openSource}
                />
                <Inspector graph={graph} selection={selection} onOpenSource={openSource} />
            </div>
            {sourceOpenError ? (
                <div className="source-error-toast" role="alert">
                    <strong>Could not open source</strong>
                    <span>{sourceOpenError}</span>
                    <button onClick={() => setSourceOpenError(null)}>Dismiss</button>
                </div>
            ) : null}
            <footer className="statusbar">
                <span>{graph.nodes.length} nodes</span>
                <span>{graph.edges.length} references</span>
                <span>{graph.groups.length} visible regions</span>
                <span>{graph.diagnostics.length} diagnostics</span>
                <span className="status-spacer" />
                <span>Left / middle / Space + drag to pan</span>
                <span>Wheel to zoom · Shift for faster zoom</span>
            </footer>
        </main>
    );
}

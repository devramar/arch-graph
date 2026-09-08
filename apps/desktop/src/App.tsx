import { useEffect, useMemo, useRef, useState } from 'react';
import { GraphCanvas } from './components/GraphCanvas';
import { Inspector } from './components/Inspector';
import { Sidebar, type NodeKindFilter, type NodeFilterKind } from './components/Sidebar';
import {
    chooseProjectFolder,
    desktopRuntimeAvailable,
    listenForProjectDrop,
    openProjectSource,
    scanProject,
} from './lib/desktop';
import { demoScan } from './lib/demoGraph';
import type { GraphSelection, ProjectScan, SourceLocation } from './types';

const defaultFilters: NodeKindFilter = {
    architecture: true,
    reference: true,
    subreference: true,
};

export default function App() {
    const [project, setProject] = useState<ProjectScan | null>(null);
    const [selection, setSelection] = useState<GraphSelection>(null);
    const [filters, setFilters] = useState(defaultFilters);
    const [search, setSearch] = useState('');
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [dropHover, setDropHover] = useState(false);
    const [sourceOpenError, setSourceOpenError] = useState<string | null>(null);
    const searchRef = useRef<HTMLInputElement | null>(null);
    const desktop = useMemo(() => desktopRuntimeAvailable(), []);
    const graph = project?.graph ?? null;

    const loadRoot = async (root: string) => {
        setLoading(true);
        setError(null);
        setSelection(null);
        try {
            setProject(await scanProject(root));
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
                            <button onClick={() => setProject(demoScan)}>Load Demo Graph</button>
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
                <button className="project-button" onClick={desktop ? openFolder : () => setProject(null)}>
                    {desktop ? 'Open Project' : 'Back'}
                </button>
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
                    filters={filters}
                    onFilterChange={(kind: NodeFilterKind, enabled) => setFilters((current) => ({ ...current, [kind]: enabled }))}
                />
                <GraphCanvas
                    key={`${graph.project.root}:${project.configuration.default_view ?? ''}`}
                    graph={graph}
                    configuration={project.configuration}
                    filters={filters}
                    search={search}
                    selection={selection}
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
                <span>{graph.diagnostics.length} diagnostics</span>
                <span className="status-spacer" />
                <span>Left / middle / Space + drag to pan</span>
                <span>Wheel to zoom · Shift for faster zoom</span>
            </footer>
        </main>
    );
}

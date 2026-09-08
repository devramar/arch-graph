import type { DragEvent } from 'react';
import type {
    ArchitectureGraph,
    ArchitectureLayer,
    ArchitectureNode,
    DesktopLayerGroup,
} from '../types';

export type NodeFilterKind = 'architecture' | 'reference' | 'subreference';
export type NodeKindFilter = Record<NodeFilterKind, boolean>;

interface SidebarProps {
    graph: ArchitectureGraph;
    layers: ArchitectureLayer[];
    layerGroups: DesktopLayerGroup[];
    configurationExists: boolean;
    canCreateConfiguration: boolean;
    configurationSaving: boolean;
    filters: NodeKindFilter;
    onFilterChange: (kind: NodeFilterKind, enabled: boolean) => void;
    onLayerGroupsChange: (groups: DesktopLayerGroup[]) => void;
    onCreateConfiguration: () => void;
}

const labels: Record<NodeFilterKind, string> = {
    architecture: 'Architecture',
    reference: 'References',
    subreference: 'Subreferences',
};

export function filterKindForNode(node: ArchitectureNode): NodeFilterKind {
    if (node.kind === 'architecture') return 'architecture';
    return node.referenceScope === 'local' ? 'subreference' : 'reference';
}

function dragPayload(event: DragEvent): { layerId: string } | null {
    try {
        const raw = event.dataTransfer.getData('application/x-archgraph-layer');
        if (!raw) return null;
        const parsed = JSON.parse(raw) as { layerId?: unknown };
        return typeof parsed.layerId === 'string' ? { layerId: parsed.layerId } : null;
    } catch {
        return null;
    }
}

function uniqueGroupId(groups: DesktopLayerGroup[], layerId: string): string {
    const base = `layer:${layerId}`;
    if (!groups.some((group) => group.id === base)) return base;
    let index = 2;
    while (groups.some((group) => group.id === `${base}:${index}`)) index += 1;
    return `${base}:${index}`;
}

function groupName(layerIds: string[], layers: ArchitectureLayer[]): string {
    const names = new Map(layers.map((layer) => [layer.id, layer.displayName]));
    return layerIds.map((layerId) => names.get(layerId) ?? layerId).join(' + ');
}

export function Sidebar({
    graph,
    layers,
    layerGroups,
    configurationExists,
    canCreateConfiguration,
    configurationSaving,
    filters,
    onFilterChange,
    onLayerGroupsChange,
    onCreateConfiguration,
}: SidebarProps) {
    const counts = graph.nodes.reduce<Record<NodeFilterKind, number>>(
        (result, node) => {
            result[filterKindForNode(node)] += 1;
            return result;
        },
        { architecture: 0, reference: 0, subreference: 0 },
    );
    const layerById = new Map(layers.map((layer) => [layer.id, layer]));

    const moveLayer = (layerId: string, targetGroupId: string | null) => {
        const existing = layerGroups.find((group) => group.layerIds.includes(layerId));
        if (targetGroupId && existing?.id === targetGroupId) return;

        let next = layerGroups
            .map((group) => ({ ...group, layerIds: group.layerIds.filter((id) => id !== layerId) }))
            .filter((group) => group.layerIds.length > 0);

        if (targetGroupId) {
            next = next.map((group) => group.id === targetGroupId
                ? { ...group, layerIds: [...group.layerIds, layerId] }
                : group);
        } else {
            next.push({
                id: uniqueGroupId(next, layerId),
                name: layerById.get(layerId)?.displayName ?? layerId,
                enabled: true,
                layerIds: [layerId],
            });
        }

        next = next.map((group) => ({
            ...group,
            name: groupName(group.layerIds, layers),
        }));
        onLayerGroupsChange(next);
    };

    return (
        <aside className="sidebar">
            <section>
                <div className="eyebrow">Project</div>
                <h2>{graph.project.name}</h2>
                <p className="project-root" title={graph.project.root}>{graph.project.root}</p>
            </section>

            <section>
                <div className="section-heading-row">
                    <h3>Layers</h3>
                    <span className={`session-state ${configurationExists ? 'saved' : 'session-only'}`}>
                        {configurationExists ? 'saved' : 'session only'}
                    </span>
                </div>
                <p className="layer-help">Drag layers together to merge matching node names. Separate groups remain visible as separate graph regions.</p>
                <div className="layer-group-list">
                    {layerGroups.map((group) => (
                        <div
                            className={`layer-group-card ${group.enabled ? '' : 'disabled'}`}
                            key={group.id}
                            onDragOver={(event) => {
                                event.preventDefault();
                                event.dataTransfer.dropEffect = 'move';
                            }}
                            onDrop={(event) => {
                                event.preventDefault();
                                const payload = dragPayload(event);
                                if (payload) moveLayer(payload.layerId, group.id);
                            }}
                        >
                            <label className="layer-group-heading">
                                <input
                                    type="checkbox"
                                    checked={group.enabled}
                                    onChange={(event) => onLayerGroupsChange(layerGroups.map((candidate) => candidate.id === group.id
                                        ? { ...candidate, enabled: event.target.checked }
                                        : candidate))}
                                />
                                <span>{group.name}</span>
                            </label>
                            <div className="layer-chip-list">
                                {group.layerIds.map((layerId) => {
                                    const layer = layerById.get(layerId);
                                    if (!layer) return null;
                                    return (
                                        <div
                                            className="layer-chip"
                                            draggable
                                            key={layerId}
                                            title={`${layer.displayName} · ${layer.graph.nodes.filter((node) => node.kind === 'architecture').length} declarations`}
                                            onDragStart={(event) => {
                                                event.dataTransfer.effectAllowed = 'move';
                                                event.dataTransfer.setData(
                                                    'application/x-archgraph-layer',
                                                    JSON.stringify({ layerId }),
                                                );
                                            }}
                                        >
                                            <span className="layer-grip">⋮⋮</span>
                                            <span>{layer.displayName}</span>
                                        </div>
                                    );
                                })}
                            </div>
                        </div>
                    ))}
                    <div
                        className="new-layer-group-drop"
                        onDragOver={(event) => {
                            event.preventDefault();
                            event.dataTransfer.dropEffect = 'move';
                        }}
                        onDrop={(event) => {
                            event.preventDefault();
                            const payload = dragPayload(event);
                            if (payload) moveLayer(payload.layerId, null);
                        }}
                    >
                        Drop a layer here to split it into a new group
                    </div>
                </div>
                {!configurationExists && canCreateConfiguration ? (
                    <button
                        className="create-config-button"
                        disabled={configurationSaving}
                        onClick={onCreateConfiguration}
                    >
                        {configurationSaving ? 'Creating…' : 'Create .archgraph · save this session'}
                    </button>
                ) : configurationSaving ? (
                    <p className="config-saving">Saving session…</p>
                ) : null}
            </section>

            <section>
                <h3>Nodes</h3>
                <div className="filter-list">
                    {(Object.keys(labels) as NodeFilterKind[]).map((kind) => (
                        <label className="filter-row" key={kind}>
                            <input
                                type="checkbox"
                                checked={filters[kind]}
                                onChange={(event) => onFilterChange(kind, event.target.checked)}
                            />
                            <span className={`node-dot node-dot-${kind}`} />
                            <span>{labels[kind]}</span>
                            <span className="filter-count">{counts[kind]}</span>
                        </label>
                    ))}
                </div>
            </section>

            <section>
                <h3>Diagnostics</h3>
                {graph.diagnostics.length === 0 ? (
                    <p className="muted">No diagnostics.</p>
                ) : (
                    <div className="diagnostic-list">
                        {graph.diagnostics.slice(0, 8).map((diagnostic, index) => (
                            <div className="diagnostic" key={`${diagnostic.code}:${index}`}>
                                <strong>{diagnostic.code}</strong>
                                <span>{diagnostic.message}</span>
                            </div>
                        ))}
                    </div>
                )}
            </section>
        </aside>
    );
}

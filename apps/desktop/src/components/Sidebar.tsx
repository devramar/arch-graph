import type { ArchitectureGraph, NodeKind } from '../types';

export type NodeKindFilter = Record<NodeKind, boolean>;

interface SidebarProps {
    graph: ArchitectureGraph;
    filters: NodeKindFilter;
    onFilterChange: (kind: NodeKind, enabled: boolean) => void;
}

const labels: Record<NodeKind, string> = {
    architecture: 'Architecture',
    module: 'Modules',
    external: 'External',
    unresolved: 'Unresolved',
};

export function Sidebar({ graph, filters, onFilterChange }: SidebarProps) {
    const counts = graph.nodes.reduce<Record<NodeKind, number>>(
        (result, node) => {
            result[node.kind] += 1;
            return result;
        },
        { architecture: 0, module: 0, external: 0, unresolved: 0 },
    );

    return (
        <aside className="sidebar">
            <section>
                <div className="eyebrow">Project</div>
                <h2>{graph.project.name}</h2>
                <p className="project-root" title={graph.project.root}>{graph.project.root}</p>
            </section>

            <section>
                <h3>Nodes</h3>
                <div className="filter-list">
                    {(Object.keys(labels) as NodeKind[]).map((kind) => (
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

import type { ArchitectureGraph, GraphSelection } from '../types';

interface InspectorProps {
    graph: ArchitectureGraph;
    selection: GraphSelection;
}

function Source({ file, line }: { file: string; line?: number | null }) {
    return (
        <div className="source-path">
            <span>{file}</span>
            {line ? <span className="source-line">:{line}</span> : null}
        </div>
    );
}

export function Inspector({ graph, selection }: InspectorProps) {
    if (!selection) {
        return (
            <aside className="inspector empty-panel">
                <div>
                    <strong>Inspector</strong>
                    <p>Select a node or relationship.</p>
                </div>
            </aside>
        );
    }

    if (selection.kind === 'node') {
        const node = selection.item;
        const inbound = graph.edges.filter((edge) => edge.target === node.id).length;
        const outbound = graph.edges.filter((edge) => edge.source === node.id).length;

        return (
            <aside className="inspector">
                <div className="eyebrow">{node.kind}</div>
                <h2>{node.name}</h2>
                <div className="metrics-row">
                    <span>{inbound} incoming</span>
                    <span>{outbound} outgoing</span>
                </div>
                {node.summary ? <p className="inspector-copy">{node.summary}</p> : null}
                {node.source ? (
                    <section>
                        <h3>Source</h3>
                        <Source file={node.source.file} line={node.source.line} />
                    </section>
                ) : null}
            </aside>
        );
    }

    const edge = selection.item;
    const source = graph.nodes.find((node) => node.id === edge.source);
    const target = graph.nodes.find((node) => node.id === edge.target);

    return (
        <aside className="inspector">
            <div className="eyebrow">relationship</div>
            <h2 className="edge-title">
                {source?.name ?? edge.source}
                <span>→</span>
                {target?.name ?? edge.targetName}
            </h2>
            <div className={`resolution resolution-${edge.resolution}`}>{edge.resolution}</div>
            <section>
                <h3>Dependency</h3>
                <p className="inspector-copy">{edge.description || 'No relationship description was provided.'}</p>
            </section>
            <section>
                <h3>Declared at</h3>
                <Source file={edge.sourceLocation.file} line={edge.sourceLocation.line} />
            </section>
        </aside>
    );
}

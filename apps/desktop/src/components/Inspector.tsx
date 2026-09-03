import type { ArchitectureGraph, GraphSelection, SourceLocation } from '../types';
import { MarkdownDocument } from './MarkdownDocument';

interface InspectorProps {
    graph: ArchitectureGraph;
    selection: GraphSelection;
    onOpenSource: (source: SourceLocation) => void;
}

function Source({ source, onOpenSource }: { source: SourceLocation; onOpenSource: (source: SourceLocation) => void }) {
    return (
        <div className="source-block">
            <div className="source-path">
                <span>{source.file}</span>
                {source.line ? <span className="source-line">:{source.line}</span> : null}
            </div>
            <button className="source-open-button" onClick={() => onOpenSource(source)}>Open in editor</button>
        </div>
    );
}

export function Inspector({ graph, selection, onOpenSource }: InspectorProps) {
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
                        <Source source={node.source} onOpenSource={onOpenSource} />
                    </section>
                ) : null}
                {node.documentation ? (
                    <section className="architecture-document-section">
                        <h3>Architecture document</h3>
                        <MarkdownDocument markdown={node.documentation} />
                    </section>
                ) : node.kind === 'architecture' ? (
                    <section>
                        <h3>Architecture document</h3>
                        <p className="muted">No document content was included in this graph.</p>
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
                <h3>Declaration</h3>
                <Source source={edge.sourceLocation} onOpenSource={onOpenSource} />
            </section>
        </aside>
    );
}

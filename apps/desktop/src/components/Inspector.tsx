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
                    <p>Select a node or reference.</p>
                </div>
            </aside>
        );
    }

    if (selection.kind === 'node') {
        const node = selection.item;
        const inboundEdges = graph.edges.filter((edge) => edge.target === node.id);
        const outbound = graph.edges.filter((edge) => edge.source === node.id).length;
        const referencedBy = inboundEdges
            .map((edge) => graph.nodes.find((candidate) => candidate.id === edge.source)?.name)
            .filter((name): name is string => Boolean(name));
        const matchingArchitectureDocuments = node.kind === 'reference'
            ? graph.nodes.filter((candidate) => candidate.kind === 'architecture' && candidate.name === node.name)
            : [];

        return (
            <aside className="inspector">
                <div className="eyebrow">
                    {node.kind === 'reference' && node.referenceScope === 'local' ? 'subreference' : node.kind}
                </div>
                <h2>{node.name}</h2>
                <div className="metrics-row">
                    <span>{inboundEdges.length} incoming</span>
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
                ) : (
                    <section>
                        <h3>Architecture document</h3>
                        <p className="muted">
                            {node.referenceScope === 'local'
                                ? `This is a local subreference named \`${node.name}\`. It is scoped to this declaration and never merges or resolves to architecture documents by name.`
                                : matchingArchitectureDocuments.length > 1
                                    ? `Multiple architecture documents describe \`${node.name}\`, so this reference cannot be linked uniquely.`
                                    : `No architecture document describes \`${node.name}\`.`}
                        </p>
                    </section>
                )}
                {node.kind === 'reference' && referencedBy.length > 0 ? (
                    <section>
                        <h3>Referenced by</h3>
                        <div className="reference-source-list">
                            {[...new Set(referencedBy)].map((name) => <span key={name}>{name}</span>)}
                        </div>
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
            <div className="eyebrow">{edge.referenceKind}</div>
            <h2 className="edge-title">
                {source?.name ?? edge.source}
                <span>→</span>
                {target?.name ?? edge.targetName}
            </h2>
            <section>
                <h3>{edge.referenceKind === 'subreference' ? 'Subreference' : 'Reference'}</h3>
                <p className="inspector-copy">{edge.description || 'No reference description was provided.'}</p>
            </section>
            <section>
                <h3>Declaration</h3>
                <Source source={edge.sourceLocation} onOpenSource={onOpenSource} />
            </section>
        </aside>
    );
}

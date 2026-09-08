import { useEffect, useState } from 'react';
import type {
    ArchitectureDeclaration,
    ArchitectureGraph,
    GraphSelection,
    SourceLocation,
} from '../types';
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

function DeclarationDocumentation({ declaration }: { declaration: ArchitectureDeclaration }) {
    if (!declaration.documentation) {
        return <p className="muted">No declaration documentation was included in this graph.</p>;
    }
    if (declaration.sourceFormat === 'markdown') {
        return <MarkdownDocument markdown={declaration.documentation} />;
    }
    return <div className="decorated-documentation">{declaration.documentation}</div>;
}

export function Inspector({ graph, selection, onOpenSource }: InspectorProps) {
    const [activeDeclaration, setActiveDeclaration] = useState(0);
    const selectedNodeId = selection?.kind === 'node' ? selection.item.id : null;

    useEffect(() => {
        setActiveDeclaration(0);
    }, [selectedNodeId]);

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
        const matchingArchitectureDeclarations = node.kind === 'reference'
            ? graph.nodes.filter((candidate) => (
                candidate.kind === 'architecture'
                && candidate.groupId === node.groupId
                && candidate.name === node.name
            ))
            : [];
        const declaration = node.declarations[Math.min(activeDeclaration, Math.max(0, node.declarations.length - 1))];

        return (
            <aside className="inspector">
                <div className="eyebrow">
                    {node.kind === 'reference' && node.referenceScope === 'local' ? 'subreference' : node.kind}
                </div>
                <h2>{node.name}</h2>
                <div className="metrics-row">
                    <span>{inboundEdges.length} incoming</span>
                    <span>{outbound} outgoing</span>
                    {node.declarations.length > 1 ? <span>{node.declarations.length} sources</span> : null}
                </div>

                {node.kind === 'architecture' ? (
                    <section className="architecture-document-section">
                        <div className="inspector-section-heading">
                            <h3>Declarations</h3>
                            {node.declarations.length > 1 ? <span className="merged-source-badge">merged node</span> : null}
                        </div>
                        {node.declarations.length > 1 ? (
                            <div className="declaration-tabs" role="tablist" aria-label={`${node.name} declarations`}>
                                {node.declarations.map((candidate, index) => (
                                    <button
                                        key={`${candidate.layerId}:${candidate.source.file}:${candidate.source.line ?? ''}`}
                                        role="tab"
                                        aria-selected={index === activeDeclaration}
                                        className={index === activeDeclaration ? 'active' : ''}
                                        onClick={() => setActiveDeclaration(index)}
                                        title={candidate.source.file}
                                    >
                                        {candidate.layerName}
                                    </button>
                                ))}
                            </div>
                        ) : null}
                        {declaration ? (
                            <div className="declaration-panel">
                                <div className="declaration-layer-line">
                                    <span>{declaration.layerName}</span>
                                    <code>{declaration.layerId}</code>
                                </div>
                                <Source source={declaration.source} onOpenSource={onOpenSource} />
                                <div className="declaration-documentation">
                                    <DeclarationDocumentation declaration={declaration} />
                                </div>
                            </div>
                        ) : (
                            <p className="muted">No architecture declaration was included in this graph.</p>
                        )}
                    </section>
                ) : (
                    <section>
                        <h3>Architecture declaration</h3>
                        <p className="muted">
                            {node.referenceScope === 'local'
                                ? `This is a local subreference named \`${node.name}\`. It belongs only to its source declaration and never merges or resolves by name.`
                                : matchingArchitectureDeclarations.length > 1
                                    ? `Multiple architecture declarations describe \`${node.name}\`, so this reference cannot be linked uniquely.`
                                    : `No architecture declaration describes \`${node.name}\` in this layer group.`}
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
            <div className="metrics-row">
                <span>layer: {edge.layerId}</span>
            </div>
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

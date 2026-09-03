import cytoscape, { type Core, type EdgeSingular, type EventObject, type NodeSingular } from 'cytoscape';
import dagre from 'cytoscape-dagre';
import fcose from 'cytoscape-fcose';
import { useEffect, useMemo, useRef, useState } from 'react';
import type {
    ArchitectureEdge,
    ArchitectureGraph,
    ArchitectureNode,
    GraphSelection,
    NodeKind,
    SourceLocation,
} from '../types';
import type { NodeKindFilter } from './Sidebar';

cytoscape.use(dagre);
cytoscape.use(fcose);

type LayoutName = 'dependency' | 'organic' | 'sticky';

interface GraphCanvasProps {
    graph: ArchitectureGraph;
    filters: NodeKindFilter;
    search: string;
    selection: GraphSelection;
    onSelectionChange: (selection: GraphSelection) => void;
    onOpenSource: (source: SourceLocation) => void;
}

interface HoveredEdge {
    edge: ArchitectureEdge;
    sourceName: string;
    targetName: string;
    x: number;
    y: number;
}

interface GraphContextMenu {
    x: number;
    y: number;
    source: SourceLocation;
    label: string;
}

const DEFAULT_WHEEL_SENSITIVITY = 0.54;
const SHIFT_WHEEL_SENSITIVITY = 1.0;

const nodeStyle: cytoscape.Stylesheet[] = [
    {
        selector: 'node',
        style: {
            'background-color': '#667085',
            'border-width': 1,
            'border-color': '#98a2b3',
            color: '#f2f4f7',
            label: 'data(label)',
            'font-family': 'Inter, ui-sans-serif, system-ui, sans-serif',
            'font-size': 11,
            'text-valign': 'center',
            'text-halign': 'center',
            'text-wrap': 'wrap',
            'text-max-width': 96,
            width: 'label',
            height: 34,
            padding: 12,
            shape: 'round-rectangle',
        },
    },
    { selector: 'node[kind = "architecture"]', style: { 'background-color': '#175cd3', 'border-color': '#53b1fd' } },
    { selector: 'node[kind = "module"]', style: { 'background-color': '#344054', 'border-color': '#98a2b3' } },
    { selector: 'node[kind = "external"]', style: { 'background-color': '#027a48', 'border-color': '#6ce9a6' } },
    { selector: 'node[kind = "unresolved"]', style: { 'background-color': '#7a2e0e', 'border-color': '#fdb022', 'border-style': 'dashed' } },
    {
        selector: 'edge',
        style: {
            width: 1.5,
            'curve-style': 'bezier',
            'line-color': '#667085',
            'target-arrow-color': '#98a2b3',
            'target-arrow-shape': 'triangle',
            'arrow-scale': 0.8,
        },
    },
    {
        selector: 'node:selected',
        style: {
            'overlay-opacity': 0,
            'border-width': 3,
            'border-color': '#f2f4f7',
        },
    },
    {
        selector: 'edge:selected',
        style: {
            'overlay-opacity': 0,
            'line-color': '#d0d5dd',
            'target-arrow-color': '#f2f4f7',
            width: 3,
        },
    },
    { selector: '.dimmed', style: { opacity: 0.12 } },
    { selector: '.hidden-by-filter', style: { display: 'none' } },
];

function elementsFor(graph: ArchitectureGraph): cytoscape.ElementDefinition[] {
    return [
        ...graph.nodes.map((node) => ({
            group: 'nodes' as const,
            data: { id: node.id, label: node.name, kind: node.kind, raw: node },
        })),
        ...graph.edges.map((edge) => ({
            group: 'edges' as const,
            data: { id: edge.id, source: edge.source, target: edge.target, raw: edge },
        })),
    ];
}

function runLayout(cy: Core, layout: LayoutName, incremental = false) {
    if (layout === 'dependency') {
        cy.layout({
            name: 'dagre',
            rankDir: 'LR',
            rankSep: 90,
            nodeSep: 45,
            edgeSep: 18,
            padding: 48,
        } as cytoscape.LayoutOptions).run();
        return;
    }

    cy.layout({
        name: 'fcose',
        quality: incremental ? 'proof' : 'default',
        randomize: !incremental,
        animate: incremental,
        animationDuration: incremental ? 350 : 0,
        fit: !incremental,
        padding: 48,
        nodeRepulsion: 5200,
        idealEdgeLength: 110,
        edgeElasticity: 0.45,
        nestingFactor: 0.1,
        gravity: 0.25,
        numIter: incremental ? 900 : 2500,
        initialEnergyOnIncremental: 0.22,
    } as cytoscape.LayoutOptions).run();
}

function edgeNames(graph: ArchitectureGraph, edge: ArchitectureEdge) {
    const source = graph.nodes.find((node) => node.id === edge.source);
    const target = graph.nodes.find((node) => node.id === edge.target);
    return {
        sourceName: source?.name ?? edge.source,
        targetName: target?.name ?? edge.targetName,
    };
}

export function GraphCanvas({ graph, filters, search, selection, onSelectionChange, onOpenSource }: GraphCanvasProps) {
    const containerRef = useRef<HTMLDivElement | null>(null);
    const cyRef = useRef<Core | null>(null);
    const spaceDownRef = useRef(false);
    const panRef = useRef<{ pointerId: number; x: number; y: number } | null>(null);
    const layoutRef = useRef<LayoutName>('dependency');
    const [layout, setLayout] = useState<LayoutName>('dependency');
    const [hoveredEdge, setHoveredEdge] = useState<HoveredEdge | null>(null);
    const [contextMenu, setContextMenu] = useState<GraphContextMenu | null>(null);
    const [showEdgeDetails, setShowEdgeDetails] = useState(true);
    const elements = useMemo(() => elementsFor(graph), [graph]);

    useEffect(() => {
        const container = containerRef.current;
        if (!container) return;

        const cy = cytoscape({
            container,
            elements,
            style: nodeStyle,
            minZoom: 0.08,
            maxZoom: 6,
            wheelSensitivity: DEFAULT_WHEEL_SENSITIVITY,
            boxSelectionEnabled: false,
            selectionType: 'single',
            userPanningEnabled: true,
            userZoomingEnabled: true,
        });
        cyRef.current = cy;
        runLayout(cy, layoutRef.current);
        cy.fit(undefined, 48);

        const onNodeTap = (event: EventObject) => {
            setContextMenu(null);
            const node = event.target as NodeSingular;
            onSelectionChange({ kind: 'node', item: node.data('raw') as ArchitectureNode });
        };
        const onEdgeTap = (event: EventObject) => {
            setContextMenu(null);
            const edge = event.target as EdgeSingular;
            onSelectionChange({ kind: 'edge', item: edge.data('raw') as ArchitectureEdge });
        };
        const onBackgroundTap = (event: EventObject) => {
            setContextMenu(null);
            if (event.target === cy) onSelectionChange(null);
        };
        const updateEdgeHover = (event: EventObject) => {
            const edge = event.target as EdgeSingular;
            const raw = edge.data('raw') as ArchitectureEdge;
            const fallback = edge.renderedMidpoint();
            const point = event.renderedPosition ?? fallback;
            setHoveredEdge({
                edge: raw,
                sourceName: edge.source().data('label') as string,
                targetName: edge.target().data('label') as string,
                x: point.x,
                y: point.y,
            });
        };
        const clearHover = () => setHoveredEdge(null);
        const openNodeContext = (event: EventObject) => {
            const node = event.target as NodeSingular;
            const raw = node.data('raw') as ArchitectureNode;
            if (!raw.source) return;
            const point = event.renderedPosition ?? node.renderedPosition();
            setContextMenu({ x: point.x, y: point.y, source: raw.source, label: raw.name });
        };
        const openEdgeContext = (event: EventObject) => {
            const edge = event.target as EdgeSingular;
            const raw = edge.data('raw') as ArchitectureEdge;
            const point = event.renderedPosition ?? edge.renderedMidpoint();
            const names = edgeNames(graph, raw);
            setContextMenu({
                x: point.x,
                y: point.y,
                source: raw.sourceLocation,
                label: `${names.sourceName} → ${names.targetName}`,
            });
        };
        const settleStickyLayout = () => {
            if (layoutRef.current !== 'sticky') return;
            runLayout(cy, 'sticky', true);
        };

        cy.on('tap', 'node', onNodeTap);
        cy.on('tap', 'edge', onEdgeTap);
        cy.on('tap', onBackgroundTap);
        cy.on('mouseover mousemove', 'edge', updateEdgeHover);
        cy.on('mouseout', 'edge', clearHover);
        cy.on('cxttap', 'node', openNodeContext);
        cy.on('cxttap', 'edge', openEdgeContext);
        cy.on('dragfree', 'node', settleStickyLayout);
        cy.on('pan zoom', clearHover);

        const keyDown = (event: KeyboardEvent) => {
            const tagName = (event.target as HTMLElement | null)?.tagName;
            if (['INPUT', 'TEXTAREA'].includes(tagName ?? '')) return;

            if (event.code === 'Escape') setContextMenu(null);

            if (event.code === 'KeyF' && !event.repeat) {
                cy.fit(undefined, 48);
                event.preventDefault();
                return;
            }

            if (event.code !== 'Space' || event.repeat) return;
            spaceDownRef.current = true;
            container.classList.add('space-pan-active');
            event.preventDefault();
        };
        const keyUp = (event: KeyboardEvent) => {
            if (event.code !== 'Space') return;
            spaceDownRef.current = false;
            panRef.current = null;
            container.classList.remove('space-pan-active');
        };
        const pointerDown = (event: PointerEvent) => {
            setContextMenu(null);
            const customPan = event.button === 1 || (event.button === 0 && spaceDownRef.current);
            if (!customPan) return;
            panRef.current = { pointerId: event.pointerId, x: event.clientX, y: event.clientY };
            container.setPointerCapture(event.pointerId);
            event.preventDefault();
            event.stopPropagation();
        };
        const pointerMove = (event: PointerEvent) => {
            const pan = panRef.current;
            if (!pan || pan.pointerId !== event.pointerId) return;
            cy.panBy({ x: event.clientX - pan.x, y: event.clientY - pan.y });
            pan.x = event.clientX;
            pan.y = event.clientY;
            event.preventDefault();
        };
        const pointerEnd = (event: PointerEvent) => {
            if (panRef.current?.pointerId !== event.pointerId) return;
            panRef.current = null;
            if (container.hasPointerCapture(event.pointerId)) container.releasePointerCapture(event.pointerId);
            event.preventDefault();
        };
        const suppressAux = (event: MouseEvent) => {
            if (event.button === 1) event.preventDefault();
        };
        const suppressNativeContextMenu = (event: MouseEvent) => event.preventDefault();
        const amplifiedShiftZoom = (event: WheelEvent) => {
            if (!event.shiftKey) return;

            event.preventDefault();
            event.stopImmediatePropagation();
            const rect = container.getBoundingClientRect();
            const delta = Math.max(-180, Math.min(180, event.deltaY));
            const factor = Math.pow(10, (-delta / 250) * SHIFT_WHEEL_SENSITIVITY);
            const nextZoom = Math.max(cy.minZoom(), Math.min(cy.maxZoom(), cy.zoom() * factor));
            cy.zoom({
                level: nextZoom,
                renderedPosition: { x: event.clientX - rect.left, y: event.clientY - rect.top },
            });
        };

        window.addEventListener('keydown', keyDown, true);
        window.addEventListener('keyup', keyUp, true);
        container.addEventListener('pointerdown', pointerDown, true);
        container.addEventListener('pointermove', pointerMove, true);
        container.addEventListener('pointerup', pointerEnd, true);
        container.addEventListener('pointercancel', pointerEnd, true);
        container.addEventListener('auxclick', suppressAux);
        container.addEventListener('contextmenu', suppressNativeContextMenu);
        container.addEventListener('wheel', amplifiedShiftZoom, { capture: true, passive: false });

        return () => {
            window.removeEventListener('keydown', keyDown, true);
            window.removeEventListener('keyup', keyUp, true);
            container.removeEventListener('pointerdown', pointerDown, true);
            container.removeEventListener('pointermove', pointerMove, true);
            container.removeEventListener('pointerup', pointerEnd, true);
            container.removeEventListener('pointercancel', pointerEnd, true);
            container.removeEventListener('auxclick', suppressAux);
            container.removeEventListener('contextmenu', suppressNativeContextMenu);
            container.removeEventListener('wheel', amplifiedShiftZoom, true);
            cy.destroy();
            cyRef.current = null;
        };
    }, [elements, graph]);

    useEffect(() => {
        const cy = cyRef.current;
        if (!cy) return;
        cy.nodes().forEach((node) => {
            const kind = node.data('kind') as NodeKind;
            node.toggleClass('hidden-by-filter', !filters[kind]);
        });
        cy.edges().forEach((edge) => {
            edge.toggleClass('hidden-by-filter', edge.source().hasClass('hidden-by-filter') || edge.target().hasClass('hidden-by-filter'));
        });
    }, [filters]);

    useEffect(() => {
        const cy = cyRef.current;
        if (!cy) return;
        const query = search.trim().toLocaleLowerCase();
        cy.elements().removeClass('dimmed');
        if (!query) return;

        const matchingNodes = cy.nodes().filter((node) => (node.data('label') as string).toLocaleLowerCase().includes(query));
        const neighborhood = matchingNodes.closedNeighborhood();
        cy.elements().not(neighborhood).addClass('dimmed');
    }, [search]);

    useEffect(() => {
        const cy = cyRef.current;
        if (!cy) return;
        cy.elements().unselect();
        if (selection) cy.getElementById(selection.item.id).select();
    }, [selection]);

    const changeLayout = (next: LayoutName) => {
        setLayout(next);
        layoutRef.current = next;
        const cy = cyRef.current;
        if (!cy) return;
        runLayout(cy, next);
        cy.fit(undefined, 48);
    };

    const selectedEdge = selection?.kind === 'edge' ? selection.item : null;
    const detailEdge = hoveredEdge?.edge ?? selectedEdge;
    const detailNames = detailEdge ? edgeNames(graph, detailEdge) : null;

    return (
        <div className="graph-panel">
            <div className="graph-toolbar">
                <button onClick={() => cyRef.current?.fit(undefined, 48)}>Fit</button>
                <button
                    className={showEdgeDetails ? 'active' : ''}
                    onClick={() => setShowEdgeDetails((current) => !current)}
                    title="Show relationship descriptions at the bottom of the graph"
                >
                    Descriptions
                </button>
                <select value={layout} onChange={(event) => changeLayout(event.target.value as LayoutName)} aria-label="Graph layout">
                    <option value="dependency">Dependency</option>
                    <option value="organic">Organic</option>
                    <option value="sticky">Sticky</option>
                </select>
            </div>
            <div className="graph-canvas" ref={containerRef} />
            {hoveredEdge ? (
                <div className="edge-tooltip" style={{ left: hoveredEdge.x + 14, top: hoveredEdge.y + 14 }}>
                    <strong>{hoveredEdge.sourceName} → {hoveredEdge.targetName}</strong>
                    <p>{hoveredEdge.edge.description || 'No relationship description.'}</p>
                </div>
            ) : null}
            {showEdgeDetails && detailEdge && detailNames ? (
                <div className="edge-description-panel">
                    <strong>{detailNames.sourceName} → {detailNames.targetName}</strong>
                    <p>{detailEdge.description || 'No relationship description.'}</p>
                </div>
            ) : null}
            {contextMenu ? (
                <div className="graph-context-menu" style={{ left: contextMenu.x, top: contextMenu.y }}>
                    <div className="graph-context-label">{contextMenu.label}</div>
                    <button onClick={() => { onOpenSource(contextMenu.source); setContextMenu(null); }}>
                        Open in editor
                    </button>
                </div>
            ) : null}
        </div>
    );
}

import cytoscape, { type Core, type EdgeSingular, type EventObject, type NodeSingular } from 'cytoscape';
import dagre from 'cytoscape-dagre';
import fcose from 'cytoscape-fcose';
import { useEffect, useMemo, useRef, useState } from 'react';
import type {
    ArchitectureEdge,
    ArchitectureGraph,
    ArchitectureNode,
    GraphSelection,
    ProjectConfiguration,
    SourceLocation,
} from '../types';
import { filterKindForNode, type NodeKindFilter } from './Sidebar';

cytoscape.use(dagre);
cytoscape.use(fcose);

export type LayoutName = 'directed' | 'organic' | 'sticky';

type ColourPair = {
    name: string;
    primary: string;
    secondary: string;
};

interface GraphCanvasProps {
    graph: ArchitectureGraph;
    configuration: ProjectConfiguration;
    filters: NodeKindFilter;
    search: string;
    selection: GraphSelection;
    layout: LayoutName;
    onLayoutChange: (layout: LayoutName) => void;
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

interface RenderedRegion {
    id: string;
    name: string;
    layerIds: string[];
    left: number;
    top: number;
    width: number;
    height: number;
}

interface ViewLayoutSettings {
    referenceDistance: number;
    subreferenceDistance: number;
    nodeSpacing: number;
    subreferenceAttraction: number;
}

const DEFAULT_WHEEL_SENSITIVITY = 0.54;
const SHIFT_WHEEL_SENSITIVITY = 1.0;
const REGION_GAP = 180;
const REGION_PADDING = 28;

const ARCHITECTURE_COLOUR: ColourPair = {
    name: 'architecture',
    primary: '#53b1fd',
    secondary: '#175cd3',
};

const REFERENCE_COLOURS: ColourPair[] = [
    { name: 'purple', primary: '#b692f6', secondary: '#3e1c6d' },
    { name: 'blue', primary: '#84caff', secondary: '#194185' },
    { name: 'teal', primary: '#5fe9d0', secondary: '#134e48' },
    { name: 'green', primary: '#75e0a7', secondary: '#175c3a' },
    { name: 'orange', primary: '#fec84b', secondary: '#713b12' },
    { name: 'pink', primary: '#fda4ca', secondary: '#851651' },
    { name: 'red', primary: '#fda29b', secondary: '#912018' },
    { name: 'indigo', primary: '#a4bcfd', secondary: '#3538cd' },
];

const DEFAULT_LAYOUT_SETTINGS: Record<LayoutName, ViewLayoutSettings> = {
    directed: {
        referenceDistance: 175,
        subreferenceDistance: 72,
        nodeSpacing: 56,
        subreferenceAttraction: 1.6,
    },
    organic: {
        referenceDistance: 155,
        subreferenceDistance: 82,
        nodeSpacing: 56,
        subreferenceAttraction: 1.55,
    },
    sticky: {
        referenceDistance: 175,
        subreferenceDistance: 68,
        nodeSpacing: 62,
        subreferenceAttraction: 1.8,
    },
};

const nodeStyle: cytoscape.StylesheetJson = [
    {
        selector: 'node',
        style: {
            'background-color': 'data(backgroundColor)',
            'border-width': 2,
            'border-color': 'data(borderColor)',
            color: '#f2f4f7',
            label: 'data(label)',
            'font-family': 'Inter, ui-sans-serif, system-ui, sans-serif',
            'font-size': 11,
            'text-valign': 'center',
            'text-halign': 'center',
            'text-wrap': 'wrap',
            'text-max-width': '104',
            width: 'label',
            height: 34,
            padding: '12',
            shape: 'round-rectangle',
        },
    },
    {
        selector: 'node[referenceScope = "shared"]',
        style: {
            'font-size': 10.5,
            height: 32,
            padding: '11',
        },
    },
    {
        selector: 'node[referenceScope = "local"]',
        style: {
            'border-style': 'dashed',
            'font-size': 10,
            'text-max-width': '88',
            height: 30,
            padding: '9',
            shape: 'ellipse',
        },
    },
    {
        selector: 'edge',
        style: {
            width: 1.5,
            'curve-style': 'bezier',
            'line-color': '#667085',
            'target-arrow-color': 'data(targetColour)',
            'target-arrow-shape': 'triangle',
            'arrow-scale': 0.85,
        },
    },
    {
        selector: 'edge[referenceKind = "subreference"]',
        style: {
            width: 1.25,
            'line-color': '#596273',
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
            width: 3,
        },
    },
    { selector: '.dimmed', style: { opacity: 0.12 } },
    { selector: '.hidden-by-filter', style: { display: 'none' } },
];

function stableHash(value: string): number {
    let hash = 2166136261;
    for (let index = 0; index < value.length; index += 1) {
        hash ^= value.charCodeAt(index);
        hash = Math.imul(hash, 16777619);
    }
    return hash >>> 0;
}

function namedColour(name: string | undefined): ColourPair | undefined {
    if (!name) return undefined;
    return REFERENCE_COLOURS.find((colour) => colour.name === name);
}

function colourForNode(node: ArchitectureNode, configuration: ProjectConfiguration): ColourPair {
    if (node.kind === 'architecture') return ARCHITECTURE_COLOUR;

    const overrides = node.referenceScope === 'local'
        ? configuration.app_colours.colour_overrides.subreferences
        : configuration.app_colours.colour_overrides.references;
    const override = namedColour(overrides[node.name]);
    if (override) return override;

    return REFERENCE_COLOURS[stableHash(node.name) % REFERENCE_COLOURS.length];
}

function elementsFor(
    graph: ArchitectureGraph,
    configuration: ProjectConfiguration,
): cytoscape.ElementDefinition[] {
    const colours = new Map<string, ColourPair>();
    for (const node of graph.nodes) {
        colours.set(node.id, colourForNode(node, configuration));
    }

    return [
        ...graph.nodes.map((node) => {
            const colour = colours.get(node.id) ?? ARCHITECTURE_COLOUR;
            return {
                group: 'nodes' as const,
                data: {
                    id: node.id,
                    label: node.name,
                    kind: node.kind,
                    groupId: node.groupId,
                    referenceScope: node.referenceScope ?? '',
                    backgroundColor: colour.secondary,
                    borderColor: colour.primary,
                    raw: node,
                },
            };
        }),
        ...graph.edges.map((edge) => ({
            group: 'edges' as const,
            data: {
                id: edge.id,
                source: edge.source,
                target: edge.target,
                groupId: edge.groupId,
                referenceKind: edge.referenceKind,
                targetColour: colours.get(edge.target)?.primary ?? ARCHITECTURE_COLOUR.primary,
                raw: edge,
            },
        })),
    ];
}

function asNumber(value: unknown, fallback: number): number {
    return typeof value === 'number' && Number.isFinite(value) && value > 0 ? value : fallback;
}

function settingsFor(configuration: ProjectConfiguration, layout: LayoutName): ViewLayoutSettings {
    const defaults = DEFAULT_LAYOUT_SETTINGS[layout];
    const raw = configuration.view_settings[layout];
    if (!raw || typeof raw !== 'object' || Array.isArray(raw)) return defaults;

    const values = raw as Record<string, unknown>;
    return {
        referenceDistance: asNumber(values.reference_distance, defaults.referenceDistance),
        subreferenceDistance: asNumber(values.subreference_distance, defaults.subreferenceDistance),
        nodeSpacing: asNumber(values.node_spacing, defaults.nodeSpacing),
        subreferenceAttraction: asNumber(values.subreference_attraction, defaults.subreferenceAttraction),
    };
}

function packGroups(cy: Core, graph: ArchitectureGraph) {
    if (graph.groups.length <= 1) return;

    let cursorX = 0;
    for (const group of graph.groups) {
        const groupNodes = cy.nodes().filter((node) => node.data('groupId') === group.id);
        if (groupNodes.empty()) continue;
        const box = groupNodes.boundingBox({ includeLabels: true });
        const dx = cursorX - box.x1;
        const dy = -box.y1;
        groupNodes.positions((node) => ({
            x: node.position('x') + dx,
            y: node.position('y') + dy,
        }));
        cursorX += box.w + REGION_GAP;
    }
}

function runLayout(
    cy: Core,
    layout: LayoutName,
    configuration: ProjectConfiguration,
    graph: ArchitectureGraph,
    incremental: boolean,
    onSettled: () => void,
) {
    if (cy.nodes().empty()) {
        onSettled();
        return;
    }

    const settings = settingsFor(configuration, layout);
    const options = layout === 'directed'
        ? {
            name: 'dagre',
            rankDir: 'LR',
            rankSep: Math.max(90, settings.referenceDistance * 0.62),
            nodeSep: settings.nodeSpacing,
            edgeSep: 22,
            padding: 48,
            minLen: (edge: EdgeSingular) => edge.data('referenceKind') === 'subreference' ? 1 : 2,
            edgeWeight: (edge: EdgeSingular) => edge.data('referenceKind') === 'subreference' ? 4 : 1,
        }
        : {
            name: 'fcose',
            quality: incremental ? 'proof' : 'default',
            randomize: !incremental,
            animate: incremental,
            animationDuration: incremental ? 350 : 0,
            fit: false,
            padding: 48,
            nodeRepulsion: 6200,
            idealEdgeLength: (edge: EdgeSingular) => edge.data('referenceKind') === 'subreference'
                ? settings.subreferenceDistance
                : settings.referenceDistance,
            edgeElasticity: (edge: EdgeSingular) => edge.data('referenceKind') === 'subreference'
                ? 0.45 / settings.subreferenceAttraction
                : 0.45,
            nestingFactor: 0.1,
            gravity: 0.22,
            numIter: incremental ? 1000 : 2800,
            initialEnergyOnIncremental: 0.2,
        };

    const layoutRunner = cy.layout(options as cytoscape.LayoutOptions);
    layoutRunner.one('layoutstop', () => {
        packGroups(cy, graph);
        onSettled();
    });
    layoutRunner.run();
}

function edgeNames(graph: ArchitectureGraph, edge: ArchitectureEdge) {
    const source = graph.nodes.find((node) => node.id === edge.source);
    const target = graph.nodes.find((node) => node.id === edge.target);
    return {
        sourceName: source?.name ?? edge.source,
        targetName: target?.name ?? edge.targetName,
    };
}

export function GraphCanvas({
    graph,
    configuration,
    filters,
    search,
    selection,
    layout,
    onLayoutChange,
    onSelectionChange,
    onOpenSource,
}: GraphCanvasProps) {
    const containerRef = useRef<HTMLDivElement | null>(null);
    const cyRef = useRef<Core | null>(null);
    const spaceDownRef = useRef(false);
    const panRef = useRef<{ pointerId: number; x: number; y: number } | null>(null);
    const layoutRef = useRef<LayoutName>(layout);
    const [hoveredEdge, setHoveredEdge] = useState<HoveredEdge | null>(null);
    const [contextMenu, setContextMenu] = useState<GraphContextMenu | null>(null);
    const [showEdgeDetails, setShowEdgeDetails] = useState(true);
    const [regions, setRegions] = useState<RenderedRegion[]>([]);
    const elements = useMemo(() => elementsFor(graph, configuration), [graph, configuration]);

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
        layoutRef.current = layout;

        const updateRegions = () => {
            const zoom = cy.zoom();
            const pan = cy.pan();
            const next = graph.groups.flatMap<RenderedRegion>((group) => {
                const groupNodes = cy.nodes().filter((node) => (
                    node.data('groupId') === group.id && node.visible()
                ));
                if (groupNodes.empty()) return [];
                const box = groupNodes.boundingBox({ includeLabels: true });
                return [{
                    id: group.id,
                    name: group.name,
                    layerIds: group.layerIds,
                    left: box.x1 * zoom + pan.x - REGION_PADDING,
                    top: box.y1 * zoom + pan.y - REGION_PADDING - 18,
                    width: box.w * zoom + REGION_PADDING * 2,
                    height: box.h * zoom + REGION_PADDING * 2 + 18,
                }];
            });
            setRegions(next);
        };

        const settle = (fit: boolean) => {
            if (fit && !cy.nodes().empty()) cy.fit(undefined, 64);
            updateRegions();
        };

        runLayout(cy, layout, configuration, graph, false, () => settle(true));

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
            const declaration = raw.declarations[0];
            if (!declaration) return;
            const point = event.renderedPosition ?? node.renderedPosition();
            setContextMenu({ x: point.x, y: point.y, source: declaration.source, label: raw.name });
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
            runLayout(cy, 'sticky', configuration, graph, true, () => settle(false));
        };
        const fitGraph = () => {
            if (cy.nodes().empty()) return;
            cy.fit(undefined, 64);
            updateRegions();
        };

        cy.on('tap', 'node', onNodeTap);
        cy.on('tap', 'edge', onEdgeTap);
        cy.on('tap', onBackgroundTap);
        cy.on('mouseover mousemove', 'edge', updateEdgeHover);
        cy.on('mouseout', 'edge', clearHover);
        cy.on('cxttap', 'node', openNodeContext);
        cy.on('cxttap', 'edge', openEdgeContext);
        cy.on('dragfree', 'node', settleStickyLayout);
        cy.on('pan zoom position resize', updateRegions);
        cy.on('pan zoom', clearHover);

        const keyDown = (event: KeyboardEvent) => {
            const tagName = (event.target as HTMLElement | null)?.tagName;
            if (['INPUT', 'TEXTAREA'].includes(tagName ?? '')) return;

            if (event.code === 'Escape') setContextMenu(null);

            if (event.code === 'KeyF' && !event.repeat) {
                fitGraph();
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

        (container as HTMLDivElement & { __archgraphFit?: () => void }).__archgraphFit = fitGraph;
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
            delete (container as HTMLDivElement & { __archgraphFit?: () => void }).__archgraphFit;
            cy.destroy();
            cyRef.current = null;
            setRegions([]);
        };
    }, [elements, graph, configuration]);

    useEffect(() => {
        if (layoutRef.current === layout) return;
        layoutRef.current = layout;
        const cy = cyRef.current;
        if (!cy) return;
        runLayout(cy, layout, configuration, graph, false, () => {
            if (!cy.nodes().empty()) cy.fit(undefined, 64);
        });
    }, [layout, configuration, graph]);

    useEffect(() => {
        const cy = cyRef.current;
        if (!cy) return;
        cy.nodes().forEach((node) => {
            const raw = node.data('raw') as ArchitectureNode;
            node.toggleClass('hidden-by-filter', !filters[filterKindForNode(raw)]);
        });
        cy.edges().forEach((edge) => {
            edge.toggleClass(
                'hidden-by-filter',
                edge.source().hasClass('hidden-by-filter') || edge.target().hasClass('hidden-by-filter'),
            );
        });
        cy.emit('resize');
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

    const fitGraph = () => {
        const container = containerRef.current as (HTMLDivElement & { __archgraphFit?: () => void }) | null;
        container?.__archgraphFit?.();
    };

    const selectedEdge = selection?.kind === 'edge' ? selection.item : null;
    const detailEdge = hoveredEdge?.edge ?? selectedEdge;
    const detailNames = detailEdge ? edgeNames(graph, detailEdge) : null;

    return (
        <div className="graph-panel">
            <div className="graph-region-overlay" aria-hidden="true">
                {regions.map((region) => (
                    <div
                        className="graph-region"
                        key={region.id}
                        style={{
                            left: region.left,
                            top: region.top,
                            width: region.width,
                            height: region.height,
                        }}
                    >
                        <span className="graph-region-label">{region.name}</span>
                        {region.layerIds.length > 1 ? (
                            <span className="graph-region-meta">{region.layerIds.length} merged layers</span>
                        ) : null}
                    </div>
                ))}
            </div>
            <div className="graph-toolbar">
                <button onClick={fitGraph}>Fit</button>
                <button
                    className={showEdgeDetails ? 'active' : ''}
                    onClick={() => setShowEdgeDetails((current) => !current)}
                    title="Show reference descriptions at the bottom of the graph"
                >
                    Descriptions
                </button>
                <select value={layout} onChange={(event) => onLayoutChange(event.target.value as LayoutName)} aria-label="Graph layout">
                    <option value="directed">Directed</option>
                    <option value="organic">Organic</option>
                    <option value="sticky">Sticky</option>
                </select>
            </div>
            <div className="graph-canvas" ref={containerRef} />
            {hoveredEdge ? (
                <div className="edge-tooltip" style={{ left: hoveredEdge.x + 14, top: hoveredEdge.y + 14 }}>
                    <strong>{hoveredEdge.sourceName} → {hoveredEdge.targetName}</strong>
                    <p>{hoveredEdge.edge.description || 'No reference description.'}</p>
                </div>
            ) : null}
            {showEdgeDetails && detailEdge && detailNames ? (
                <div className="edge-description-panel">
                    <strong>{detailNames.sourceName} → {detailNames.targetName}</strong>
                    <p>{detailEdge.description || 'No reference description.'}</p>
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

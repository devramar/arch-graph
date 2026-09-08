import type { ProjectScan } from '../types';

const project = {
    name: 'demo-events',
    root: '/demo/events',
};

const graph = {
    version: 3,
    project,
    groups: [
        { id: 'all', name: 'Architecture', layerIds: ['architecture'] },
    ],
    nodes: [
        {
            id: 'group:all:architecture:Events',
            name: 'Events',
            kind: 'architecture' as const,
            groupId: 'all',
            declarations: [
                {
                    layerId: 'architecture',
                    layerName: 'Architecture',
                    source: { file: 'src/features/events/ARCHITECTURE.md', line: 3 },
                    documentation: '# Events\n\nARCH_NODE:Events\n\nCoordinates the event feature and its major subsystems.\n',
                    sourceFormat: 'markdown' as const,
                },
            ],
        },
        {
            id: 'group:all:architecture:EventSync',
            name: 'EventSync',
            kind: 'architecture' as const,
            groupId: 'all',
            declarations: [
                {
                    layerId: 'architecture',
                    layerName: 'Architecture',
                    source: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 3 },
                    documentation: '# Event Synchronization\n\nARCH_NODE:EventSync\n\nARCH_REFERENCE:DateKey\n\nUsed as the canonical day representation.\n',
                    sourceFormat: 'markdown' as const,
                },
            ],
        },
        {
            id: 'group:all:reference:DateKey',
            name: 'DateKey',
            kind: 'reference' as const,
            groupId: 'all',
            referenceScope: 'shared' as const,
            declarations: [],
        },
        {
            id: 'group:all:subref:architecture:passwords',
            name: 'Password Management',
            kind: 'reference' as const,
            groupId: 'all',
            referenceScope: 'local' as const,
            declarations: [],
        },
    ],
    edges: [
        {
            id: 'group:all:architecture:events-sync',
            source: 'group:all:architecture:Events',
            target: 'group:all:architecture:EventSync',
            targetName: 'EventSync',
            groupId: 'all',
            layerId: 'architecture',
            description: 'Delegates remote reconciliation to the synchronization subsystem.',
            sourceLocation: { file: 'src/features/events/ARCHITECTURE.md', line: 12 },
            referenceKind: 'reference' as const,
        },
        {
            id: 'group:all:architecture:sync-datekey',
            source: 'group:all:architecture:EventSync',
            target: 'group:all:reference:DateKey',
            targetName: 'DateKey',
            groupId: 'all',
            layerId: 'architecture',
            description: 'Used as the canonical day representation when constructing synchronization windows.',
            sourceLocation: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 8 },
            referenceKind: 'reference' as const,
        },
        {
            id: 'group:all:architecture:sync-passwords',
            source: 'group:all:architecture:EventSync',
            target: 'group:all:subref:architecture:passwords',
            targetName: 'Password Management',
            groupId: 'all',
            layerId: 'architecture',
            description: 'Keeps password management scoped to EventSync.',
            sourceLocation: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 12 },
            referenceKind: 'subreference' as const,
        },
    ],
    diagnostics: [],
};

export const demoScan: ProjectScan = {
    configuration: {
        ignored_paths: [],
        layers: {
            architecture: {
                display_name: 'Architecture',
                files: ['ARCHITECTURE.md'],
                markers: {
                    ARCH_NODE: ['ARCH_NODE'],
                    ARCH_REFERENCE: ['ARCH_REFERENCE'],
                    ARCH_SUBREFERENCE: ['ARCH_SUBREFERENCE'],
                },
            },
        },
        app_colours: {
            colour_overrides: {
                references: {},
                subreferences: {
                    'Password Management': 'purple',
                },
            },
        },
        default_view: 'sticky',
        view_settings: {},
    },
    configurationExists: false,
    diagnostics: [],
    layers: [
        {
            id: 'architecture',
            displayName: 'Architecture',
            graph,
        },
    ],
    graph,
};

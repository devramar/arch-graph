import type { ProjectScan } from '../types';

export const demoScan: ProjectScan = {
    configuration: {
        aliasing: {
            'ARCHITECTURE.md': ['ARCHITECTURE.md'],
            ARCH_NODE: ['ARCH_NODE'],
            ARCH_REFERENCE: ['ARCH_REFERENCE'],
            ARCH_SUBREFERENCE: ['ARCH_SUBREFERENCE'],
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
        view_settings: {
            sticky: {
                reference_distance: 175,
                subreference_distance: 68,
                subreference_attraction: 1.8,
            },
        },
    },
    graph: {
        version: 2,
        project: {
            name: 'demo-events',
            root: '/demo/events',
        },
        nodes: [
            {
                id: 'arch:events#Events',
                name: 'Events',
                kind: 'architecture',
                source: { file: 'src/features/events/ARCHITECTURE.md', line: 3 },
                documentation: '# Events\n\nARCH_NODE:Events\n\n## Description\n\nCoordinates the event feature and its major subsystems.\n',
            },
            {
                id: 'arch:sync#EventSync',
                name: 'EventSync',
                kind: 'architecture',
                source: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 3 },
                documentation: '# Event Synchronization\n\nARCH_NODE:EventSync\n\n## References\n\nARCH_REFERENCE:DateKey\n\nUsed as the canonical day representation when constructing synchronization windows.\n',
            },
            {
                id: 'reference:DateKey',
                name: 'DateKey',
                kind: 'reference',
                referenceScope: 'shared',
            },
            {
                id: 'reference:EventStore',
                name: 'EventStore',
                kind: 'reference',
                referenceScope: 'shared',
            },
            {
                id: 'subref:arch:sync#EventSync:2#Password Management',
                name: 'Password Management',
                kind: 'reference',
                referenceScope: 'local',
            },
        ],
        edges: [
            {
                id: 'edge:events:sync',
                source: 'arch:events#Events',
                target: 'arch:sync#EventSync',
                targetName: 'EventSync',
                description: 'Delegates remote reconciliation to the synchronization subsystem.',
                sourceLocation: { file: 'src/features/events/ARCHITECTURE.md', line: 31 },
                referenceKind: 'reference',
            },
            {
                id: 'edge:sync:datekey',
                source: 'arch:sync#EventSync',
                target: 'reference:DateKey',
                targetName: 'DateKey',
                description: 'Used as the canonical day representation when constructing synchronization windows.',
                sourceLocation: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 24 },
                referenceKind: 'reference',
            },
            {
                id: 'edge:sync:store',
                source: 'arch:sync#EventSync',
                target: 'reference:EventStore',
                targetName: 'EventStore',
                description: 'Provides the local event state against which synchronization results are reconciled.',
                sourceLocation: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 28 },
                referenceKind: 'reference',
            },
            {
                id: 'edge:sync:passwords',
                source: 'arch:sync#EventSync',
                target: 'subref:arch:sync#EventSync:2#Password Management',
                targetName: 'Password Management',
                description: 'Keeps password management scoped to EventSync rather than merging it across the graph.',
                sourceLocation: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 32 },
                referenceKind: 'subreference',
            },
        ],
        diagnostics: [],
    },
};

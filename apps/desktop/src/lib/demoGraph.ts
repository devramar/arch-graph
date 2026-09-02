import type { ArchitectureGraph } from '../types';

export const demoGraph: ArchitectureGraph = {
    version: 1,
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
        },
        {
            id: 'arch:sync#EventSync',
            name: 'EventSync',
            kind: 'architecture',
            source: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 3 },
        },
        {
            id: 'module:DateKey#DateKey',
            name: 'DateKey',
            kind: 'module',
            source: { file: 'src/core/date/DateKey.ts', line: 1 },
        },
        {
            id: 'module:EventStore#EventStore',
            name: 'EventStore',
            kind: 'module',
            source: { file: 'src/features/events/storage/EventStore.ts', line: 8 },
        },
        {
            id: 'unresolved:RemoteChanges',
            name: 'RemoteChanges',
            kind: 'unresolved',
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
            resolution: 'architecture',
        },
        {
            id: 'edge:sync:datekey',
            source: 'arch:sync#EventSync',
            target: 'module:DateKey#DateKey',
            targetName: 'DateKey',
            description: 'Used as the canonical day representation when constructing synchronization windows.',
            sourceLocation: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 24 },
            resolution: 'module',
        },
        {
            id: 'edge:sync:store',
            source: 'arch:sync#EventSync',
            target: 'module:EventStore#EventStore',
            targetName: 'EventStore',
            description: 'Provides the local event state against which synchronization results are reconciled.',
            sourceLocation: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 28 },
            resolution: 'module',
        },
        {
            id: 'edge:sync:remote',
            source: 'arch:sync#EventSync',
            target: 'unresolved:RemoteChanges',
            targetName: 'RemoteChanges',
            description: 'Illustrates a dependency the scanner could not resolve.',
            sourceLocation: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 32 },
            resolution: 'unresolved',
        },
    ],
    diagnostics: [
        {
            code: 'ARCH003',
            severity: 'warning',
            message: 'Could not resolve dependency: RemoteChanges',
            source: { file: 'src/features/events/sync/ARCHITECTURE.md', line: 32 },
            candidates: [],
        },
    ],
};

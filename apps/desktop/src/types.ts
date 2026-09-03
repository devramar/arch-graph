export type NodeKind = 'architecture' | 'module' | 'external' | 'unresolved';
export type EdgeResolution = 'architecture' | 'module' | 'unresolved' | 'ambiguous';
export type DiagnosticSeverity = 'warning' | 'error';
export type DiagnosticCode = 'ARCH001' | 'ARCH002' | 'ARCH003' | 'ARCH004' | 'ARCH005';

export interface SourceLocation {
    file: string;
    line?: number | null;
}

export interface ArchitectureNode {
    id: string;
    name: string;
    kind: NodeKind;
    source?: SourceLocation | null;
    summary?: string | null;
    documentation?: string | null;
}

export interface ArchitectureEdge {
    id: string;
    source: string;
    target: string;
    targetName: string;
    description?: string | null;
    sourceLocation: SourceLocation;
    resolution: EdgeResolution;
}

export interface Diagnostic {
    code: DiagnosticCode;
    severity: DiagnosticSeverity;
    message: string;
    source?: SourceLocation | null;
    candidates?: string[];
}

export interface ArchitectureGraph {
    version: number;
    project: {
        name: string;
        root: string;
    };
    nodes: ArchitectureNode[];
    edges: ArchitectureEdge[];
    diagnostics: Diagnostic[];
}

export type GraphSelection =
    | { kind: 'node'; item: ArchitectureNode }
    | { kind: 'edge'; item: ArchitectureEdge }
    | null;

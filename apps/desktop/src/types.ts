export type NodeKind = 'architecture' | 'reference';
export type ReferenceScope = 'shared' | 'local';
export type ReferenceKind = 'reference' | 'subreference';
export type DiagnosticSeverity = 'warning' | 'error';
export type DiagnosticCode = 'ARCH001' | 'ARCH002' | 'ARCH004' | 'ARCH005';

export interface SourceLocation {
    file: string;
    line?: number | null;
}

export interface ArchitectureNode {
    id: string;
    name: string;
    kind: NodeKind;
    referenceScope?: ReferenceScope | null;
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
    referenceKind: ReferenceKind;
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

export interface ProjectConfiguration {
    aliasing: {
        'ARCHITECTURE.md': string[];
        ARCH_NODE: string[];
        ARCH_REFERENCE: string[];
        ARCH_SUBREFERENCE: string[];
    };
    app_colours: {
        colour_overrides: {
            references: Record<string, string>;
            subreferences: Record<string, string>;
        };
    };
    default_view?: string | null;
    view_settings: Record<string, unknown>;
}

export interface ProjectScan {
    graph: ArchitectureGraph;
    configuration: ProjectConfiguration;
}

export type GraphSelection =
    | { kind: 'node'; item: ArchitectureNode }
    | { kind: 'edge'; item: ArchitectureEdge }
    | null;

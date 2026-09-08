export type NodeKind = 'architecture' | 'reference';
export type ReferenceScope = 'shared' | 'local';
export type ReferenceKind = 'reference' | 'subreference';
export type SourceFormat = 'markdown' | 'decoratedText';
export type DiagnosticSeverity = 'warning' | 'error';
export type DiagnosticCode = 'ARCH001' | 'ARCH002' | 'ARCH004' | 'ARCH005' | 'ARCH006' | 'ARCH007';

export interface SourceLocation {
    file: string;
    line?: number | null;
}

export interface ArchitectureDeclaration {
    layerId: string;
    layerName: string;
    source: SourceLocation;
    documentation?: string | null;
    sourceFormat: SourceFormat;
}

export interface ArchitectureNode {
    id: string;
    name: string;
    kind: NodeKind;
    groupId: string;
    referenceScope?: ReferenceScope | null;
    declarations: ArchitectureDeclaration[];
}

export interface ArchitectureEdge {
    id: string;
    source: string;
    target: string;
    targetName: string;
    groupId: string;
    layerId: string;
    description?: string | null;
    sourceLocation: SourceLocation;
    referenceKind: ReferenceKind;
}

export interface Diagnostic {
    code: DiagnosticCode;
    severity: DiagnosticSeverity;
    message: string;
    source?: SourceLocation | null;
    layerId?: string | null;
    groupId?: string | null;
    candidates?: string[];
}

export interface ProjectInfo {
    name: string;
    root: string;
}

export interface GraphGroup {
    id: string;
    name: string;
    layerIds: string[];
}

export interface ArchitectureGraph {
    version: number;
    project: ProjectInfo;
    groups: GraphGroup[];
    nodes: ArchitectureNode[];
    edges: ArchitectureEdge[];
    diagnostics: Diagnostic[];
}

export interface ArchitectureLayer {
    id: string;
    displayName: string;
    graph: ArchitectureGraph;
}

export interface LayerCompositionGroup {
    id: string;
    name: string;
    layerIds: string[];
}

export interface DesktopLayerGroup extends LayerCompositionGroup {
    enabled: boolean;
}

export interface ProjectConfiguration {
    ignored_paths: string[];
    layers: Record<string, {
        display_name: string;
        path_root: string;
        ignored_paths: string[];
        files: string[];
        markers: {
            ARCH_NODE: string[];
            ARCH_REFERENCE: string[];
            ARCH_SUBREFERENCE: string[];
        };
    }>;
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
    layers: ArchitectureLayer[];
    diagnostics: Diagnostic[];
    configuration: ProjectConfiguration;
    configurationExists: boolean;
}

export type GraphSelection =
    | { kind: 'node'; item: ArchitectureNode }
    | { kind: 'edge'; item: ArchitectureEdge }
    | null;

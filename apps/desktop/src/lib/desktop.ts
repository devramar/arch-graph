import { invoke, isTauri } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import type { ProjectConfiguration, ProjectScan, SourceLocation } from '../types';

export function desktopRuntimeAvailable(): boolean {
    return isTauri();
}

export async function scanProject(root: string): Promise<ProjectScan> {
    return invoke<ProjectScan>('scan_project', { root });
}

export async function updateProjectConfiguration(
    root: string,
    configuration: ProjectConfiguration,
): Promise<ProjectConfiguration> {
    return invoke<ProjectConfiguration>('update_project_configuration', { root, configuration });
}

export async function openProjectSource(root: string, source: SourceLocation): Promise<void> {
    await invoke('open_project_source', { root, file: source.file });
}

export async function chooseProjectFolder(): Promise<string | null> {
    const selected = await open({
        directory: true,
        multiple: false,
        title: 'Open project root',
    });

    return typeof selected === 'string' ? selected : null;
}

export async function listenForProjectDrop(
    onDrop: (paths: string[]) => void,
    onHoverChanged: (hovering: boolean) => void,
): Promise<() => void> {
    return getCurrentWebview().onDragDropEvent((event) => {
        if (event.payload.type === 'over') {
            onHoverChanged(true);
            return;
        }

        if (event.payload.type === 'drop') {
            onHoverChanged(false);
            onDrop(event.payload.paths);
            return;
        }

        onHoverChanged(false);
    });
}

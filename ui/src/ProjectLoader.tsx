import { invoke } from '@tauri-apps/api/core';
import { createResource, createSignal, Show } from 'solid-js';
import { ProgressBar } from './ProgressBar';
import { Project } from './project/Project';
import init, { Project as WasmProject } from '@wasm';

export type SkylessProject = {
  assetsPath: string;
  mapPath: string;
};

export type TfsProject = {
  sprPath: string;
  datPath: string;
  otbPath: string;
  otbmPath: string;
  housesPath: string;
  spawnsPath: string;
};

type Progress = {
  progress: number;
  label?: string;
};

export const ProjectLoader = ({
  projectToLoad,
}: {
  projectToLoad: SkylessProject | TfsProject;
}) => {
  const [progress, setProgress] = createSignal<Progress>({
    progress: 0,
  });

  const [project] = createResource(async () => {
    const wsUrl = await invoke<string>('get_websocket_url');
    await init();

    const [, project] = await Promise.all([
      invoke<TfsProject>('load', { project: projectToLoad }),
      WasmProject.load(wsUrl, setProgress),
    ]);
    return project;
  });

  return (
    <Show
      when={project()}
      fallback={
        <div class="flex flex-col h-screen justify-center bg-neutral-800">
          <ProgressBar
            progress={progress().progress}
            label={progress().label || 'Loading'}
          />
        </div>
      }
    >
      <Project project={project()!} />
    </Show>
  );
};

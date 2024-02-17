import { createSignal, Show } from 'solid-js';
import { FilePicker } from './FilePicker';
import { ProjectLoader, TfsProject } from './ProjectLoader';
import { join } from 'path-browserify';

export const App = () => {
  const [projectToLoad, setProjectToLoad] = createSignal<TfsProject>();

  return (
    <Show
      when={projectToLoad()}
      fallback={
        <FilePicker
          onPick={async (path: string) => {
            const tfsProject = {
              sprPath: join(path, 'Tibia.spr'),
              datPath: join(path, 'Tibia.dat'),
              otbPath: join(path, 'items.otb'),
              otbmPath: join(path, 'map.otbm'),
              housesPath: join(path, 'map-house.xml'),
              spawnsPath: join(path, 'map-spawn.xml'),
            };
            setProjectToLoad(tfsProject);
          }}
          directory={true}
        >
          <input type="button" value="Import TFS project" />
        </FilePicker>
      }
    >
      <ProjectLoader projectToLoad={projectToLoad()!} />
    </Show>
  );
};

import { Component, createContext, For, onMount, useContext } from 'solid-js';
import { Project as ProjectModel } from '@wasm';
import { Map } from './map';
import { Position } from '../model';

const ProjectContext = createContext<ProjectModel>();

export const useProject = () => useContext(ProjectContext);

type ProjectProps = {
  project: ProjectModel;
};

export const Project: Component<ProjectProps> = (props) => {
  return (
    <ProjectContext.Provider value={props.project}>
      <div class="flex h-screen">
        <div class="flex-1">
          {/* FIXME:  hardcoded position */}
          <Map center={new Position(32369, 32241, 7)} interactive={true} />
        </div>
      </div>
    </ProjectContext.Provider>
  );
};

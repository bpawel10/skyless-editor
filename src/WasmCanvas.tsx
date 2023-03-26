import { Component, onMount } from 'solid-js';
import init, { render } from '../src-wasm/pkg/skyless_editor_wasm';

interface WasmCanvasProps {
  id: string;
}

export const WasmCanvas: Component<WasmCanvasProps> = (props) => {
  onMount(async () => {
    await init();
    render(props.id);
  });

  return (
    <>
      <p>Canvas</p>
      <canvas id={props.id} />
      <p>Canvas end</p>
    </>
  );
};

import { Accessor, createEffect, createSignal, on } from 'solid-js';
import { WebGLMapRenderer } from '@wasm';
import { MapTransformation } from '.';
import { useProject } from '../../Project';

export type MapRenderer = {
  render: () => void;
};

export const createMapRenderer = (
  canvas: Accessor<HTMLCanvasElement | undefined>,
  { transformation }: MapTransformation,
): MapRenderer => {
  const project = useProject();

  const [renderer, setRenderer] = createSignal<WebGLMapRenderer>();

  createEffect(
    on(
      canvas,
      () => canvas() && setRenderer(new WebGLMapRenderer(project!, canvas()!)),
    ),
  );

  createEffect(
    on(transformation, () => {
      renderer()?.render(transformation().into());
    }),
  );

  const render = () => {
    renderer()?.render(transformation().into());
  };

  return { render };
};

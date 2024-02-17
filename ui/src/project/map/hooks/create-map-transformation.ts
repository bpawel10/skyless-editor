import { Accessor, createEffect, createSignal, on } from 'solid-js';
import { Position } from '../../../model';
import { Matrix3 } from '../../../utils';
import { TILE_SIZE } from '../constants';
import { getProjectionViewMatrix } from '../utils';

export type MapTransformation = {
  transformation: Accessor<Matrix3>;
  move: (x: number, y: number) => void;
  zoom: (zoom: number, moveX: number, moveY: number) => void;
};

const getInitialTransformation = (
  canvas: HTMLCanvasElement,
  position: Position,
  zoom: number,
): Matrix3 => {
  const matrix = getProjectionViewMatrix(canvas);

  const translationX =
    (canvas.width - TILE_SIZE * zoom) / 2 - position.x * TILE_SIZE * zoom;
  const translationY =
    (canvas.height - TILE_SIZE * zoom) / 2 - position.y * TILE_SIZE * zoom;

  matrix.translate(translationX, translationY);
  matrix.scale(zoom);

  return matrix;
};

export const createMapTransformation = (
  canvas: Accessor<HTMLCanvasElement | undefined>,
  center: Position,
  zoom = 1 / window.devicePixelRatio,
): MapTransformation => {
  const [transformation, setTransformation] = createSignal(Matrix3.identity());

  createEffect(
    on(
      canvas,
      () =>
        canvas() &&
        setTransformation(getInitialTransformation(canvas()!, center, zoom)),
    ),
  );

  const move = (x: number, y: number) => {
    const newTransformation = transformation();
    newTransformation.translate(x, y);
    setTransformation(newTransformation);
  };

  const zoomFn = (zoom: number, moveX: number, moveY: number) => {
    const newTransformation = getProjectionViewMatrix(canvas()!);
    newTransformation.scale(zoom);
    newTransformation.translate(moveX, moveY);
    setTransformation(newTransformation);
  };

  return {
    transformation,
    move,
    zoom: zoomFn,
  };
};

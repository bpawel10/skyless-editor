import { createSignal } from 'solid-js';
import { MapTransformation, MapRenderer } from '.';
import { Matrix3 } from '../../../utils';
import { MAX_ZOOM, MIN_ZOOM, ZOOM_SPEED } from '../constants';
import { ListenerType, MapInputHandler } from './map-input-handler';

const getClipSpacePosition = (
  canvas: HTMLCanvasElement,
  x: number,
  y: number,
): [number, number] => {
  const normalizedX = x / canvas.clientWidth;
  const normalizedY = y / canvas.clientHeight;
  const clipX = normalizedX * 2 - 1;
  const clipY = normalizedY * -2 + 1;
  return [clipX, clipY];
};

export const createMapInputHandler = (
  canvasRef: HTMLCanvasElement,
  _: MapRenderer,
  transformation: MapTransformation,
): void => {
  const [inputHandler] = createSignal<MapInputHandler>(
    new MapInputHandler(canvasRef),
  );

  inputHandler().addEventListener(ListenerType.MOVE, (event: Event) => {
    if (!(event instanceof CustomEvent)) {
      throw new Error('Got MOVE event which is not CustomEvent');
    }

    const [deltaX, deltaY] = event.detail;
    const zoom = transformation.transformation().zoom;
    transformation.move(deltaX / zoom, deltaY / zoom);
  });

  inputHandler().addEventListener(ListenerType.ZOOM, (event: Event) => {
    if (!(event instanceof CustomEvent)) {
      throw new Error('Got ZOOM event which is not CustomEvent');
    }

    const { delta } = event.detail;
    const zoom = transformation.transformation().zoom;

    const [clipX, clipY] = getClipSpacePosition(canvasRef, zoom, zoom);
    const inversed = new Matrix3([...transformation.transformation().into()]);
    inversed.inverse();
    const [preZoomX, preZoomY] = inversed.transformPoint(clipX, clipY);
    const newZoom = Math.max(
      MIN_ZOOM,
      Math.min(MAX_ZOOM, zoom * Math.pow(2, delta * -ZOOM_SPEED)),
    );

    const inversed2 = new Matrix3([...transformation.transformation().into()]);
    inversed2.inverse();
    const [postZoomX, postZoomY] = inversed2.transformPoint(clipX, clipY);

    transformation.zoom(newZoom, postZoomX - preZoomX, postZoomY - preZoomY);
  });
};

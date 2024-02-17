import { Component, createEffect, createSignal, on, onMount } from 'solid-js';
import { v1 } from 'uuid';
import { WebGLMapRenderer } from '@wasm';
import { Position } from '../../model';
import { Matrix3 } from '../../utils';
import { ListenerType, MapInputHandler } from './hooks/map-input-handler';
import { getProjectionViewMatrix } from './utils';
import { MAX_ZOOM, MIN_ZOOM, TILE_SIZE, ZOOM_SPEED } from './constants';
import { useProject } from '../Project';

interface IMapProps {
  center?: Position;
  interactive?: boolean;
  zoom?: number;
}

export const Map: Component<IMapProps> = (props) => {
  const canvasId = v1();
  let canvasRef: HTMLCanvasElement;
  const project = useProject();
  const [zoom, setZoom] = createSignal(props.zoom || 1);
  const [translation, setTranslation] = createSignal<[number, number]>([0, 0]);
  const [transformation, setTransformation] = createSignal<Matrix3>();
  const [renderer, setRenderer] = createSignal<WebGLMapRenderer>();
  const [inputHandler, setInputHandler] = createSignal<MapInputHandler>();

  createEffect(
    on(transformation, () => renderer()?.render(transformation()!.into())),
  );

  onMount(() => {
    canvasRef.width = canvasRef.clientWidth;
    canvasRef.height = canvasRef.clientHeight;

    const initialTranslation = props.center
      ? getInitialTranslation(canvasRef, props.center, zoom())
      : ([0, 0] as [number, number]);
    setTranslation(initialTranslation);

    const initialTransformation = getProjectionViewMatrix(canvasRef);
    initialTransformation.scale(zoom());
    initialTransformation.translate(...translation());

    setRenderer(new WebGLMapRenderer(project!, canvasRef));
    setTransformation(initialTransformation);

    const observer = new ResizeObserver(() => {
      canvasRef.width = canvasRef.clientWidth;
      canvasRef.height = canvasRef.clientHeight;

      renderer()?.render(transformation()!.into());
    });
    observer.observe(canvasRef);

    if (props.interactive) {
      setInputHandler(new MapInputHandler(canvasRef));

      inputHandler()!.addEventListener(ListenerType.MOVE, (event: Event) => {
        if (!(event instanceof CustomEvent)) {
          throw new Error('Got MOVE event which is not CustomEvent');
        }

        const [deltaX, deltaY] = event.detail;
        const newTransformation = transformation()!.clone();
        newTransformation.translate(deltaX / zoom(), deltaY / zoom());
        setTransformation(newTransformation);
      });

      inputHandler()!.addEventListener(ListenerType.ZOOM, (event: Event) => {
        if (!(event instanceof CustomEvent)) {
          throw new Error('Got ZOOM event which is not CustomEvent');
        }

        const { zoomX, zoomY, delta } = event.detail;
        const [clipX, clipY] = getClipSpacePosition(canvasRef, zoomX, zoomY);
        const inversed = new Matrix3([...transformation()!.into()]);
        inversed.inverse();
        const [preZoomX, preZoomY] = inversed.transformPoint(clipX, clipY);
        const newZoom = Math.max(
          MIN_ZOOM,
          Math.min(MAX_ZOOM, zoom() * Math.pow(2, delta * -ZOOM_SPEED)),
        );
        const newTransformation = getProjectionViewMatrix(canvasRef);
        newTransformation.scale(newZoom);

        const inversed2 = newTransformation.clone();
        inversed2.inverse();
        const [postZoomX, postZoomY] = inversed2.transformPoint(clipX, clipY);
        const x = transformation()!.x;
        const y = transformation()!.y;
        const newTranslation: [number, number] = [
          x + preZoomX - postZoomX,
          y + preZoomY - postZoomY,
        ];
        newTransformation.translate(postZoomX - preZoomX, postZoomY - preZoomY);

        setZoom(newZoom);
        setTranslation(newTranslation);
        setTransformation(newTransformation);
      });
    }

    renderer()!.render(transformation()!.into());
  });

  return (
    <div class="flex h-full">
      <canvas
        ref={canvasRef!}
        id={canvasId}
        class="flex-1 w-full h-full bg-black"
      ></canvas>
    </div>
  );
};

const getInitialTranslation = (
  canvas: HTMLCanvasElement,
  position: Position,
  zoom: number,
): [number, number] => {
  const translationX =
    ((canvas.width - TILE_SIZE) / 2 - position.x * TILE_SIZE) / zoom;
  const translationY =
    ((canvas.height - TILE_SIZE) / 2 - position.y * TILE_SIZE) / zoom;
  return [translationX, translationY];
};

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

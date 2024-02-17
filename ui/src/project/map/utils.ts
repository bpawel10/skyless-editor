import { Matrix3 } from '../../utils';

export const getProjectionViewMatrix = (
  canvasRef: HTMLCanvasElement,
): Matrix3 => {
  const projection = Matrix3.identity();
  projection.project(canvasRef.width, canvasRef.height);
  return projection;
};

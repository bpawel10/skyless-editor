export enum ListenerType {
  MOVE = 'move',
  ZOOM = 'zoom',
}

export class MapInputHandler implements EventTarget {
  private listeners: {
    move: EventListenerOrEventListenerObject[];
    zoom: EventListenerOrEventListenerObject[];
  } = {
    [ListenerType.MOVE]: [],
    [ListenerType.ZOOM]: [],
  };
  private lastMousePos: [number, number] = [0, 0];
  private mouseMoveHandler = this.handleMouseMove.bind(this);
  private mouseUpHandler = this.handleMouseUp.bind(this);
  private mouseMoveListener: any;

  constructor(private canvas: HTMLCanvasElement) {
    this.canvas.addEventListener('mousedown', this.handleMouseDown.bind(this));
    this.canvas.addEventListener('wheel', this.handleWheel.bind(this));
  }

  addEventListener(
    type: ListenerType,
    callback: EventListenerOrEventListenerObject,
    options?: boolean | AddEventListenerOptions,
  ): void {
    this.listeners[type].push(callback);
  }

  dispatchEvent(event: Event): boolean {
    this.listeners[event.type as ListenerType].forEach((listener) =>
      'handleEvent' in listener ? listener.handleEvent(event) : listener(event),
    );
    return true;
  }

  removeEventListener(
    type: string,
    callback: EventListenerOrEventListenerObject,
    options?: boolean | EventListenerOptions,
  ): void {
    // TODO:
  }

  handleMouseDown(event: MouseEvent): void {
    event.preventDefault();

    window.addEventListener('mousemove', this.mouseMoveHandler);
    window.addEventListener('mouseup', this.mouseUpHandler);

    const { clientX, clientY } = event;

    this.lastMousePos = [clientX, clientY];
  }

  handleMouseMove({ clientX, clientY }: MouseEvent): void {
    const [lastX, lastY] = this.lastMousePos;

    const deltaX = clientX - lastX;
    const deltaY = clientY - lastY;

    this.lastMousePos = [clientX, clientY];

    if (deltaX !== 0 || deltaY !== 0) {
      this.dispatchEvent(new CustomEvent('move', { detail: [deltaX, deltaY] }));
    }
  }

  handleMouseUp(): void {
    window.removeEventListener('mousemove', this.mouseMoveHandler);
    window.removeEventListener('mouseup', this.mouseUpHandler);
  }

  handleWheel(event: WheelEvent): void {
    event.preventDefault();

    const { offsetX: zoomX, offsetY: zoomY, deltaY: delta } = event;

    if (delta !== 0) {
      this.dispatchEvent(
        new CustomEvent('zoom', { detail: { zoomX, zoomY, delta } }),
      );
    }
  }
}

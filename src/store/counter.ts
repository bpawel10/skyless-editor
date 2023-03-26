import { createResource } from 'solid-js';
import init, { Counter } from '../../src-wasm/pkg/skyless_editor_wasm';

const [counter, { mutate: mutateCounter }] = createResource(async () => {
  await init();
  return new Counter();
});

const refreshCounter = () =>
  mutateCounter(
    Object.assign(Object.create(Object.getPrototypeOf(counter())), counter()),
  );

const refreshCounterDelayed = () => setTimeout(refreshCounter, 0);

export { counter, refreshCounter, refreshCounterDelayed };

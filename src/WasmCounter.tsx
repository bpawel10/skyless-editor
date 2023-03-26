import { Component, Show } from 'solid-js';
import { counter, refreshCounter } from './store/counter';

export const WasmCounter: Component = () => {
  return (
    <>
      <h2>Wasm counter</h2>
      <Show when={counter() !== undefined} fallback={<p>Loading...</p>}>
        <p>Value: {counter()?.get()}</p>
        <button
          onClick={() => {
            counter()?.increment();
            refreshCounter();
          }}
        >
          Increment
        </button>
        <button
          onClick={() => {
            counter()?.decrement();
            refreshCounter();
          }}
        >
          Decrement
        </button>
      </Show>
    </>
  );
};

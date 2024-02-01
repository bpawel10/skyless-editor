// import { invoke } from '@tauri-apps/api/tauri';
import { WasmCanvas } from './WasmCanvas';
import { WasmCounter } from './WasmCounter';

function App() {
  async function greet() {
    // await invoke('greet', { name: name() });
  }

  return (
    <>
      <h1>Wasm</h1>
      <WasmCounter />
      <WasmCanvas id="canvas" />
    </>
  );
}

export default App;

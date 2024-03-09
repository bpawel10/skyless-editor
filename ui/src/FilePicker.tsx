import { Component, createSignal, JSXElement, Show } from 'solid-js';
import { children as childrenHelper } from 'solid-js';
import { open } from '@tauri-apps/plugin-dialog';

interface FilePickerProps {
  children: JSXElement;
  onPick: Function;
  directory?: boolean;
}

export const FilePicker: Component<FilePickerProps> = (props) => {
  const c = childrenHelper(() => props.children);

  const [element, setElement] = createSignal<Element>();

  const pick = async () => {
    const path = await open({
      ...(props.directory && { directory: props.directory }),
    });
    setElement(props.onPick(path));
  };

  return (
    <Show when={element()} fallback={<div onClick={pick}>{c()}</div>}>
      {element()}
    </Show>
  );
};

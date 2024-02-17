export const ProgressBar = (props: { progress?: number; label?: string }) => (
  <div class="flex flex-col items-center">
    <div class="w-36 h-1.5 bg-neutral-500 rounded overflow-hidden">
      <div
        class="h-full bg-green-500"
        style={{
          width: `${(props.progress || 0) * 100}%`,
        }}
      />
    </div>
    {props.label && (
      <p class="mt-0.5 text-sm text-neutral-400">{props.label}</p>
    )}
  </div>
);

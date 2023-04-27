
import { Accessor, Component, createSignal, onMount } from 'solid-js';


const ImageComponent: Component<{ src: Accessor<string>; }> = (props) => {
  let [blackAndWhite, setBlackAndWhite] = createSignal(false);

  let [flip, setFlip] = createSignal(false);
  let [fullscreen, setFullscreen] = createSignal(false);

  return (<>
    <div>
      <div class="form-control flex flex-row justify-center">
        <label class="label cursor-pointer">
          <span class="label-text font-thin p-2">black and white</span>
          <input type="checkbox" class="toggle" checked={blackAndWhite()} onclick={() => {
            setBlackAndWhite(!blackAndWhite());
          }} />
        </label>
        <label class="label cursor-pointer">
          <span class="label-text font-thin p-2">flip</span>
          <input type="checkbox" class="toggle" checked={flip()} onclick={() => {
            setFlip(!flip());
          }} />       </label>
      </div>
      <div class="img-magnifier-container">
        <img onclick={() => {
          setFullscreen(!fullscreen())
        }} class={` rounded-md shadow-md object-contain object-scale-down max-h-[70vh]  ${blackAndWhite() ? " grayscale " : " "} ${flip() ? " scale-x-[-1] " : ""}`} src={props.src()} />
      </div>
    </div>
  </>
  );
};

export default ImageComponent;

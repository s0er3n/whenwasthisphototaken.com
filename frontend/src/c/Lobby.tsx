
import { Component, createSignal, Show, For, Accessor, onCleanup } from 'solid-js';
import { setLoggedIn } from '../App';

import Chart, { setData } from './Chart';
import Image from './Image';

let [image, setImage] = createSignal("");
export let [state, setState] = createSignal("image")
let [results, setResults] = createSignal([])
export let [result, setResult] = createSignal(0)
let [guess, setGuess] = createSignal();
let [timer, setTimer] = createSignal<number>()
let [round, setRound] = createSignal([1, 5])

const Lobby: Component = (props) => {

  let interval = setInterval(() => {
    if (timer()) {
      setTimer(timer() - 1)
    }
  }, 1000)



  let socket: any;

  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  let counter = 0;
  let initWS = () => {

    if (counter >= 5) {
      location.href = location.origin

      return
    }

    socket = new WebSocket(`${import.meta.env.VITE_BACKEND_URL}/ws`)

    socket.onopen = () => {
      console.log("connected")
      window.history.pushState({}, "", location.origin);
      window.history.pushState({}, "", "?l=t");
    }

    socket.onmessage = (e) => {
      // console.log(e.data)
      const data = JSON.parse(e.data)
      if (data.Image) {
        if (state() !== "image") {
          setTimer(40)
        }
        setState("image")
        console.log(data.Image)

        setImage(data.Image.url)
        setData(data.Image.guesses)
        setRound([data.Image.pos, data.Image.len])
      }
      if (data.AfterImage) {
        if (state() !== "afterImage") {
          setTimer(5)
        }
        setState("afterImage")
        setGuess(undefined);
        setImage(data.AfterImage.url)
        setData(data.AfterImage.guesses)
        setResults(data.AfterImage.scores)
        setResult(data.AfterImage.result)
      }
      if (data.Results) {
        if (state() !== "results") {
          setTimer(30)
        }
        setState("results")
        setResults(data.Results.scores)

      }
    }
    socket.onclose = () => {
      // window.location = window.location.origin
      counter = counter + 1;
      setTimeout(() => { initWS() }, 1000)
      if (counter === 1) {
        // resetting timeout bc 5 connection loesses are ok if its not happening in 1 minute :)
        setTimeout(() => {
          counter = 0
        }, 60000)
      }
    }
  }
  initWS()
  onCleanup(() => clearInterval(interval))
  return (
    <><div
      class="p-5 font-mono"
    >
      {/* <div>Timer: {timer()}</div> */}
      <Show when={state() !== "results"}>
        <div class="flex flex-col justify-center items-center">
          <a href="/"><h1 class="text-4xl font-thin font-mono">WhenWasThisPhotoTaken.com</h1></a>
          <span>write the year in chat to guess! write <code class='code'>!next to go to {state() !== "afterImage" ? `${true ? "round" : ""} results` : "the next image"}.</code></span>
          <span>{round()[0]}/{round()[1]}</span>
        </div>
        <div class='flex justify-evenly'>
          <Show when={state() === "afterImage"}>
            <div class="flex flex-col justify-center items-center">
              <Show when={state() === "afterImage"} >
                <span>Correct Result: <span class="font-bold">{result()}</span></span>
              </Show>
              <For each={results().sort((a, b) => b[2] - a[2])}>{(r, i) =>
                <li>
                  {i() + 1}: {r[0]}  {Math.floor(r[2])} Points ({Math.floor(r[1])})
                </li>
              }</For>
            </div>
          </Show>
          <Image src={image} />
        </div>
        <div><Chart /></div>
        <Show when={state() === "image"}>
          {/* <div class="pl-[1.1rem] pr-1 w-full"> */}
          {/*   <input class="w-full" min="1900" max="2023" onChange={(e: any) => { */}
          {/*     setGuess(e.target.value) */}
          {/**/}
          {/*     if (guess()) { */}
          {/*       socket?.send(`${props.user_name()};${guess()}`) */}
          {/*     } */}
          {/*   }} type="range" /> */}
          {/* </div> */}
          {/* <div>Your guess:{guess() ?? " make a guess"}</div > */}
        </Show>
      </Show >
      <Show when={state() === "results"}>
        <div class="flex flex-col justify-center items-center">
          <h1 class='text-4xl'>Results:</h1>
          <For each={results().sort((a, b) => b[1] - a[1])}>{(r, i) =>
            <li>
              {i() + 1}: {r[0]} {Math.floor(r[1])} Points
            </li>
          }</For>
        </div>
      </Show>
    </div ></>

  );
};

export default Lobby;

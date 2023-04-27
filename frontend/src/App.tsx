import { Component, createSignal, Show, For, Switch, Match, onMount } from 'solid-js';
import AddImage from './c/AddImage';
import Lobby from './c/Lobby';

export let [loggedIn, setLoggedIn] = createSignal(false);
const App: Component = () => {

  let [state, setState] = createSignal("normal")


  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  let code = urlParams.get("code") ?? ""
  let login = urlParams.get("l") ?? ""

  if (code) {
    fetch(`https://backend.whenwasthisphototaken.com/login?code=${code}`, { credentials: 'include' }).then(() => {
      setLoggedIn(true)
    })
  }
  if (login) {
    setLoggedIn(true)
  }

  return (
    <>
      <Switch >
        <Match when={state() === "normal"} >
          <Show when={loggedIn()} fallback={
            <div class="flex flex-col items-center justify-center h-screen">
              <a href={`https://id.twitch.tv/oauth2/authorize?client_id=x07c2xe4de156ip8sundhmisvkvacz&response_type=code&redirect_uri=${window.location.origin + "/"}&scope=`}>
                <button class='btn btn-primary'>Connect with Twitch</button></a>
            </div>

          }>
            <div class='flex justify-center pt-2'>
              <button class='btn btn-xs' onclick={() => {
                setState("add_image")
              }}
              > add your own image</button>
            </div>
            <Lobby />
          </Show>
          {/* <div class='flex w-full fixed bottom-0 justify-center'> */}
          {/*   <button onClick={() => { */}
          {/*     setState("add_image") */}
          {/*   }}>add image for everyone to play</button> */}
          {/* </div> */}
        </Match>
        <Match when={state() === "add_image"} >

          <div class='flex justify-center pt-2'>
            <button class='btn btn-xs' onclick={() => {
              setState("normal")
            }}
            > back to the game</button>
          </div>
          <AddImage />

        </Match>
      </Switch>

    </>

  );
};

export default App;

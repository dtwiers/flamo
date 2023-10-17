import { createSignal } from 'solid-js';
// import logo from "./assets/flamo-logo.svg";
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

function App() {
  const [greetMsg, setGreetMsg] = createSignal('');
  const [name, setName] = createSignal('');

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke('greet', { name: name() }));
  }

  return (
    <div class='container'>
      <h1>Welcome to Flamo!</h1>

      <div class='row'>
        <img src='/flamo-logo.svg' class='logo' alt='Flamo logo' />
      </div>

      <form
        class='row'
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id='greet-input'
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder='Enter a name...'
        />
        <button type='submit'>Greet</button>
      </form>

      <p>{greetMsg()}</p>
    </div>
  );
}

export default App;

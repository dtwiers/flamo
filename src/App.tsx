import { Show, createSignal } from 'solid-js';
// import logo from "./assets/flamo-logo.svg";
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

function App() {
  const [imgsrc, setImgsrc] = createSignal('');

  async function getImg() {
    setImgsrc('');
    setImgsrc(await invoke('make_image'));
  }

  return (
    <div class='container'>
      <h1>Welcome to Flamo!</h1>

      <div class='row'>
        <img src='/flamo-logo.svg' class='logo' alt='Flamo logo' />
      </div>

      <button onClick={getImg}>Get Image</button>

      <div class='row'>
        <Show when={imgsrc()}>
          <img src={imgsrc()} alt='Image' />
        </Show>
      </div>
    </div>
  );
}

export default App;

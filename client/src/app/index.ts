import { start_doodle } from 'jji421-myapp';

const port = 'ws://localhost:5000/api/doodle';

start_doodle(port);
export function onReceiveMessage(evt: MessageEvent) {
  let val = document.createElement('p');
  val.innerHTML = evt.data;
  document.getElementById('wasm-area').appendChild(val);
}

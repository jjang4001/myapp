import { start_doodle } from 'jji421-myapp';

const port = 'ws://localhost:5000/api/doodle';
// let width = window.innerWidth;
// let height = window.innerHeight;

start_doodle(port);
export function onReceiveMessage(evt: MessageEvent) {
  let val = document.createElement('p');
  val.innerHTML = evt.data;
  document.getElementById('wasm-area').appendChild(val);
}

// const canvas: HTMLCanvasElement = document.getElementById("myapp-canvas") as HTMLCanvasElement;
// const context = canvas.getContext('2d');

// window.addEventListener('resize', (evt: Event) => {
//   console.log('resize');
//   let tmp = context.getImageData(0, 0, width, height);
//   console.log(tmp);
//   let newWidth = window.innerWidth;
//   let newHeight = window.innerHeight;
//   width = newWidth;
//   height = newHeight;
//   canvas.width = newWidth;
//   canvas.height = newHeight;
//   canvas.getContext('2d').putImageData(tmp, newWidth, newHeight);
// });

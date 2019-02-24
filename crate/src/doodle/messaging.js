// export class Messenger {
//   ws = null;

//   constructor(wsAddress) {
//     console.log('made messenger');
//     this.ws = new WebSocket(wsAddress);
//     this.ws.onmessage = this.onReceiveMessage;
//     this.ws.onclose = this.onCloseConnection;
//   }
//   // constructor() {
//   //   console.log('made messenger');
//   // }

//   sendMessage(msg) {
//     if (!this.ws) return;
//     this.ws.send(msg);
//   }

//   onReceiveMessage(evt) {
//     let val = document.createElement('p');
//     val.innerHTML = evt.data;
//     document.getElementById('wasm-area').appendChild(val);
//   }

//   onCloseConnection() {
//     console.log('connection closed.');
//   }
// }

export function name() {
  return 'World';
}

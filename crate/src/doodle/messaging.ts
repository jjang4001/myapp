export class Messenger {
  ws: WebSocket = null;

  constructor(wsAddress: string) {
    this.ws = new WebSocket(wsAddress);
    this.ws.onmessage = this.onReceiveMessage;
    this.ws.onclose = this.onCloseConnection;
  }

  sendMessage(msg: string) {
    if (!this.ws) return;
    this.ws.send(msg);
  }

  onReceiveMessage(evt: MessageEvent) {
    let val = document.createElement('p');
    val.innerHTML = evt.data;
    document.getElementById(DOM_MESSAGING_WINDOW).appendChild(val);
  }

  onCloseConnection() {
    console.log('connection closed.');
  }
}

const DOM_MESSAGING_WINDOW = 'wasm-message-window';

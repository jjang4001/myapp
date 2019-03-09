export class Messenger {
  ws: WebSocket = null;
  context: CanvasRenderingContext2D = null;

  constructor(wsAddress: string, context: CanvasRenderingContext2D) {
    this.ws = new WebSocket(wsAddress);
    this.ws.onmessage = this.onReceiveMessage;
    this.ws.onclose = this.onCloseConnection;
    this.context = context;
  }

  sendMessage(msg: string) {
    if (!this.ws) return;
    this.ws.send(msg);
  }

  onReceiveMessage(evt: MessageEvent) {
    let val = document.createElement('p');
    val.innerHTML = evt.data;
    document.getElementById(DOM_MESSAGING_WINDOW).appendChild(val);
    let a = JSON.parse(evt.data);
    // console.log(a.msg);
    let b = JSON.parse(a.msg);
    console.log(b);
  }

  onCloseConnection() {
    console.log('connection closed.');
  }
}

const DOM_MESSAGING_WINDOW = 'wasm-message-window';

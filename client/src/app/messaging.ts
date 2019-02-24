interface Messaging {
  ws: WebSocket;
  sendMessage(msg: string): void;
  onReceiveMessage(evt: MessageEvent): void,
  onCloseConnection(): void;
}

export class Messenger implements Messaging {
  ws: WebSocket = null;

  constructor(wsAddress: string) {
    console.log('made messenger');
    this.ws = new WebSocket(wsAddress);
    this.ws.onmessage = this.onReceiveMessage;
    this.ws.onclose = this.onCloseConnection;
  }

  sendMessage(msg: string): void {
    if (!this.ws) return;
    this.ws.send(msg);
  }

  onReceiveMessage(evt: MessageEvent): void {
    let val = document.createElement('p');
    val.innerHTML = evt.data;
    document.getElementById('wasm-area').appendChild(val);
  }

  onCloseConnection(): void {
    console.log('connection closed.');
  }
}

package hub

// Hub defines the struct for managing connected clients
type Hub struct {
	clients    map[*Client]bool      // registered clients
	broadcast  chan BroadcastMessage // inbound messages from the clients
	register   chan *Client          // regsiter requests from the clients
	unregister chan *Client          // unregister requests from clients
}

// BroadcastMessage defines the message identified by the sender
type BroadcastMessage struct {
	message []byte
	client  *Client
}

// NewHub initializes a new hub
func NewHub() *Hub {
	return &Hub{
		broadcast:  make(chan BroadcastMessage),
		register:   make(chan *Client),
		unregister: make(chan *Client),
		clients:    make(map[*Client]bool),
	}
}

// Run is the process that runs for the duration of the server and controls adding, removing and sending messages to clients
func (h *Hub) Run() {
	for {
		select {
		case client := <-h.register:
			h.clients[client] = true
		case client := <-h.unregister:
			if _, ok := h.clients[client]; ok {
				delete(h.clients, client)
				close(client.send)
			}
		case broadcastMessage := <-h.broadcast:
			for client := range h.clients {
				if client == broadcastMessage.client {
					continue
				}

				select {
				case client.send <- broadcastMessage.message:
				default:
					close(client.send)
					delete(h.clients, client)
				}
			}
		}
	}
}

package handlers

import (
	"fmt"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	CheckOrigin:     func(r *http.Request) bool { return true },
}

// DoodleHandler is the endpoint for receiving doodle messages
func DoodleHandler(w http.ResponseWriter, r *http.Request) {
	socket, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer socket.Close()
	for {
		msgType, msg, err := socket.ReadMessage()
		if err != nil {
			fmt.Println(err)
			break
		}
		fmt.Println(string(msg))
		if err = socket.WriteMessage(msgType, msg); err != nil {
			fmt.Println(err)
			break
		}
	}
}

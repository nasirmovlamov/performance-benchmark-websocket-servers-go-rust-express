package main

import (
	"fmt"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow all connections (insecure for prod)
	},
}

func handler(w http.ResponseWriter, r *http.Request) {
	// Upgrade HTTP connection to WebSocket
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		fmt.Println("Upgrade error:", err)
		return
	}
	defer conn.Close()

	for {
		// Read message from client
		_, msg, err := conn.ReadMessage()
		if err != nil {
			fmt.Println("Read error:", err)
			break
		}
		fmt.Printf("Received: %s\n", msg)

		// Respond with "Hello World"
		err = conn.WriteMessage(websocket.TextMessage, []byte("Hello World"))
		if err != nil {
			fmt.Println("Write error:", err)
			break
		}
	}
}

func main() {
	http.HandleFunc("/ws", handler)
	fmt.Println("Server started on :8080")
	http.ListenAndServe(":8080", nil)
}


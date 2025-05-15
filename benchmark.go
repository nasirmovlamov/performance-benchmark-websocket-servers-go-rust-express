package main

import (
	"fmt"
	"log"
	"sync"
	"time"

	"github.com/gorilla/websocket"
)

const (
	numClients   = 100
	numMessages  = 10
	serverURL    = "ws://localhost:8080/ws"
)

func main() {
	var wg sync.WaitGroup
	start := time.Now()

	for i := 0; i < numClients; i++ {
		wg.Add(1)
		go func(clientID int) {
			defer wg.Done()

			conn, _, err := websocket.DefaultDialer.Dial(serverURL, nil)
			if err != nil {
				log.Printf("Client %d failed to connect: %v", clientID, err)
				return
			}
			defer conn.Close()

			for j := 0; j < numMessages; j++ {
				startTime := time.Now()
				err := conn.WriteMessage(websocket.TextMessage, []byte(fmt.Sprintf("Client %d: Msg %d", clientID, j)))
				if err != nil {
					log.Printf("Client %d write error: %v", clientID, err)
					return
				}

				_, msg, err := conn.ReadMessage()
				if err != nil {
					log.Printf("Client %d read error: %v", clientID, err)
					return
				}

				latency := time.Since(startTime)
				log.Printf("Client %d received: %s | latency: %v", clientID, msg, latency)
			}
		}(i)
	}

	wg.Wait()
	fmt.Printf("Benchmark finished in %v\n", time.Since(start))
}

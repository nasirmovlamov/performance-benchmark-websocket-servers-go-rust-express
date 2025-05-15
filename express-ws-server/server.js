const express = require('express');
const http = require('http');
const WebSocket = require('ws');

const app = express();
const port = 8080;

const server = http.createServer(app);
const wss = new WebSocket.Server({ server });

// Serve static files (optional, like HTML clients)
app.use(express.static('public'));

wss.on('connection', (ws) => {
  console.log('Client connected');

  ws.on('message', (message) => {
    console.log(`Received: ${message}`);
    ws.send('Hello World');
  });

  ws.on('close', () => {
    console.log('Client disconnected');
  });
});

server.listen(port, () => {
  console.log(`Server is listening on http://localhost:${port}`);
});

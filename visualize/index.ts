import { serve } from "bun";
import {} from "fs/promises";

const filePath = "output.log";

const indexHtml = Bun.file("./index.html");

const server = Bun.serve({
  async fetch(req, server) {
    const url = new URL(req.url);
    if (url.pathname === "/") {
      const response = await indexHtml.text();
      // Отдаём содержимое index.html с правильным Content-Type
      return new Response(response, {
        headers: { "Content-Type": "text/html; charset=utf-8" },
      });
    }
    // upgrade the request to a WebSocket
    if (url.pathname === "/chat") {
      server.upgrade(req);
      return; // do not return a Response
    }
    return new Response("Upgrade failed", { status: 500 });
  },
  websocket: {
    message(ws, message) {}, // a message is received
    async open(ws) {
      const outpost = Bun.file("output.log");
      const content = await outpost.text();
      ws.subscribe("chat");
      server.publish("chat", content);
    }, // a socket is opened
    close(ws, code, message) {}, // a socket is closed
    drain(ws) {}, // the socket is ready to receive more data
  },
});

console.log(`Listening on ${server.hostname}:${server.port}`);

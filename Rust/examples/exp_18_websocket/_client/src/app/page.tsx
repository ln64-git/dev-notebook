"use client";

import { useState, useEffect } from "react";

const HomePage = () => {
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [responseData, setResponseData] = useState<string | null>(null);
  const [connectionStatus, setConnectionStatus] = useState<string | null>(null);

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:8080");
    socket.onopen = () => {
      console.log("WebSocket connection established.");
      socket.send("Connected");
      setConnectionStatus("Connected");
      setWs(socket);
    };
    socket.onmessage = (event) => {
      console.log("Received message from server:", event.data);
      setResponseData(event.data);
    };
    socket.onclose = () => {
      setConnectionStatus("Closed");
      socket.send("Disonnected");
      console.log("WebSocket connection closed.");
    };
    return () => {
      if (socket) {
        socket.close();
      }
    };
  }, []);

  return (
    <div className="flex h-full mt-10 max-w-[580px] mx-auto text-slate-100">
      <div className="flex-1 px-4 text-zinc-400 mt-1.5 gap-2">
        <h1>{connectionStatus}</h1>
        {responseData && (
          <div className="mt-4">
            <p>Server Response:</p>
            <pre>{responseData}</pre>
          </div>
        )}
      </div>
      <div className=""></div>
    </div>
  );
};

export default HomePage;

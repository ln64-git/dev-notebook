import { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [responseData, setResponseData] = useState<string | null>(null);

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:8080");
    socket.onopen = () => {
      console.log("WebSocket connection established.");
      setWs(socket);
    };
    socket.onmessage = (event) => {
      console.log("Received message from server:", event.data);
      setResponseData(event.data);
    };
    socket.onclose = () => {
      console.log("WebSocket connection closed.");
    };
    return () => {
      if (socket) {
        socket.close();
      }
    };
  }, []);

  const fetchData = () => {
    if (ws) {
      ws.send("Fetch Data");
    }
  };

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>
      <button className="bg-indigo-950 rounded-md p-2" onClick={fetchData}>
        Fetch Data
      </button>
      {responseData && (
        <div className="mt-4">
          <p>Server Response:</p>
          <pre>{responseData}</pre>
        </div>
      )}
    </div>
  );
}

export default App;

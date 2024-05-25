use tokio::sync::broadcast;
use ws::{listen, Handler, Message, Result};

pub struct WebSocketServer {
    pub sender: broadcast::Sender<String>,
    pub connected: bool, // New field to track connection state
}

impl WebSocketServer {
    pub async fn start(sender: broadcast::Sender<String>) {
        if let Err(e) = listen("127.0.0.1:8080", move |_out| WebSocketServer {
            sender: sender.clone(),
            connected: false,
        }) {
            eprintln!("Failed to start WebSocket server: {}", e);
        }
    }
}

impl Handler for WebSocketServer {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(text) => {
                if !self.connected {
                    self.connected = true;
                    println!("WebSocket connection established.");
                }
                println!("Received message from database: {}", text);
                if self.connected {
                    if let Err(e) = self.sender.send(text.clone()) {
                        println!("Failed to send message to clients: {}", e);
                    } else {
                        println!("Sent message to clients: {}", text);
                    }
                } else {
                    println!("WebSocket connection not established, skipping message send.");
                }
            }
            Message::Binary(_) => {
                println!("Received binary message which is not supported.");
            }
        }
        Ok(())
    }
}

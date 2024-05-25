use tokio::sync::broadcast;
use ws::{listen, Handler, Message, Result};

struct WebSocketServer {
    sender: broadcast::Sender<String>,
}

// Inside the WebSocketServer implementation
impl Handler for WebSocketServer {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(text) => {
                println!("Received message from database: {}", text);
                if let Err(e) = self.sender.send(text.clone()) {
                    println!("Failed to send message to clients: {}", e);
                } else {
                    println!("Sent message to clients: {}", text);
                }
            }
            Message::Binary(_) => {
                // Handle binary messages if needed
                // For example, you can ignore them or log a warning
                println!("Received binary message which is not supported.");
            }
        }
        Ok(())
    }

    // Other handler methods...
}

pub fn start_websocket_server(sender: broadcast::Sender<String>) {
    println!("Starting WebSocket server...");
    listen("127.0.0.1:8080", move |_out| WebSocketServer {
        sender: sender.clone(),
    })
    .unwrap();
}

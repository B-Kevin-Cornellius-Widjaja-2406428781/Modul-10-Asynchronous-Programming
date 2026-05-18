use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, broadcast::{Sender, channel}};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClientMessage {
    message_type: ClientMessageType,
    data: Option<String>,
    data_array: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ClientMessageType {
    Register,
    Message,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerMessage {
    message_type: ServerMessageType,
    data: Option<String>,
    data_array: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum ServerMessageType {
    Users,
    Message,
}

#[derive(Debug, Serialize)]
struct ChatPayload {
    from: String,
    message: String,
    time: u128,
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    users: Arc<Mutex<HashMap<SocketAddr, String>>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            if let Ok(parsed) = serde_json::from_str::<ClientMessage>(text) {
                                match parsed.message_type {
                                    ClientMessageType::Register => {
                                        if let Some(name) = parsed.data {
                                            {
                                                let mut users = users.lock().await;
                                                users.insert(addr, name);
                                            }
                                            broadcast_users(&bcast_tx, &users).await?;
                                        }
                                    }
                                    ClientMessageType::Message => {
                                        let name = {
                                            let users = users.lock().await;
                                            users.get(&addr).cloned()
                                        };
                                        if let (Some(from), Some(message)) = (name, parsed.data) {
                                            let payload = ChatPayload {
                                                from,
                                                message,
                                                time: SystemTime::now()
                                                    .duration_since(UNIX_EPOCH)
                                                    .unwrap_or_default()
                                                    .as_millis(),
                                            };
                                            let outgoing = ServerMessage {
                                                message_type: ServerMessageType::Message,
                                                data: Some(serde_json::to_string(&payload)?),
                                                data_array: None,
                                            };
                                            bcast_tx.send(serde_json::to_string(&outgoing)?)?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(Err(err)) => {
                        eprintln!("ws error from {addr:?}: {err}");
                        break;
                    }
                    None => break,
                }
            }
            msg = bcast_rx.recv() => {
                match msg {
                    Ok(text) => ws_stream.send(Message::text(text)).await?,
                    Err(_) => break,
                }
            }
        }
    }

    {
        let mut users = users.lock().await;
        users.remove(&addr);
    }
    broadcast_users(&bcast_tx, &users).await?;

    Ok(())
}

async fn broadcast_users(
    bcast_tx: &Sender<String>,
    users: &Arc<Mutex<HashMap<SocketAddr, String>>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let list = {
        let users = users.lock().await;
        users.values().cloned().collect::<Vec<_>>()
    };
    let msg = ServerMessage {
        message_type: ServerMessageType::Users,
        data: None,
        data_array: Some(list),
    };
    let text = serde_json::to_string(&msg)?;
    let _ = bcast_tx.send(text);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);
    let users: Arc<Mutex<HashMap<SocketAddr, String>>> = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        let users = users.clone();
        tokio::spawn(async move {
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx, users).await
        });
    }
}
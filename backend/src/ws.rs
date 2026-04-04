//! Board-room fan-out using `tokio::sync::broadcast` and a single serialized `Arc<Vec<u8>>` per message.

use actix::prelude::*;
use actix::StreamHandler;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast;

const BROADCAST_CAP: usize = 1024;

#[derive(Clone)]
pub struct WsHub {
    inner: Arc<Mutex<HashMap<String, broadcast::Sender<Arc<Vec<u8>>>>>>,
}

impl WsHub {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn subscribe(&self, room: &str) -> broadcast::Receiver<Arc<Vec<u8>>> {
        let mut m = self.inner.lock().unwrap();
        let tx = m.entry(room.to_string()).or_insert_with(|| {
            let (t, _) = broadcast::channel(BROADCAST_CAP);
            t
        });
        tx.subscribe()
    }

    /// Fan-out one payload to every live subscriber in the room (`Arc` is cheap to clone per peer).
    pub fn publish(&self, room: &str, payload: Arc<Vec<u8>>) {
        let m = self.inner.lock().unwrap();
        if let Some(tx) = m.get(room) {
            let _ = tx.send(payload);
        }
    }
}

pub struct BoardWsSession {
    room: String,
    hub: WsHub,
    rx: broadcast::Receiver<Arc<Vec<u8>>>,
}

impl Actor for BoardWsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_millis(8), |act, ctx| {
            loop {
                match act.rx.try_recv() {
                    Ok(bytes) => {
                        let send_ok = match std::str::from_utf8(bytes.as_ref()) {
                            Ok(s) => ctx.text(s).is_ok(),
                            Err(_) => ctx.binary(bytes.as_ref().to_vec()).is_ok(),
                        };
                        if !send_ok {
                            ctx.stop();
                            return;
                        }
                    }
                    Err(broadcast::error::TryRecvError::Empty) => break,
                    Err(broadcast::error::TryRecvError::Lagged(n)) => {
                        log::warn!("ws client lagged {n} messages; disconnecting");
                        ctx.stop();
                        return;
                    }
                    Err(broadcast::error::TryRecvError::Closed) => {
                        ctx.stop();
                        return;
                    }
                }
            }
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for BoardWsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(t)) => {
                let payload = Arc::new(t.as_bytes().to_vec());
                self.hub.publish(&self.room, payload);
            }
            Ok(ws::Message::Binary(b)) => {
                let payload = Arc::new(b.to_vec());
                self.hub.publish(&self.room, payload);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Ping(p)) => ctx.pong(&p),
            _ => {}
        }
    }
}

pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    room: web::Path<String>,
    state: web::Data<crate::AppState>,
) -> Result<HttpResponse, Error> {
    let room = room.into_inner();
    let rx = state.ws_hub.subscribe(&room);
    let session = BoardWsSession {
        room,
        hub: state.ws_hub.clone(),
        rx,
    };
    ws::start(session, &req, stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn broadcast_distributes_arc_payload_to_all_receivers() {
        let hub = WsHub::new();
        let mut a = hub.subscribe("board-1");
        let mut b = hub.subscribe("board-1");
        let payload = Arc::new(vec![1u8, 2, 3, 4]);
        hub.publish("board-1", Arc::clone(&payload));
        assert_eq!(*a.recv().await.unwrap(), *payload);
        assert_eq!(*b.recv().await.unwrap(), *payload);
    }
}

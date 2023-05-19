use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::{
    sync::{
        mpsc,
        watch::{self},
    },
    time::timeout,
};

use super::message::{self, Message};

pub struct MessageState {
    event_tx: mpsc::UnboundedSender<Message>,
    handler: &'static dyn message::Handler,
    notify_map: Arc<Mutex<HashMap<String, watch::Sender<Option<Message>>>>>,
}

impl MessageState {
    pub fn new(handler: &'static dyn message::Handler) -> Self {
        let (event_tx, mut event_rx) = mpsc::unbounded_channel();
        let map = Arc::new(Mutex::new(HashMap::new()));

        let notify = map.clone();
        let state = MessageState {
            event_tx,
            handler,
            notify_map: notify,
        };

        {
            let map = map.clone();
            tokio::spawn(async move {
                // handler message
                while let Some(message) = event_rx.recv().await {
                    {
                        // this not ues `unwrap`
                        // we need a message list, if have handler error, just re-call this function.
                        println!("hanedler message {:?}", message);
                        handler.send_message(message.clone()).unwrap();
                        println!("hanedler message finish");
                    }
                    {
                        let key = message.recv.clone();
                        let mut map = map.lock().unwrap();
                        if let Some(tx) = map.get(&key) {
                            tx.send(Some(message)).unwrap();
                            map.remove(&key).unwrap();
                        }
                    }
                }
            });
        }

        state
    }

    // send message to server
    pub async fn send_message(&self, message: Message) {
        println!("send message");
        self.event_tx.send(message).unwrap();
    }

    // get message from server
    pub async fn recv_message(
        &self,
        username: &str,
        since: &str,
    ) -> Option<(Vec<Message>, String)> {
        // get message from hanlder
        // println!("recv message get from handler");
        let ret = self.handler.recv_message(username, since).unwrap();
        // println!("get ret:{:?}", ret);
        if let Some(ret) = ret {
            if ret.0.len() != 0 {
                return Some(ret);
            }
        }
        {
            // println!("wait");
            // if don't have message
            // we need wait new message
            let (tx, mut rx) = watch::channel(None);

            let key = String::from(username);
            {
                let mut map = self.notify_map.lock().unwrap();
                map.insert(key.clone(), tx);
            }

            {
                #[cfg(test)]
                let timeout = timeout(Duration::from_secs(2), rx.changed());
                #[cfg(not(test))]
                let timeout = timeout(Duration::from_secs(30), rx.changed());

                // println!("wait");
                match timeout.await {
                    Ok(result) => {
                        // println!("get message");
                        if result.is_ok() {
                            return rx.borrow().clone().map(|m| (vec![m], "1".to_string()));
                        } else {
                            return None;
                        }
                    }
                    Err(_) => {
                        // println!("timeout");
                        {
                            let mut map = self.notify_map.lock().unwrap();
                            map.remove(&key);
                        }
                        return None;
                    }
                }
            }
        }
    }
}


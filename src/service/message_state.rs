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

#[cfg(test)]
pub mod test {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
        time::Duration,
    };

    use tokio::{
        sync::{mpsc, watch},
        task,
        time::timeout,
    };

    use crate::service::{message::Message, services};

    pub async fn test_message_state() {
        // println!("test message state");

        // new message in server
        {
            services().state.recv_message("user0", "0").await.unwrap();
        }

        // wait new message
        {
            let recv_async = task::spawn(async {
                // println!("run recv");
                services().state.recv_message("user0", "4")
            });
            let message = recv_async.await.unwrap().await;
            // println!("message1: {:?}", message);
            assert_eq!(message, None);
        }

        // new message come
        {
            let raw_message = Message {
                send: "user1".to_string(),
                recv: "user0".to_string(),
                content: "message in state".to_string(),
                timestamp: "123".to_string(),
            };
            // wait new message
            let recv_async = task::spawn(async { services().state.recv_message("user0", "4") });

            // send message
            {
                tokio::spawn(async {
                    let message = Message {
                        send: "user1".to_string(),
                        recv: "user0".to_string(),
                        content: "message in state".to_string(),
                        timestamp: "123".to_string(),
                    };
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    services().state.send_message(message).await;
                });
            }

            // check message
            let message = recv_async.await.unwrap().await;
            assert!(message.is_some());
            assert_eq!(message, Some((vec![raw_message.clone()], "5".to_string())));
        }

        // tokio::time::sleep(Duration::from_secs(60)).await;
    }

    // #[tokio::test]
    async fn test_notify() {
        let mut map: Arc<Mutex<HashMap<&str, watch::Sender<i32>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let (tx1, mut rx1) = watch::channel::<i32>(0);
        let (tx2, mut rx2) = watch::channel::<i32>(0);

        {
            let mut map1 = map.lock().unwrap();
            map1.insert("A1", tx1);
            map1.insert("B2", tx2);
        }

        let (event_tx, mut event_rx) = mpsc::unbounded_channel::<(String, i32)>();
        {
            let map2 = map.clone();
            tokio::spawn(async move {
                while let Some(m) = event_rx.recv().await {
                    println!("thread a recv: {}: {}", m.0, m.1);
                    {
                        let key = m.0.as_str();
                        let mut map = map2.lock().unwrap();
                        if let Some(tx) = map.get(key) {
                            tx.send(m.1).unwrap();
                            map.remove(key).unwrap();
                        } else {
                            println!("no this key in map");
                        }
                    }
                }
            });
        }

        event_tx.send(("A1".to_string(), 1)).unwrap();

        tokio::spawn(async move {
            if rx1.changed().await.is_ok() {
                println!("received 1 = {:?}", *rx1.borrow());
            }
            println!("finish 1")
        });

        tokio::spawn(async move {
            match timeout(Duration::from_secs(3), rx2.changed()).await {
                Ok(result) => {
                    if result.is_ok() {
                        println!("received 2 = {:?}", *rx2.borrow());
                    }
                }
                Err(_) => {
                    println!("timeout!!!")
                }
            }
            println!("finish 2")
        });

        // event_tx.send(("B2".to_string(), 2)).unwrap();
        tokio::time::sleep(Duration::from_secs(2)).await;
        // event_tx.send(("B2".to_string(), 3)).unwrap();
        event_tx.send(("A1".to_string(), 4)).unwrap();

        tokio::time::sleep(Duration::from_secs(40)).await;
    }
}

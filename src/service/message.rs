use serde::{Deserialize, Serialize};
// bincode
use bincode;

use crate::utility::error::Result;

pub trait Handler: Send + Sync {
    // user send message
    fn send_message(&self, message: Message) -> Result<()>;
    // user get message since a given time
    fn recv_message(&self, username: &str, since: &str) -> Result<Option<(Vec<Message>, String)>>;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Message {
    pub send: String,
    pub recv: String,
    pub content: String,
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        bincode::deserialize(&bytes).unwrap()
    }
}

#[cfg(test)]
pub mod test {

    use crate::service::{message::Message, services};

    // #[test]
    pub fn test_message() {
        // setup
        // let tmp_dir = setup_services("test_user");

        // message 1
        {
            // message timecount=0 send=0 recv=1
            let message0_01 = Message {
                send: "user0".to_string(),
                recv: "user1".to_string(),
                content: "hello".to_string(),
            };
            services()
                .handler
                .send_message(message0_01.clone())
                .unwrap();

            // get user0
            let tuple0 = services()
                .handler
                .recv_message("user0", "0")
                .unwrap()
                .unwrap();
            assert_eq!(tuple0.0, vec![message0_01.clone()].to_vec());
            assert_eq!(tuple0.1, "1");

            // get user1
            let tuple1 = services()
                .handler
                .recv_message("user1", "0")
                .unwrap()
                .unwrap();
            assert_eq!(tuple1.0, vec![message0_01.clone()].to_vec());
            assert_eq!(tuple1.1, "1");
        }
        // now {user0 since = 1 , user1 since = 1}

        // multiple message
        {
            let message0_01 = Message {
                send: "user0".to_string(),
                recv: "user1".to_string(),
                content: "hello".to_string(),
            };
            services()
                .handler
                .send_message(message0_01.clone())
                .unwrap();

            let message1_10 = Message {
                send: "user1".to_string(),
                recv: "user0".to_string(),
                content: "world".to_string(),
            };
            services()
                .handler
                .send_message(message1_10.clone())
                .unwrap();

            let message2_20 = Message {
                send: "user2".to_string(),
                recv: "user0".to_string(),
                content: "hello".to_string(),
            };
            services()
                .handler
                .send_message(message2_20.clone())
                .unwrap();

            let message3_12 = Message {
                send: "user1".to_string(),
                recv: "user2".to_string(),
                content: "world".to_string(),
            };
            services()
                .handler
                .send_message(message3_12.clone())
                .unwrap();

            // get user0 message
            let tuple0 = services()
                .handler
                .recv_message("user0", "1")
                .unwrap()
                .unwrap();
            assert_eq!(
                tuple0.0,
                vec![
                    message0_01.clone(),
                    message1_10.clone(),
                    message2_20.clone()
                ]
                .to_vec()
            );
            assert_eq!(tuple0.1, "4");

            // get user1 message
            let tuple1 = services()
                .handler
                .recv_message("user1", "1")
                .unwrap()
                .unwrap();
            assert_eq!(
                tuple1.0,
                vec![
                    message0_01.clone(),
                    message1_10.clone(),
                    message3_12.clone(),
                ]
            );
            assert_eq!(tuple1.1, "4");

            // get user2 message
            let tuple2 = services()
                .handler
                .recv_message("user2", "0")
                .unwrap()
                .unwrap();
            assert_eq!(tuple2.0, vec![message2_20.clone(), message3_12.clone(),]);
            assert_eq!(tuple2.1, "2");
        }
        // delete
        // remove_dir_all(tmp_dir).unwrap();
    }
}

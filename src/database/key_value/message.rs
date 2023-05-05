use crate::{
    database::Database,
    service::message::{Handler, Message},
    utility::error::Result,
};

impl Database {
    fn get_count(&self, username: &str) -> Result<u128> {
        Ok(self
            .user_count
            .get(username.as_bytes())?
            .map(|x| u128::from_be_bytes(x.as_slice().try_into().unwrap()))
            .unwrap_or(0))
    }
}

impl Handler for Database {
    fn send_message(&self, message: Message) -> Result<()> {
        // let recv = format!("{}-recv", message.recv);

        // update recv
        let recv_count = self.get_count(message.recv.as_str())?;
        self.user_count
            .insert(message.recv.as_bytes(), &(recv_count + 1).to_be_bytes())?;

        // update send
        let send_count = self.get_count(&message.send)?;
        self.user_count
            .insert(message.send.as_bytes(), &(send_count + 1).to_be_bytes())?;

        // update message
        let sender = format!("{}{:06}", message.send, send_count);
        let recver = format!("{}{:06}", message.recv, recv_count);
        // let messageid = format!("{}{}_{}{}", sender, send_count, recver, recv_count);
        let messageid = format!("{}_{}", sender, recver);
        self.messageid_message
            .insert(messageid.as_bytes(), &message.to_bytes())?;

        // set message to sender and recver
        self.user_messageid
            .insert(sender.as_bytes(), messageid.as_bytes())?;
        self.user_messageid
            .insert(recver.as_bytes(), messageid.as_bytes())?;

        Ok(())
    }

    fn recv_message(&self, username: &str, since: &str) -> Result<Option<(Vec<Message>, String)>> {
        Ok(Some((
            self.user_messageid
                .iter_form(
                    username,
                    format!("{:06}", since.parse::<u128>().unwrap()).as_bytes(),
                )
                .map(|tuple| {
                    let messageid = tuple.1;
                    Message::from_bytes(self.messageid_message.get(&messageid).unwrap().unwrap())
                })
                .collect(),
            self.get_count(username)?.to_string(),
        )))
    }
}

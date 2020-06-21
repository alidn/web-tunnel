use crate::socketserver::{
    FileChunk, PasswordCorrect, PasswordWrong, SendPassword, StartSend, WsServer,
};
use actix::prelude::*;
use std::collections::HashMap;

pub struct Server {
    pairs: HashMap<String, Pair>,
    current_pass: i32,
}

struct Pair {
    pub sender: Addr<WsServer>,
    pub receiver: Option<Addr<WsServer>>,
}

pub struct SendRequest {
    pub recipient_addr: Addr<WsServer>,
}

pub struct EndOfFile(pub String);

pub struct ReceiveRequest {
    pub recipient_addr: Addr<WsServer>,
    pub password: String,
}

impl actix::Message for SendRequest {
    type Result = ();
}

impl actix::Message for ReceiveRequest {
    type Result = ();
}

impl actix::Message for EndOfFile {
    type Result = ();
}

impl Handler<EndOfFile> for Server {
    type Result = ();

    fn handle(&mut self, msg: EndOfFile, _ctx: &mut Self::Context) -> Self::Result {
        let password = msg.0;
        if let Some(pair) = self.pairs.get(&password) {
            if let Some(receiver) = &pair.receiver {
                receiver.do_send(EndOfFile(String::from("")));
            }
        }
    }
}

impl Handler<FileChunk> for Server {
    type Result = ();

    fn handle(&mut self, msg: FileChunk, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(pair) = self.pairs.get(&msg.password) {
            if let Some(receiver) = &pair.receiver {
                receiver.do_send(msg);
            }
        }
    }
}

impl Handler<SendRequest> for Server {
    type Result = ();

    fn handle(&mut self, msg: SendRequest, _ctx: &mut Context<Self>) -> Self::Result {
        // TODO: generate a unique 6 or 5 digit password.
        self.current_pass += 1;
        self.pairs.insert(
            self.current_pass.to_string(),
            Pair {
                sender: msg.recipient_addr.clone(),
                receiver: None,
            },
        );
        msg.recipient_addr.do_send(SendPassword {
            password: self.current_pass,
        });
    }
}

impl Handler<ReceiveRequest> for Server {
    type Result = ();

    fn handle(&mut self, msg: ReceiveRequest, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(pair) = self.pairs.get(&msg.password) {
            msg.recipient_addr.do_send(PasswordCorrect {});
            pair.sender.do_send(StartSend);
            self.pairs.insert(
                msg.password,
                Pair {
                    sender: pair.sender.clone(),
                    receiver: Some(msg.recipient_addr),
                },
            );
        } else {
            msg.recipient_addr.do_send(PasswordWrong {});
            ()
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            pairs: HashMap::new(),
            current_pass: 0,
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

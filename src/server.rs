use crate::socketserver::{PasswordCorrect, SendPassword, WsServer};
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

impl Handler<SendRequest> for Server {
    type Result = ();

    fn handle(&mut self, msg: SendRequest, _ctx: &mut Context<Self>) -> Self::Result {
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

    fn handle(&mut self, msg: ReceiveRequest, ctx: &mut Self::Context) -> Self::Result {
        if let Some(pair) = self.pairs.get(&msg.password) {
            msg.recipient_addr.do_send(PasswordCorrect {});
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

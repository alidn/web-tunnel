use crate::socketserver::{
    ClientMessage, FileChunk, PasswordCorrect, PasswordWrong, SendPassword, StartSend, WsServer,
    WsServerMessage,
};
use actix::prelude::*;
use std::collections::HashMap;

pub struct Server {
    pairs: HashMap<String, Pair>,
    current_pass: i32,
}

impl Server {
    pub fn get_pair(&self, pass: String) -> Option<&Pair> {
        self.pairs.get(&pass)
    }

    pub fn get_sender(&self, pass: String) -> Option<&Addr<WsServer>> {
        self.pairs
            .get(&pass)
            .map_or_else(|| None, |pair| Some(&pair.sender))
    }

    pub fn get_receiver(&self, pass: String) -> Option<&Addr<WsServer>> {
        self.pairs
            .get(&pass)
            .map_or_else(|| None, |pair| pair.receiver.as_ref())
    }
}

pub struct Pair {
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

pub struct Password(pub i32);

impl actix::Message for Password {
    type Result = ();
}

impl actix::Message for SendRequest {
    type Result = i32;
}

impl actix::Message for ReceiveRequest {
    type Result = ();
}

impl actix::Message for EndOfFile {
    type Result = ();
}

impl Handler<WsServerMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: WsServerMessage, _ctx: &mut Self::Context) -> Self::Result {
        // TODO: remove unwrapping password
        match msg.client_message {
            ClientMessage::FileChunk(_) => {
                if let Some(receiver) = self.get_receiver(msg.password.unwrap().to_string()) {
                    receiver.do_send(msg.client_message);
                } else {
                    msg.ws_addr.do_send(PasswordWrong);
                }
            }
            ClientMessage::EOF => {
                if let Some(receiver) = self.get_receiver(msg.password.unwrap()) {
                    receiver.do_send(ClientMessage::EOF);
                }
            }
            ClientMessage::ReceiveRequest(pass) => {
                if let Some(sender) = self.get_sender(pass.clone()) {
                    msg.ws_addr.do_send(PasswordCorrect);
                    sender.do_send(StartSend);
                    self.pairs.insert(
                        pass,
                        Pair {
                            sender: sender.clone(),
                            receiver: Some(msg.ws_addr),
                        },
                    );
                } else {
                    msg.ws_addr.do_send(PasswordWrong);
                }
            }
            ClientMessage::SendRequest => {
                self.current_pass += 1;
                msg.ws_addr.do_send(Password(self.current_pass));
                self.pairs.insert(
                    self.current_pass.to_string(),
                    Pair {
                        sender: msg.ws_addr,
                        receiver: None,
                    },
                );
            }
            ClientMessage::Undefined => {}
        };
    }
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
    type Result = i32;

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
        // msg.recipient_addr.do_send(SendPassword {
        //     password: self.current_pass,
        // });
        self.current_pass
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

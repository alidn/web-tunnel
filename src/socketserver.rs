use crate::server::{EndOfFile, ReceiveRequest, SendRequest, Server};
use actix::*;
use actix_http::ws::Item;
use actix_web_actors::ws;
use bytes::Bytes;

pub struct WsServer {
    pub server_addr: Addr<Server>,
    pub password: Option<String>,
}

impl Actor for WsServer {
    type Context = ws::WebsocketContext<Self>;
}

pub struct PasswordCorrect;

pub struct PasswordWrong;

pub struct FileChunk {
    pub chunk: Bytes,
    pub password: String,
}

impl actix::Message for PasswordWrong {
    type Result = ();
}

impl actix::Message for FileChunk {
    type Result = ();
}

impl actix::Message for PasswordCorrect {
    type Result = ();
}

pub struct SendPassword {
    pub password: i32,
}

pub struct StartSend;

impl actix::Message for SendPassword {
    type Result = ();
}

impl actix::Message for StartSend {
    type Result = ();
}

impl Handler<StartSend> for WsServer {
    type Result = ();

    fn handle(&mut self, _msg: StartSend, ctx: &mut Self::Context) -> Self::Result {
        ctx.text("/startsend")
    }
}

impl Handler<SendPassword> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: SendPassword, ctx: &mut Self::Context) -> Self::Result {
        self.password = Some(msg.password.to_string());
        ctx.text(msg.password.to_string())
    }
}

impl Handler<PasswordCorrect> for WsServer {
    type Result = ();

    fn handle(&mut self, _msg: PasswordCorrect, ctx: &mut Self::Context) -> Self::Result {
        ctx.text("/correct")
    }
}

impl Handler<PasswordWrong> for WsServer {
    type Result = ();

    fn handle(&mut self, _msg: PasswordWrong, ctx: &mut Self::Context) -> Self::Result {
        self.password = None;
        ctx.text("/wrong");
    }
}

impl Handler<EndOfFile> for WsServer {
    type Result = ();

    fn handle(&mut self, _msg: EndOfFile, ctx: &mut Self::Context) -> Self::Result {
        ctx.text("/eof");
    }
}

impl Handler<FileChunk> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: FileChunk, ctx: &mut Self::Context) -> Self::Result {
        ctx.binary(msg.chunk)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsServer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(message) = msg {
            if let ws::Message::Text(text_message) = message {
                if text_message.starts_with("/send") {
                    self.server_addr.do_send(SendRequest {
                        recipient_addr: ctx.address(),
                    });
                    println!("{}", text_message);
                } else if text_message.starts_with("/receive") {
                    self.password = Some(text_message[8..].to_string());
                    self.server_addr.do_send(ReceiveRequest {
                        recipient_addr: ctx.address(),
                        password: text_message[8..].to_string(),
                    });
                    println!("{}", text_message);
                } else if text_message.starts_with("/done") {
                    self.server_addr
                        .do_send(EndOfFile(self.password.clone().unwrap()));
                    println!("{}", text_message);
                } else {
                    println!("Received unknown message");
                }
            } else if let ws::Message::Binary(binary_message) = message {
                println!("Received a chunk");
                self.server_addr.do_send(FileChunk {
                    chunk: binary_message,
                    password: self.password.clone().unwrap(),
                });
            } else if let ws::Message::Continuation(item) = message {
                match item {
                    Item::FirstText(binary_message) => {
                        self.server_addr.do_send(FileChunk {
                            chunk: binary_message,
                            password: self.password.clone().unwrap(),
                        });
                    }
                    Item::FirstBinary(binary_message) => {
                        self.server_addr.do_send(FileChunk {
                            chunk: binary_message,
                            password: self.password.clone().unwrap(),
                        });
                    }
                    Item::Continue(binary_message) => {
                        self.server_addr.do_send(FileChunk {
                            chunk: binary_message,
                            password: self.password.clone().unwrap(),
                        });
                    }
                    Item::Last(binary_message) => {
                        self.server_addr.do_send(FileChunk {
                            chunk: binary_message,
                            password: self.password.clone().unwrap(),
                        });
                    }
                }
            } else {
                println!("Message with unknown type")
            }
        }
    }
}

use crate::server::{Password, Server};
use actix::*;
use actix_http::ws::Item;
use actix_web_actors::ws;
use bytes::Bytes;

pub enum ClientMessage {
    FileChunk(Bytes),
    SendRequest,
    ReceiveRequest(String),
    EOF,
    Undefined,
    Metadata(String)
}

impl actix::Message for ClientMessage {
    type Result = ();
}

pub struct WsServerMessage {
    pub client_message: ClientMessage,
    pub ws_addr: Addr<WsServer>,
    pub password: Option<String>,
}

impl actix::Message for WsServerMessage {
    type Result = ();
}

impl Handler<ClientMessage> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ClientMessage::FileChunk(chunk) => ctx.binary(chunk),
            ClientMessage::EOF => ctx.text("/eof"),
            ClientMessage::Metadata(metadata) => ctx.text(format!("/metadata{}", metadata)),
            _ => {}
        }
    }
}

impl From<ws::Message> for ClientMessage {
    fn from(message: ws::Message) -> Self {
        match message {
            ws::Message::Binary(file_chunk) => ClientMessage::FileChunk(file_chunk),
            ws::Message::Continuation(item) => match item {
                Item::FirstText(file_chunk)
                | Item::FirstBinary(file_chunk)
                | Item::Continue(file_chunk)
                | Item::Last(file_chunk) => ClientMessage::FileChunk(file_chunk),
            },
            ws::Message::Text(text_message) => {
                if text_message.starts_with("/send") {
                    ClientMessage::SendRequest
                } else if text_message.starts_with("/receive") {
                    ClientMessage::ReceiveRequest(text_message[8..].to_string())
                } else if text_message.starts_with("/done") {
                    ClientMessage::EOF
                } else if text_message.starts_with("/metadata") {
                    ClientMessage::Metadata(text_message[9..].to_string())
                } else {
                    ClientMessage::Undefined
                }
            }
            _ => ClientMessage::Undefined,
        }
    }
}

pub struct WsServer {
    pub server_addr: Addr<Server>,
    pub password: Option<String>,
}

impl Actor for WsServer {
    type Context = ws::WebsocketContext<Self>;
}

pub struct PasswordCorrect;

pub struct PasswordWrong;

impl actix::Message for PasswordWrong {
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

impl Handler<Password> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Password, ctx: &mut Self::Context) -> Self::Result {
        self.password = Some(msg.0.to_string());
        ctx.text(format!("/code{}", msg.0))
    }
}

impl Handler<StartSend> for WsServer {
    type Result = ();

    fn handle(&mut self, _msg: StartSend, ctx: &mut Self::Context) -> Self::Result {
        ctx.text("/startsend")
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

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsServer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(message) = msg {
            let client_message = ClientMessage::from(message);
            self.server_addr.do_send(WsServerMessage {
                client_message,
                ws_addr: ctx.address(),
                password: self.password.clone()
            });
        }
    }
}

use crate::server::{ReceiveRequest, SendRequest, Server};
use actix::*;
use actix_web_actors::ws;

pub struct WsServer {
    pub(crate) server_addr: Addr<Server>,
}

impl Actor for WsServer {
    type Context = ws::WebsocketContext<Self>;
}

pub struct PasswordCorrect;

impl actix::Message for PasswordCorrect {
    type Result = ();
}

pub struct SendPassword {
    pub password: i32,
}

impl actix::Message for SendPassword {
    type Result = ();
}

impl Handler<SendPassword> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: SendPassword, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.password.to_string())
    }
}

impl Handler<PasswordCorrect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: PasswordCorrect, ctx: &mut Self::Context) -> Self::Result {
        ctx.text("Password Correct!")
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsServer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text_message)) = msg {
            if text_message.starts_with("/send") {
                self.server_addr.do_send(SendRequest {
                    recipient_addr: ctx.address(),
                });
            } else {
                self.server_addr.do_send(ReceiveRequest {
                    recipient_addr: ctx.address(),
                    password: text_message,
                });
            }
        }
    }
}

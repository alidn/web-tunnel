use actix::*;
use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod server;
mod socketserver;

async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    server_addr: web::Data<Addr<server::Server>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        socketserver::WsServer {
            server_addr: server_addr.get_ref().clone(),
            password: None,
        },
        &req,
        stream,
    )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server_addr = server::Server::default().start();

    HttpServer::new(move || {
        App::new()
            .data(server_addr.clone())
            .service(web::resource("/ws/").to(ws_route))
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}

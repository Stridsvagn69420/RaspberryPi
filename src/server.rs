use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
pub async fn server(bindaddr: &str, staticroot: &'static str, unix: bool) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .service(Files::new("/", staticroot).prefer_utf8(true))
            .service(hello)
    });
    if unix {
        server.bind_uds(bindaddr)?.run().await
    } else {
        server.bind(bindaddr)?.run().await
    }
    
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
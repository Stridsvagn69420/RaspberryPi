use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use kagero::printer::{Colors, Printer};
use crate::config;
use crate::filesystem::path::{BindType, validate_bind};

#[actix_web::main]
pub async fn server(settings: config::Config) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(Files::new("/", &settings.server.path)
                .prefer_utf8(true)
                .index_file("index.html")
            )
    });
    
    match validate_bind(&settings.server.bindaddr) {
        BindType::IPv4 => {
            return server.bind(settings.server.bindaddr)?.run().await
        },

        #[cfg(unix)]
        BindType::Unix => {
            return server.bind_uds(settings.server.bindaddr)?.run().await
        },

        BindType::NotValid => {
            let mut prnt = Printer::default();
            prnt.error("server.bindaddr", Colors::RedBright);
            prnt.errorln(" isn't a valid bind address!", Colors::Red);
            prnt.println("Bind address couldn't be used. Server will attempt to bind to http://0.0.0.0:9000...", Colors::Yellow);
            return server.bind("0.0.0.0:9000")?.run().await;
        }
    }
    
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

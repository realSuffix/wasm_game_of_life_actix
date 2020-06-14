use actix_web::{App, HttpServer, Responder, HttpResponse, web, Result};
use actix_files as fs;
use actix_files::NamedFile;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}


async fn serve_index_html() -> Result<NamedFile> {
    Ok(NamedFile::open("../wasm_game_of_life/www/dist/index.html")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .route("/test", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(web::resource("/wasm-game-of-life/").route(web::get().to(serve_index_html)))
            .service(fs::Files::new("/wasm-game-of-life/", "../wasm_game_of_life/www/dist"))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

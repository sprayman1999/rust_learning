use std::fs::File;

use actix_files::Files;
use actix_web::{HttpServer,App};

#[actix_web::main]
async fn main() -> std::io::Result::<()>{
    HttpServer::new(||{
        App::new()
            .service(Files::new("/static","assets/static"))
            .service(Files::new("/public","assets/public").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
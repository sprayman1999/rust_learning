use actix_web::{HttpServer,App,get,Responder,web};

#[get("/index.html")]
async fn hello_actix_web() -> impl Responder{
    "Hello Actix Web"
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new().service(hello_actix_web)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
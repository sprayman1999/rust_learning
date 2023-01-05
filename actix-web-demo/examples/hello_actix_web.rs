use actix_web::{HttpServer,App,get,Responder,web};

#[get("/hello/{name}")]
async fn hello_actix_web(name: web::Path<String>) -> impl Responder{
    format!("Hello Actix Web, path argv is {}",name)
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
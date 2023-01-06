use actix_web::{HttpServer, get, post, HttpResponse, App ,web};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct CustomRequestJson{
    name: String,
    age: i32,
}

// curl -X POST 127.0.0.1:8080/hello_json -H "Content-type: application/json" -d '{"name":"1","age":1}'
async fn hello_json(item: web::Json<CustomRequestJson>) -> HttpResponse{
    println!("receive request: {:?}",item);
    HttpResponse::Ok().json(item.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/hello_json").route(web::post().to(hello_json)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
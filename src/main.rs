use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}
#[get("/")]
async fn hello() -> impl Responder {
    let data: &str = "kkl";
    // HttpResponse::Ok().body("Hello world!!")
    let obj = MyObj {
        name: data.to_string(),
    };
    HttpResponse::Ok().json(obj)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
    // HttpResponse::Ok().body("Hey theree!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
use std::sync::Mutex;

use actix_files::Files;
use actix_web::{get, web, web::Data, App, HttpResponse, HttpServer, Responder};
use log::info;
use my_web_app::MyTestStruct;

#[get("/hello")]
async fn hello() -> impl Responder {
    info!("Sending a String.");
    "Hallo Welt"
}

#[get("/json-data")]
async fn jsondata(counter: Data<Mutex<i32>>) -> impl Responder {
    let mut v = counter.lock().unwrap();
    *v += 1;
    let data = MyTestStruct::from(*v);
    info!("Data: {:?}", data);
    info!("Sending: {:?}", counter);
    serde_json::to_string(&data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(Mutex::new(0))
            .service(hello)
            .service(jsondata)
            .service(Files::new("/", "./dist/").index_file("index.html"))
            .default_service(
                web::route().to(|| HttpResponse::Found().header("Location", "/").finish()),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

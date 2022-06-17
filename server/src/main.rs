use rust_gpiozero::*;
use actix_web::{get, web, route, App, HttpServer, Responder};


#[get("/greet/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("hello {name}")
}

#[route("/led", method="GET")]
async fn turn_led() -> impl Responder {
    let mut led = LED::new(26);
    led.toggle();
    led.wait();
    format!("Turned on LED")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(turn_led)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


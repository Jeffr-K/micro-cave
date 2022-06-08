mod models;

use std::io;
use actix_web::{ HttpServer, App, web, Responder, HttpResponse };
use crate::models::Status;

async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(Status { status: "Up".to_string() })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    
    println!(" \n The server is running at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
        .route("/health", web::get().to(status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

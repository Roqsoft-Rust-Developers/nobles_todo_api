use actix_web::{web, App, HttpServer, Responder};
use actix_web::HttpResponse;
use crate::models::Status;
// use actix_rt;
use std::io;
mod models;
mod config;
async fn status() -> impl Responder {
    // "{\"status\":\"OK\"}"
    HttpResponse::Ok()
    .json(Status {status:"OK".to_string() })
}

#[actix_rt::main]
async fn main()-> io::Result<()> {
    println!("Starting server at localhost:8080");
    HttpServer::new({|| 
        
        App::new()
    .route("/", web::get().to(status))})


        .bind("127.0.0.1:8080")?
        .run()
        .await
}

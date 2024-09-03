mod image_service;

use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize};
use reqwest::Client;
use std::sync::Arc;
use actix_web::web::Bytes;
use crate::image_service::ImageDimensions;

#[derive(Deserialize)]
struct PathParams {
    source: String,
}

#[derive(Deserialize)]
struct QueryParams {
    width: u32,//TODO: make both these optional
    height: u32,
}

#[get("/raw/{source}")]
async fn raw(params: web::Path<PathParams>, query: web::Query<QueryParams>, client: web::Data<Arc<Client>>) -> impl Responder {
    match get_image(&params.source, client.get_ref().clone()).await {
        Ok(resp) => {
            match image_service::resize((&resp).to_vec(), ImageDimensions{width: Some(query.width), height: Some(query.height)}, None) {
                Ok(result) => {
                    actix_web::HttpResponse::Ok()
                        .content_type("image/jpeg")
                        .body(result)
                },
                Err(e) => {
                    actix_web::HttpResponse::InternalServerError().body(format!("Something went wrong transforming image: {}", e))
                }
            }
        }
        Err(e) => {
            actix_web::HttpResponse::InternalServerError().body(format!("Something went wrong: {}", e))
        }
    }
}

async fn get_image(source: &String, client: Arc<Client>) -> Result<Bytes, Box<dyn std::error::Error>> {
    let bytes = client.get(source).send().await?.bytes().await?;

    Ok(bytes)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(Client::new());

    HttpServer::new(move || { // move is needed as client can be outlived
        let image_scope = web::scope("/images").service(raw);
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(image_scope)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


mod transforms;
pub mod image_service;
mod transform_processor;
mod chain;
mod canvas;

use std::str::FromStr;
use actix_web::{App, get, HttpServer, Responder, web};
use hex_color::HexColor;
use serde::Deserialize;
use reqwest::Client;
use std::sync::Arc;
use actix_web::web::Bytes;
use image::Rgb;
use crate::image_service::{Fit, ImageTransformOptions};

#[derive(Deserialize)]
struct PathParams {
    source: String,
}

#[derive(Deserialize)]
struct QueryParams {
    width: Option<u32>,
    height: Option<u32>,
    fit: Option<String>,
    bgcolor: Option<String>
}

#[get("/raw/{source}")]
async fn raw(params: web::Path<PathParams>, query: web::Query<QueryParams>, client: web::Data<Arc<Client>>) -> impl Responder {
    let fit_mode: Option<Fit> = query.fit.as_ref().map(|fit| Fit::from_str(fit.as_str()).unwrap());
    let bg_color: Option<Rgb<u8>> = match query.bgcolor.as_ref() {
        Some(color) => {
            match HexColor::parse_rgb(format!("#{}", color).as_str()) {
                Ok(color) => { Some(Rgb([color.r, color.g, color.b]))},
                Err(_) => { None }
            }

        },
        None => None
    };

    match get_image(&params.source, client.get_ref().clone()).await {
        Ok(resp) => {
            match image_service::resize_service((&resp).to_vec(), ImageTransformOptions{fit: fit_mode, width: query.width, height: query.height, bg_color}) {
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

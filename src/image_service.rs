use std::error::Error;
use std::io::Cursor;
use image::{ImageOutputFormat};
use actix_web::web::Bytes;

pub fn resize(img: &Bytes, width: u32, height: u32) -> Result<Vec<u8>, Box<dyn Error>> {
    let img_instance = image::load_from_memory(&img)?;

    let resized_img = img_instance.resize(width, height, image::imageops::FilterType::Lanczos3);

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);

    resized_img.write_to(&mut cursor, ImageOutputFormat::Jpeg(100)).map_err(|_| "Failed to write image".to_string())?;

    Ok(buf)
}



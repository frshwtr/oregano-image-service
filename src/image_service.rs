use crate::transforms::resize;

use std::error::Error;
use std::io::Cursor;
use std::str::FromStr;
use image::{DynamicImage, ImageFormat, Rgb};

#[derive(Debug, PartialEq)]
pub enum Fit {
    Contain,
    Pad,
}

impl FromStr for Fit {
    type Err = ();

    fn from_str(input: &str) -> Result<Fit, Self::Err> {
        match input {
            "pad" => Ok(Fit::Pad),
            "contain" => Ok(Fit::Contain),
            _ => Err(())
        }
    }
}

pub struct ImageTransformOptions {
    pub fit: Option<Fit>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub bg_color: Option<Rgb<u8>>
}

pub fn resize_service(img: Vec<u8>, options: ImageTransformOptions) -> Result<Vec<u8>, Box<dyn Error>> {
    let img_instance = image::load_from_memory(&img)?;
    let resize_width: u32 = if options.width.is_some() { options.width.unwrap() } else { img_instance.width() };
    let resize_height: u32 = if options.height.is_some() { options.height.unwrap() } else { img_instance.height() };
    let resized_img: DynamicImage;

    resized_img = match options.fit {
        Some(Fit::Pad) => {
            resize::resize_with_pad(&img_instance, resize_width, resize_height, options.bg_color)
        }
        _ => img_instance.resize_exact(resize_width, resize_height, image::imageops::FilterType::Lanczos3)
    };

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);

    resized_img.write_to(&mut cursor, ImageFormat::Jpeg).map_err(|_| "Failed to write image".to_string())?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use std::{fs};
    use crate::image_service::{ImageTransformOptions, resize_service};

    #[rstest]
    #[case(300)]
    #[case(134)]
    #[case(999)]
    #[case(6143)]
    #[case(1)]
    fn resizes_width(#[case] width: u32) {
        let test_img: Vec<u8> = fs::read("../../test/assets/test_img.png").unwrap();
        let result = resize_service(test_img, ImageTransformOptions { width: Some(width), height: Some(300), fit: None, bg_color: None });
        let result_img = image::load_from_memory(&result.unwrap());
        assert_eq!(result_img.unwrap().width(), width);
    }

    #[rstest]
    #[case(300)]
    #[case(134)]
    #[case(999)]
    #[case(6132)]
    #[case(1)]
    fn resizes_height(#[case] height: u32) {
        let test_img: Vec<u8> = fs::read("../test/assets/test_img.png").unwrap();
        let result = resize_service(test_img, ImageTransformOptions { width: Some(300), height: Some(height), fit: None, bg_color: None });
        let result_img = image::load_from_memory(&result.unwrap());

        assert_eq!(result_img.unwrap().height(), height)
    }
    #[test]
    fn preserves_width_when_none() {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize_service(test_img, ImageTransformOptions { width: None, height: Some(300), fit: None, bg_color: None });
        let result_img = image::load_from_memory(&result.unwrap());
        assert_eq!(result_img.unwrap().width(), 100)
    }

    #[test]
    fn preserves_height_when_none() {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize_service(test_img, ImageTransformOptions { width: Some(300), height: None, fit: None, bg_color: None });
        let result_img = image::load_from_memory(&result.unwrap());
        assert_eq!(result_img.unwrap().height(), 100)
    }
}

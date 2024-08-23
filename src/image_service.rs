use std::error::Error;
use std::io::Cursor;
use image::{ImageFormat};

pub struct ImageDimensions {
    pub width: Option<u32>,
    pub height: Option<u32>
}

pub fn resize(img: Vec<u8>, target_dimensions: ImageDimensions) -> Result<Vec<u8>, Box<dyn Error>> {
    let img_instance = image::load_from_memory(&img)?;
    let resize_width: u32 = if target_dimensions.width.is_some() {target_dimensions.width.unwrap()} else {img_instance.width()};
    let resize_height: u32 = if target_dimensions.height.is_some() {target_dimensions.height.unwrap()} else {img_instance.height()};

    let resized_img = img_instance.resize_exact(resize_width, resize_height, image::imageops::FilterType::Lanczos3);

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);

    resized_img.write_to(&mut cursor, ImageFormat::Jpeg).map_err(|_| "Failed to write image".to_string())?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use std::fs;
    use crate::image_service::{resize, ImageDimensions};

    #[rstest]
    #[case(300)]
    #[case(134)]
    #[case(999)]
    #[case(6143)]
    #[case(1)]
    fn resizes_width(#[case] width: u32) {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize(test_img, ImageDimensions{width: Some(width), height: Some(300)});
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
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize(test_img, ImageDimensions{width: Some(300), height: Some(height)});
        let result_img = image::load_from_memory(&result.unwrap());

        assert_eq!(result_img.unwrap().height(), height)
    }
    #[test]
    fn preserves_width_when_none() {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize(test_img, ImageDimensions{width: None, height: Some(300)});
        let result_img = image::load_from_memory(&result.unwrap());
        assert_eq!(result_img.unwrap().width(), 100)
    }
    #[test]
    fn preserves_height_when_none() {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize(test_img, ImageDimensions{width: Some(300), height: None});
        let result_img = image::load_from_memory(&result.unwrap());
        assert_eq!(result_img.unwrap().height(), 100)
    }
}
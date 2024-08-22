use std::error::Error;
use std::io::Cursor;
use image::{ImageFormat};


pub fn resize(img: Vec<u8>, width: u32, height: u32) -> Result<Vec<u8>, Box<dyn Error>> {
    let img_instance = image::load_from_memory(&img)?;

    let resized_img = img_instance.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);

    resized_img.write_to(&mut cursor, ImageFormat::Jpeg).map_err(|_| "Failed to write image".to_string())?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use std::fs;
    use crate::image_service::resize;

    #[rstest]
    #[case(300)]
    #[case(134)]
    #[case(999)]
    #[case(6143)]
    #[case(1)]
    fn resizes_width(#[case] width: u32) {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize(test_img, width, 300);
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
        let result = resize(test_img, 300, height);
        let result_img = image::load_from_memory(&result.unwrap());

        assert_eq!(result_img.unwrap().height(), height)
    }
}
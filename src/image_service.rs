use std::error::Error;
use std::io::Cursor;
use std::str::FromStr;
use image::{DynamicImage, ImageFormat};

#[derive(Debug, PartialEq)]
pub enum Fit {
    Pad
}

impl FromStr for Fit {
    type Err = ();

    fn from_str(input: &str) -> Result<Fit, Self::Err> {
        match input {
            "pad" => Ok(Fit::Pad),
            _ => Err(())
        }
    }
}

pub struct ImageTransformOptions {
    pub fit: Option<Fit>,
    pub width: Option<u32>,
    pub height: Option<u32>
}

pub fn resize(img: Vec<u8>, options: ImageTransformOptions) -> Result<Vec<u8>, Box<dyn Error>> {
    let img_instance = image::load_from_memory(&img)?;
    let resize_width: u32 = if options.width.is_some() {options.width.unwrap()} else {img_instance.width()};
    let resize_height: u32 = if options.height.is_some() {options.height.unwrap()} else {img_instance.height()};
    let resized_img: DynamicImage;


    resized_img = match options.fit {
        Some(Fit::Pad) => img_instance.resize(resize_width, resize_height, image::imageops::FilterType::Lanczos3),
        _ => img_instance.resize_exact(resize_width, resize_height, image::imageops::FilterType::Lanczos3)
    };


    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);

    resized_img.write_to(&mut cursor, ImageFormat::Jpeg).map_err(|_| "Failed to write image".to_string())?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use rstest::rstest;
    use std::{fmt, fs};
    use crate::image_service::{resize, Fit, ImageTransformOptions};

    #[derive(Debug)]
    struct TestError {
        details: String
    }

    impl TestError {
        fn new(msg: &str) -> TestError {
            TestError {
                details: msg.to_string()
            }
        }
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl Error for TestError {}

    #[rstest]
    #[case(300)]
    #[case(134)]
    #[case(999)]
    #[case(6143)]
    #[case(1)]
    fn resizes_width(#[case] width: u32) {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize(test_img, ImageTransformOptions{width: Some(width), height: Some(300), fit: None});
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
        let result = resize(test_img, ImageTransformOptions{width: Some(300), height: Some(height), fit: None});
        let result_img = image::load_from_memory(&result.unwrap());

        assert_eq!(result_img.unwrap().height(), height)
    }
    #[test]
    fn preserves_width_when_none() {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize(test_img, ImageTransformOptions{width: None, height: Some(300), fit: None});
        let result_img = image::load_from_memory(&result.unwrap());
        assert_eq!(result_img.unwrap().width(), 100)
    }
    #[test]
    fn preserves_height_when_none() {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize(test_img, ImageTransformOptions{width: Some(300), height: None, fit: None});
        let result_img = image::load_from_memory(&result.unwrap());
        assert_eq!(result_img.unwrap().height(), 100)
    }

    #[rstest]
    #[case(300)]
    #[case(900)]
    fn preserves_aspect_ratio_when_fit_is_pad(#[case] height: u32) -> Result<(), Box<dyn Error>> {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize(test_img, ImageTransformOptions{width: Some(height), height: None, fit: Some(Fit::Pad)});
        let result_img = image::load_from_memory(&result.unwrap())?;

        let result_aspect_ratio: u32 = result_img.clone().width() / result_img.height();

        if result_aspect_ratio == 1 {
            Ok(())
        } else { Err(Box::new(TestError::new(format!("Aspect ratio was not equal to 1, received {}", result_aspect_ratio).as_str()))) }
    }
}

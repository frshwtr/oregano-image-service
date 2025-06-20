use crate::chain::{ProcessOptions, ProcessRecord, ProcessableImage, ResizeOptions};
use crate::transform_processor::resize::Resize;
use crate::transform_processor::{Canvas, TransformProcessor};
use image::{ImageFormat, Rgb};
use std::error::Error;
use std::io::Cursor;
use std::str::FromStr;
use image::codecs::jpeg::JpegEncoder;

#[derive(Debug, PartialEq)]
pub enum Fit {
    Contain,
    Pad,
    ScaleDown,
    Fill,
    Cover,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Format {
    Auto,
    Gif,
    Jpeg,
    Png,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Quality {
    Lowest,
    Low,
    Medium,
    High,
    Highest,
    Lossless,
}

impl FromStr for Fit {
    type Err = ();

    fn from_str(input: &str) -> Result<Fit, Self::Err> {
        match input {
            "pad" => Ok(Fit::Pad),
            "contain" => Ok(Fit::Contain),
            "scale-down" => Ok(Fit::ScaleDown),
            "cover" => Ok(Fit::Cover),
            "fill" => Ok(Fit::Fill),
            _ => Err(()),
        }
    }
}

impl FromStr for Format {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gif" => Ok(Format::Gif),
            "jpeg" |
            "jpg" => Ok(Format::Jpeg),
            "png" => Ok(Format::Png),
            "auto" => Ok(Format::Auto),
            _ => Err(())
        }
    }
}

impl FromStr for Quality {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lowest" => Ok(Quality::Lowest),
            "low" => Ok(Quality::Low),
            "medium" => Ok(Quality::Medium),
            "high" => Ok(Quality::High),
            "highest" => Ok(Quality::Highest),
            "lossless" => Ok(Quality::Lossless),
            _ => Err(())
        }
    }
}

fn map_quality(q: Quality) -> u8 {
    match q {
        Quality::Lowest => 10,
        Quality::Low => 30,
        Quality::Medium => 50,
        Quality::High => 75,
        Quality::Highest => 95,
        Quality::Lossless => 100,
    }
}
pub struct ImageTransformOptions {
    pub fit: Option<Fit>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub bg_color: Option<Rgb<u8>>,
    pub dpr: Option<u8>,
    pub format: Option<Format>,
    pub quality: Option<Quality>,
}

pub fn resize_service(
    img: Vec<u8>,
    options: ImageTransformOptions,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let img_instance = image::load_from_memory(&img)?;
    let resize_width: u32 = if options.width.is_some() {
        options.width.unwrap()
    } else {
        img_instance.width()
    };
    let resize_height: u32 = if options.height.is_some() {
        options.height.unwrap()
    } else {
        img_instance.height()
    };


    let mut img_for_processing = ProcessableImage {
        src_img: img_instance,
        out_img: None,
        process_record: ProcessRecord {
            is_canvas_processed: false,
            is_image_resized: false,
            is_bg_color_applied: false,
        },
        process_options: ProcessOptions {
            resize: ResizeOptions {
                w: resize_width,
                h: resize_height,
                mode: options.fit.unwrap_or(Fit::Contain),
                dpr: options.dpr.unwrap_or(1),
            },
            bg_color: options.bg_color,
        },
    };
    let mut canvas = Canvas::new(Resize::default());

    canvas.execute(&mut img_for_processing);

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);

    match options.format.unwrap_or(Format::Auto) {
        Format::Gif => {
            img_for_processing
                .out_img
                .unwrap()
                .write_to(&mut cursor, ImageFormat::Gif)
                .map_err(|_| "Failed to write image".to_string())?;
        }
        Format::Png => {
            img_for_processing
                .out_img
                .unwrap()
                .write_to(&mut cursor, ImageFormat::Png)
                .map_err(|_| "Failed to write image".to_string())?;
        }
        Format::Jpeg |
        Format::Auto => {
            let mut encoder = JpegEncoder::new_with_quality(&mut cursor, map_quality(options.quality.unwrap_or(Quality::Medium)));
            encoder.encode_image(&img_for_processing.out_img.unwrap())?
        }
    }


    Ok(buf)
}

#[cfg(test)]
mod test {
    use crate::image_service::{resize_service, ImageTransformOptions};
    use rstest::rstest;
    use std::fs;

    #[rstest]
    #[case(300)]
    #[case(134)]
    #[case(999)]
    #[case(6143)]
    #[case(1)]
    fn resizes_width(#[case] width: u32) {
        let test_img: Vec<u8> = fs::read("../../test/assets/test_img.png").unwrap();
        let result = resize_service(
            test_img,
            ImageTransformOptions {
                width: Some(width),
                height: Some(300),
                fit: None,
                bg_color: None,
            },
        );
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
        let result = resize_service(
            test_img,
            ImageTransformOptions {
                width: Some(300),
                height: Some(height),
                fit: None,
                bg_color: None,
            },
        );
        let result_img = image::load_from_memory(&result.unwrap());

        assert_eq!(result_img.unwrap().height(), height)
    }
    #[test]
    fn preserves_width_when_none() {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize_service(
            test_img,
            ImageTransformOptions {
                width: None,
                height: Some(300),
                fit: None,
                bg_color: None,
            },
        );
        let result_img = image::load_from_memory(&result.unwrap());
        assert_eq!(result_img.unwrap().width(), 100)
    }

    #[test]
    fn preserves_height_when_none() {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize_service(
            test_img,
            ImageTransformOptions {
                width: Some(300),
                height: None,
                fit: None,
                bg_color: None,
            },
        );
        let result_img = image::load_from_memory(&result.unwrap());
        assert_eq!(result_img.unwrap().height(), 100)
    }
}

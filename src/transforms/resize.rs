use image::{DynamicImage, GenericImageView, Pixel, Rgb, RgbImage};

pub fn resize_with_pad(img_instance: &DynamicImage, resize_width: u32, resize_height: u32, bg_color: Option<Rgb<u8>>) -> DynamicImage {
    let mut img_canvas = RgbImage::new(resize_width, resize_height);

    for x in 0..resize_width {
        for y in 0..resize_height {
            img_canvas.put_pixel(x, y, bg_color.unwrap_or(Rgb([255u8, 0u8, 0u8])));
        }
    }

    let img_overlay = img_instance.resize(resize_width, resize_height, image::imageops::FilterType::Lanczos3);
    let (overlay_width, overlay_height) = img_overlay.dimensions();
    let offset_x = (img_canvas.width() - overlay_width) / 2;
    let offset_y = (img_canvas.height() - overlay_height) / 2;

    for x in 0..overlay_width {
        for y in 0..overlay_height {
            if x + offset_x < overlay_width + offset_x && y + offset_y < overlay_height + offset_y {
                let overlay_pixel = img_overlay.get_pixel(x, y);

                img_canvas.put_pixel(x + offset_x, y + offset_y, overlay_pixel.to_rgb());
            }
        }
    }

    DynamicImage::from(img_canvas)
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use std::{fmt, fs};
    use image::{DynamicImage, Pixel, Rgb};
    use rstest::rstest;
    use crate::transforms::resize::resize_with_pad;

    #[derive(Debug)]
    struct TestError {
        details: String,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl Error for TestError {}

    impl TestError {
        fn new(msg: &str) -> TestError {
            TestError {
                details: msg.to_string()
            }
        }
    }


    #[rstest]
    #[case(100, 300)]
    #[case(100, 400)]
    #[case(140, 900)]
    fn preserves_aspect_ratio_when_fit_is_pad(#[case] width: u32, #[case] height: u32) -> Result<(), Box<dyn Error>> {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let test_dyn_img: DynamicImage = image::load_from_memory(&test_img).unwrap();
        let result = resize_with_pad(&test_dyn_img, width, height, None);

        let resized_img_pad_size: u32 = result.height() - test_dyn_img.height();
        let resized_img_size: u32 = result.height() - resized_img_pad_size;
        let result_aspect_ratio: u32 = result.clone().width() / resized_img_size;

        if result_aspect_ratio == 1 {
            Ok(())
        } else { Err(Box::new(TestError::new(format!("Aspect ratio was not equal to 1, received {}", result_aspect_ratio).as_str()))) }
    }

    #[test]
    fn adds_bg_color_to_height_when_fit_is_pad() {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize_with_pad(&image::load_from_memory(&test_img).unwrap(), 100, 300,None);

        let img_top = result.crop_imm(0, 0, 100, 100);
        let img_bottom = result.crop_imm(0, 200, 100, 100);

        assert!(is_image_red_only(&img_top));
        assert!(is_image_red_only(&img_bottom));
    }

    #[test]
    fn adds_bg_color_to_width_when_fit_is_pad() {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let result = resize_with_pad(&image::load_from_memory(&test_img).unwrap(), 300, 100, None);

        let img_top = result.crop_imm(0, 0, 100, 100);
        let img_bottom = result.crop_imm(200, 0, 100, 100);

        assert!(is_image_red_only(&img_top));
        assert!(is_image_red_only(&img_bottom));
    }

    #[test]
    fn uses_bgcolor_when_available() {
        let test_img: Vec<u8> = fs::read("test/assets/test_img.png").unwrap();
        let bg_color: Rgb<u8> = Rgb([0, 225, 0]);
        let result = resize_with_pad(&image::load_from_memory(&test_img).unwrap(), 300, 100, Some(bg_color));

        let img_top = result.crop_imm(0, 0, 100, 100);
        let img_bottom = result.crop_imm(200, 0, 100, 100);

        assert!(is_image_mostly_color(&img_top, bg_color));
        assert!(is_image_mostly_color(&img_bottom, bg_color));
    }

    fn is_image_red_only(img: &DynamicImage) -> bool {
        let rgb_image = img.to_rgb8();

        for pixel in rgb_image.pixels() {
            let channels = pixel.0;
            let green = channels[1];
            let blue = channels[2];

            if green != 0 || blue != 0 {
                return false;
            }
        }
        true
    }
    fn is_image_mostly_color(img: &DynamicImage, color: Rgb<u8>) -> bool {
        let rgb_image = img.to_rgb8();
        let target_colors = color.channels();

        let mut counter_true: f32 = 0.0;
        let mut counter_false: f32 = 0.0;

        for pixel in rgb_image.pixels() {
            let channels = pixel.0;
            let red = channels[0];
            let green = channels[1];
            let blue = channels[2];

            if red == target_colors[0] && green == target_colors[1] && blue == target_colors[2] {
                counter_true += 1.0;
            } else {
                counter_false += 1.0;
            }
        }
        let ratio = if counter_true > 0.0 { (counter_true + counter_false) / counter_true}  else { 0.0 };
        println!("ratio is {}", ratio);
        ratio > 0.9
    }
}
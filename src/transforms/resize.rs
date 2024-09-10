use image::{DynamicImage, GenericImageView, Pixel, Rgb, RgbImage};

pub fn resize_with_pad(img_instance: &DynamicImage, resize_width: u32, resize_height: u32) -> DynamicImage {
    let mut img_canvas = RgbImage::new(resize_width, resize_height);

    for x in 0..resize_width {
        for y in 0..resize_height {
            img_canvas.put_pixel(x, y, Rgb([255, 0, 0]));
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
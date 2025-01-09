
use image::{GenericImageView, Pixel};
use crate::transform_processor::{into_next, TransformProcessor, ProcessableImage};

#[derive(Default)]
pub struct Resize {
    next: Option<Box<dyn TransformProcessor>>
}

impl Resize {
    pub fn new(next: impl TransformProcessor + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}
impl TransformProcessor for Resize {
    fn handle(&mut self, processable_image: &mut ProcessableImage) {
        if !processable_image.process_record.is_image_resized {
            let mut img_canvas = processable_image.out_img.clone().unwrap();
            let img_overlay = processable_image.src_img.resize(processable_image.process_options.resize.w, processable_image.process_options.resize.h, image::imageops::FilterType::Lanczos3);
            let (overlay_width, overlay_height) = img_overlay.dimensions();
            let offset_x = (processable_image.out_img.clone().unwrap().width() - overlay_width) / 2;
            let offset_y = (processable_image.out_img.clone().unwrap().height() - overlay_height) / 2;

            for x in 0..overlay_width {
                for y in 0..overlay_height {
                    if x + offset_x < overlay_width + offset_x && y + offset_y < overlay_height + offset_y {
                        let overlay_pixel = img_overlay.get_pixel(x, y);

                        img_canvas.put_pixel(x + offset_x, y + offset_y, overlay_pixel.to_rgb());
                    }
                }
            }
            processable_image.out_img = Some(img_canvas);
            processable_image.process_record.is_image_resized = true;

        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn TransformProcessor>> {
        &mut self.next
    }
}
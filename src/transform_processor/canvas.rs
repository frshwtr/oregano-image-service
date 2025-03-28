use crate::image_service::Fit;
use crate::transform_processor::{into_next, ProcessableImage, TransformProcessor};
use image::{ImageBuffer, Rgb};

#[derive(Default)]
pub struct Canvas {
    next: Option<Box<dyn TransformProcessor>>,
}

impl Canvas {
    pub fn new(next: impl TransformProcessor + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}
impl TransformProcessor for Canvas {
    fn handle(&mut self, processable_image: &mut ProcessableImage) {
        if !processable_image.process_record.is_canvas_processed {
            if processable_image.process_options.resize.mode == Fit::Pad {
                processable_image.out_img = Some(ImageBuffer::from_pixel(
                    processable_image.process_options.resize.w,
                    processable_image.process_options.resize.h,
                    processable_image.process_options.bg_color.unwrap_or(Rgb([0,0,0]))));
            }
            processable_image.process_record.is_bg_color_applied = true;
            processable_image.process_record.is_canvas_processed = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn TransformProcessor>> {
        &mut self.next
    }
}


use image::{RgbImage};
use crate::transform_processor::{ TransformProcessor, ProcessableImage, into_next};

#[derive(Default)]
pub struct Canvas {
    next: Option<Box<dyn TransformProcessor>>
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
                    processable_image.out_img = Some(RgbImage::new(processable_image.process_options.resize.w, processable_image.process_options.resize.h));
                    processable_image.process_record.is_canvas_processed = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn TransformProcessor>> {
        &mut self.next
    }
}
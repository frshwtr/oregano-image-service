use image::RgbImage;
use crate::image_service::Fit;
use super::transform_processor::{TransformProcessor, ProcessableImage};

#[derive(Default)]
pub struct Canvas {
    next: Option<Box<dyn TransformProcessor>>
}

impl TransformProcessor for Canvas {
    fn handle(&mut self, processable_image: &mut ProcessableImage) {
        if !(processable_image.process_record.is_canvas_processed && processable_image.process_options.resize.mode == Fit::Pad) {
            processable_image.out_img = Some(RgbImage::new(processable_image.process_options.resize.w, processable_image.process_options.resize.w))
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn TransformProcessor>> {
        &mut self.next
    }
}
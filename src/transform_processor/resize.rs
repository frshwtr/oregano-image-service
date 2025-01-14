
use image::{GenericImageView, Pixel};
use crate::image_service::Fit;
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
            processable_image.out_img = match processable_image.process_options.resize.mode {
                Fit::Pad => {
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
                    Some(img_canvas)
                },
                Fit::ScaleDown => {
                     Some(processable_image.src_img.thumbnail(processable_image.process_options.resize.w, processable_image.process_options.resize.h).to_rgb8())
                }
                _ => {
                    Some(processable_image.src_img.resize_exact(processable_image.process_options.resize.w, processable_image.process_options.resize.h, image::imageops::FilterType::Lanczos3).to_rgb8())
                }
            };

            processable_image.process_record.is_image_resized = true;

        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn TransformProcessor>> {
        &mut self.next
    }
}
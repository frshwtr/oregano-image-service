use image::{DynamicImage, Rgb};
use crate::image_service::Fit;

pub struct ProcessableImage {
    pub src_img: DynamicImage,
    pub out_img: Option<DynamicImage>,
    pub process_record: ProcessRecord,
    pub process_options: ProcessOptions
}

pub struct ProcessRecord  {
    pub is_canvas_processed: bool,
    pub is_image_resized: bool,
    pub is_bg_color_applied: bool,
}

struct ProcessOptions {
    pub resize: ResizeOptions,
    pub bg_color: Option<Rgb<u8>>
}

struct ResizeOptions {
    pub w: u32,
    pub h: u32,
    pub mode: Fit
}
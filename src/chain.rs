use image::{DynamicImage, ImageBuffer, Rgb};
use crate::image_service::Fit;

pub struct ProcessableImage {
    pub src_img: DynamicImage,
    pub out_img: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    pub process_record: ProcessRecord,
    pub process_options: ProcessOptions
}

pub struct ProcessRecord  {
    pub is_canvas_processed: bool,
    pub is_image_resized: bool,
    pub is_bg_color_applied: bool,
}

pub struct ProcessOptions {
    pub resize: ResizeOptions,
    pub  bg_color: Option<Rgb<u8>>
}

pub struct ResizeOptions {
    pub w: u32,
    pub h: u32,
    pub mode: Fit,
    pub dpr: u8
}
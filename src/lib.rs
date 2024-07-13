use wasm_bindgen::prelude::*;
use image::io::Reader as ImageReader;
use image::ImageFormat;
use std::io::Cursor;
use console_error_panic_hook;

pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn decode(bytes: Vec<u8>, mime: &str) -> Vec<u8> {
    console_error_panic_hook::set_once();

    let format = match mime {
        "image/jpeg" => ImageFormat::Jpeg,
        "image/png" => ImageFormat::Png,
        "image/gif" => ImageFormat::Gif,
        "image/bmp" => ImageFormat::Bmp,
        "image/tiff" => ImageFormat::Tiff,
        "image/webp" => ImageFormat::WebP,
        "image/x-icon" => ImageFormat::Ico,
        _ => ImageFormat::Png,
    };

    let mut reader = ImageReader::new(Cursor::new(bytes));
    reader.set_format(format);

    let img_buf = 
    reader
    .decode()
    .unwrap()
    .into_rgba8();
    let width = img_buf.width();
    let mut data = img_buf.into_raw();
    // add width bytes to end (a u32 as four u8 values):
    data.push((width >> 24) as u8);
    data.push((width >> 16) as u8);
    data.push((width >> 8 ) as u8);
    data.push((width >> 0 ) as u8);
    data
}
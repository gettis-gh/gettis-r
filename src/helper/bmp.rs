use image::{ImageBuffer, Rgba};
use std::path::Path;
use std::io;

pub fn save_bmp(path: &str, width: u32, height: u32, rgba_buffer: &[u8]) -> io::Result<()> {
    assert!(rgba_buffer.len() == (width * height * 4) as usize);

    let img_buffer: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(width, height, rgba_buffer.to_vec())
        .expect("Buffer size mismatch");

    // Nota: guarda en formato BMP directamente
    img_buffer.save_with_format(Path::new(path), image::ImageFormat::Bmp)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
use image::{DynamicImage, ImageBuffer};
use rxing::common::BitMatrix;

pub fn transparant_background (bit_matrix: &BitMatrix, width: u32, height: u32)  {
    // Variant 1: Transparent background
    let mut transparent_buffer = ImageBuffer::new(width, height);

    for (x, y) in (0..width).flat_map(|x| (0..height).map(move |y| (x, y))) {
        let color = if bit_matrix.get(x, y) {
            image::Rgba([0u8, 0u8, 0u8, 255u8])  // Black, fully opaque
        } else {
            image::Rgba([0u8, 0u8, 0u8, 0u8])    // Transparent
        };
        transparent_buffer.put_pixel(x, y, color);
    }
    let transparent_image = DynamicImage::ImageRgba8(transparent_buffer);
    transparent_image.save("barcode_transparent.png").expect("Failed to save barcode_transparent.png");
}

pub fn white_background (bit_matrix: &BitMatrix, width: u32, height: u32)  {
    // Variant 2: White background
    let mut white_buffer = ImageBuffer::new(width, height);

    for (x, y) in (0..width).flat_map(|x| (0..height).map(move |y| (x, y))) {
        let color = if bit_matrix.get(x, y) {
            image::Rgba([0u8, 0u8, 0u8, 255u8])  // Black
        } else {
            image::Rgba([255u8, 255u8, 255u8, 255u8])  // White
        };
        white_buffer.put_pixel(x, y, color);
    }

    let white_bg_image = DynamicImage::ImageRgba8(white_buffer);
    white_bg_image.save("barcode_white_bg.png").expect("Failed to save barcode_white_bg.png");
}
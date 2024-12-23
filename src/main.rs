mod preprocess;
mod decode;

use std::io::Write;
use preprocess::preprocess_image;

use image::{DynamicImage, ImageBuffer};
use rxing::{
    BarcodeFormat, EncodeHintType, EncodeHintValue, EncodingHintDictionary


    ,
    MultiFormatWriter,
    Writer,
};
use crate::decode::try_decode_barcode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load and preprocess the image
    let original_image = image::open("imgs/working barcode.png")?;
    let processed_image = preprocess_image(original_image);

    // Save preprocessed image for debugging
    processed_image.save("preprocessed.png")?;

    // Try to decode the preprocessed image
    let decoded_result = try_decode_barcode(&processed_image)?;

    println!("Original barcode content: {}", decoded_result);

    let mut output_file = std::fs::File::create("decoded_content.txt")?;
    output_file.write_all(decoded_result.as_bytes())?;

    // Generate matching barcode
    let writer = MultiFormatWriter::default();
    let mut encode_hints = EncodingHintDictionary::new();
    encode_hints.insert(
        EncodeHintType::MARGIN,
        EncodeHintValue::Margin("1".to_string())
    );

    let bit_matrix = writer.encode_with_hints(
        &decoded_result,
        &BarcodeFormat::PDF_417,
        300,
        150,
        &encode_hints
    )?;

    // Convert to image with transparency and save
    let width = bit_matrix.width();
    let height = bit_matrix.height();
    let mut img_buffer = ImageBuffer::new(width, height);

    for (x, y) in (0..width).flat_map(|x| (0..height).map(move |y| (x, y))) {
        let color = if bit_matrix.get(x, y) {
            image::Rgba([0u8, 0u8, 0u8, 255u8])  // Black, fully opaque
        } else {
            image::Rgba([0u8, 0u8, 0u8, 0u8])    // Transparent
        };
        img_buffer.put_pixel(x, y, color);
    }

    let generated_image = DynamicImage::ImageRgba8(img_buffer);
    generated_image.save("generated_barcode.png")?;


    Ok(())
}

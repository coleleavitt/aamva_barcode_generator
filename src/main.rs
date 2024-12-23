mod preprocess;
mod decode;
mod generate_barcode;

use std::fmt::format;
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
use crate::generate_barcode::{transparant_background, white_background};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // Load and preprocess the image
    let original_image = image::open("imgs/img.png")?;
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
    let width = bit_matrix.width();
    let height = bit_matrix.height();


    // Variant 1: Transparent background
    transparant_background(&bit_matrix, width, height);

    // Variant 2: White background
    white_background(&bit_matrix, width, height);


    Ok(())
}

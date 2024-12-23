use image::DynamicImage;
use rxing::{
    common::HybridBinarizer, BarcodeFormat, BinaryBitmap,
    DecodeHintType,
    DecodeHintValue,
    DecodingHintDictionary,
    MultiFormatReader,
    Reader


    ,
};

pub fn try_decode_barcode(img: &DynamicImage) -> Result<String, Box<dyn std::error::Error>> {
    let luminance = rxing::BufferedImageLuminanceSource::new(img.clone());
    let binarizer = HybridBinarizer::new(luminance);
    let mut bitmap = BinaryBitmap::new(binarizer);

    let mut decode_hints = DecodingHintDictionary::new();

    // Add all possible formats to improve detection
    decode_hints.insert(
        DecodeHintType::POSSIBLE_FORMATS,
        DecodeHintValue::PossibleFormats(vec![
            BarcodeFormat::PDF_417,
            BarcodeFormat::CODE_128,
            BarcodeFormat::CODE_39,
            BarcodeFormat::DATA_MATRIX
        ].into_iter().collect())
    );

    // Enable all additional detection features
    decode_hints.insert(
        DecodeHintType::TRY_HARDER,
        DecodeHintValue::TryHarder(true)
    );

    decode_hints.insert(
        DecodeHintType::PURE_BARCODE,
        DecodeHintValue::PureBarcode(true)
    );

    decode_hints.insert(
        DecodeHintType::CHARACTER_SET,
        DecodeHintValue::CharacterSet("UTF-8".to_string())
    );

    let mut reader = MultiFormatReader::default();
    match reader.decode_with_hints(&mut bitmap, &decode_hints) {
        Ok(result) => Ok(result.getText().to_string()),
        Err(_) => {
            // If detection fails, try with different binarizer
            let luminance = rxing::BufferedImageLuminanceSource::new(img.clone());
            let global_binarizer = rxing::common::GlobalHistogramBinarizer::new(luminance);
            let mut bitmap = BinaryBitmap::new(global_binarizer);
            let result = reader.decode_with_hints(&mut bitmap, &decode_hints)?;
            Ok(result.getText().to_string())
        }
    }
}

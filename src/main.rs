use rxing::{BarcodeFormat, EncodeHintType, EncodeHintValue, MultiFormatWriter, Writer};
use std::collections::HashMap;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let aamva_data = "@\n\
    ANSI 636014090102DL00410262ZC00240024\n\
    DL\n\
    DAQY6297168\n\
    DCSHOLLIDAY\n\
    DDEN\n\
    DACCLAIRE\n\
    DDFN\n\
    DADVIRGINIA\n\
    DDGN\n\
    DBD09222022\n\
    DBB12201995\n\
    DBA12202026\n\
    DBC2\n\
    DAU070 IN\n\
    DAYGRN\n\
    DAG15216 S AVALON BLVD\n\
    DAICOMPTON\n\
    DAJCA\n\
    DAK902200000\n\
    DCF09/22/2022508B4/BBFD/26\n\
    DCGUSA\n\
    DAW145\n\
    DAZBRO\n\
    DCK22264Y62971680901\n\
    DDAF\n\
    DDB08292017\n\
    ZC\n\
    ZCAGRN\n\
    ZCBBRN\n\
    ZCC\n\
    ZCD\n"
        .to_string();

    let mut hints = HashMap::new();
    hints.insert(
        EncodeHintType::PDF417_COMPACT,
        EncodeHintValue::Pdf417Compact(false.to_string()),
    );
    hints.insert(
        EncodeHintType::ERROR_CORRECTION,
        EncodeHintValue::ErrorCorrection("5".to_string()),
    );
    hints.insert(
        EncodeHintType::MARGIN,
        EncodeHintValue::Margin("10".to_string()),
    );

    let writer = MultiFormatWriter;
    let matrix =
        writer.encode_with_hints(&aamva_data, &BarcodeFormat::PDF_417, 400, 200, &hints)?;

    let mut pixels = Vec::new();
    for y in 0..matrix.height() {
        for x in 0..matrix.width() {
            let color = if matrix.get(x, y) {
                [0u8, 0u8, 0u8]
            } else {
                [255u8, 255u8, 255u8]
            };
            pixels.extend_from_slice(&color);
        }
    }

    let mut file = File::create("aamva_barcode.png")?;
    let mut encoder = png::Encoder::new(&mut file, matrix.width(), matrix.height());
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&pixels)?;

    Ok(())
}

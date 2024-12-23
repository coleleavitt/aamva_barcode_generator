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
    let original_image = image::open("imgs/img_1.png")?;
    let processed_image = preprocess_image(original_image);

    // Save preprocessed image for debugging
    processed_image.save("preprocessed.png")?;

    // Try to decode the preprocessed image
    // let decoded_result = try_decode_barcode(&processed_image)?;
    // let decoded_result = std::fs::read_to_string("decoded_content.txt")?;

    // Personal Information Elements
    let customer_family_name = "HOLLIDAY";
    let family_name_truncated = "N";
    let driver_first_name = "CLAIRE";
    let first_name_truncated = "N";
    let driver_middle_name = "VIRGINIA";
    let middle_name_truncated = "N";


    // Address information
    let address_street_1 = "15216 S Avalon Blvd";
    let address_city = "COMPTON";
    let address_jurisdiction_code = "CA";
    let address_postcal_code = "902200000";
    let country_identification = "USA";

    // Document Information
    let document_discrinminator = "09/22/2022508B4/BBFD/26";
    let document_issue_date = "09222022"; // MMDDYYYY
    let document_expiration_date = "12202026"; // MMDDYYYY

    // Physical Description
    let physical_description_weight = "145"; // Rounded to nearest pound
    let hair_color = "BRO"; // Standard three-letter code for brown
    let date_of_birth = "12201995"; // Format: MMDDYYYY
    let physical_decription_sex = "2"; // 2 = Female, 1 = Male
    let physical_description_height = "070"; // Format: XXX IN (total inches)
    let physical_decription_eye_color = "GRN"; // Standard three-letter code for green

    // Administrative Data
    let inventory_control_number = "22264Y62971680901";
    let compliance_type = "F";
    let card_revision_date = "08292017";

    // Optional ZC Subfile Fields
    let redundant_eye_color_code = "GRN";
    let alternative_hair_color_code = "BRN";



    let decoded_result = format!("@

ANSI 636014090102ID00410262ZC03030024IDDAQY6297168
DCS{customer_family_name}
DDE{family_name_truncated}
DAC{driver_first_name}
DDF{first_name_truncated}
DAD{driver_middle_name}
DDG{middle_name_truncated}
DBD{document_issue_date}
DBB{date_of_birth}
DBA{document_expiration_date}
DBC{physical_decription_sex}
DAU{physical_description_height} IN
DAY{physical_decription_eye_color}
DAG{address_street_1}
DAI{address_city}
DAJ{address_jurisdiction_code}
DAK{address_postcal_code}
DCF{document_discrinminator}
DCG{country_identification}
DAW{physical_description_weight}
DAZ{hair_color}
DCK{inventory_control_number}
DDA{compliance_type}
DDB{card_revision_date}
ZCZCA{redundant_eye_color_code}
ZCB{alternative_hair_color_code}
ZCC
ZCD
");
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

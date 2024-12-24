mod preprocess;
mod decode;
mod generate_barcode;
mod structure;
use structure::data::DriversLicense;

use std::io::Write;

use crate::generate_barcode::{transparant_background, white_background};
use rxing::{
    BarcodeFormat, EncodeHintType, EncodeHintValue, EncodingHintDictionary,
    MultiFormatWriter,
    Writer,
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let license = DriversLicense {
        drivers_license_number: "D1175907".to_string(),
        first_name: String::from("Herman").to_uppercase(),
        last_name: String::from("Huang").to_uppercase(),
        address: String::from("839 Heaven Ct").to_uppercase(),
        city: String::from("Hayward").to_uppercase(),
        state: String::from("CA").to_uppercase(),
        height: String::from("071 IN"),
        eye_color: String::from("BRO"),
        hair_color: "BRO".to_string(),
        dob: String::from("12011989"), // MMDDYYYY
        issue_date: String::from("06072022"), // MMDDYYYY
        expiry_date: String::from("12012027"), // MMDDYYYY
        postal_code: String::from("945444132  "),
        alternative_hair_color_encoding: String::from("BRN"),
        redundant_encoding_eye_color: String::from("BRN"),
        weight: "220".to_string(),
        middle_name: "".to_string(),
        organ_donor: "".to_string(),
        sex: "1".to_string(),
        family_name_truncated: "N".to_string(),
        endorsements: "NONE".to_string(),
        middle_name_truncated: "N".to_string(),
        first_name_truncated: "N".to_string(),
        vehicle_class: "C".to_string(),
        document_discriminator: "06/07/2022508B4/BBFD/27".to_string(), // {Issue_Date}{Sequence}{Office_Code}/{Security_Code}/{Version AKA exp date year not century}
        inventory_control_number: "22157D11759070901".to_string(),
    };

    let decoded_result = license.to_aamva_string();
    println!("Original barcode content: {}", decoded_result);


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

    transparant_background(&bit_matrix, width, height);
    white_background(&bit_matrix, width, height);

    Ok(())
}
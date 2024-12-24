// Define the struct for driver's license data
#[derive(Debug)]
pub struct DriversLicense {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) address: String,
    pub(crate) city: String,
    pub(crate) state: String,
    pub(crate) height: String,
    pub(crate) eye_color: String,
    pub(crate) dob: String,
    pub(crate) issue_date: String,
    pub(crate) expiry_date: String,
    pub(crate) postal_code: String,
    pub(crate) alternative_hair_color_encoding: String,
    pub(crate) redundant_encoding_eye_color: String,
    pub(crate) weight: String,
    pub(crate) middle_name: String,
    pub(crate) organ_donor: String,
    pub(crate) sex: String,
    pub(crate) family_name_truncated: String,
    pub(crate) endorsements: String,
    pub(crate) middle_name_truncated: String,
    pub(crate) first_name_truncated: String,
    pub(crate) vehicle_class: String,
    pub(crate) document_discriminator: String,
    pub(crate) drivers_license_number: String,
    pub(crate) hair_color: String,
    pub(crate) inventory_control_number: String,
}

impl DriversLicense {
    pub fn to_aamva_string(&self) -> String {
        format!(
            "@\n\nANSI 636014090102DL00410265ZC03060024DLDAQ{}\n\
            DCS{}\n\
            DDE{}\n\
            DAC{}\n\
            DDF{}\n\
            DAD{}\n\
            DDG{}\n\
            DCA{}\n\
            DCB01\n\
            DCD{}\n\
            DBD{}\n\
            DBB{}\n\
            DBA{}\n\
            DBC{}\n\
            DAU{}\n\
            DAY{}\n\
            DAG{}\n\
            DAI{}\n\
            DAJ{}\n\
            DAK{}\n\
            DCF{}\n\
            DCGUSA\n\
            DAW{}\n\
            DAZ{}\n\
            DCK{}\n\
            DDAF\n\
            DDB08292017\n\
            DDK{}\n\
            ZCZCA{}\n\
            ZCB{}\n\
            ZCC\n\
            ZCD",
            self.drivers_license_number,
            self.last_name,
            self.family_name_truncated,
            self.first_name,
            self.first_name_truncated,
            self.middle_name,
            self.middle_name_truncated,
            self.vehicle_class,
            self.endorsements,
            self.issue_date,
            self.dob,
            self.expiry_date,
            self.sex,
            self.height,
            self.eye_color,
            self.address,
            self.city,
            self.state,
            self.postal_code,
            self.document_discriminator,
            self.weight,
            self.hair_color,
            self.inventory_control_number,
            self.organ_donor,
            self.redundant_encoding_eye_color,
            self.alternative_hair_color_encoding
        )
    }
}

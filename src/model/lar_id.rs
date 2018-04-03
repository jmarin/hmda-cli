use std::fmt;

pub struct LarIdentifier {
    pub id: u8,
    pub lei: String,
    pub nmls_identifier: String,
}

impl LarIdentifier {
pub fn sample_lar_identifier() -> LarIdentifier {
  LarIdentifier {
    id: 2,
    lei: "10Bx939c5543TqA1144M".to_string(),
    nmls_identifier: "12345".to_string(),
  }    
}
}

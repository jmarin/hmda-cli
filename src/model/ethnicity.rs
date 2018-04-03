pub struct Ethnicity {
    pub ethnicity1: i8,
    pub ethnicity2: i8,
    pub ethnicity3: i8,
    pub ethnicity4: i8,
    pub ethnicity5: i8,
    pub other_hispanic_latino: String,
    pub ethnicity_observed: u8,
}

impl Ethnicity {
  pub fn sample_ethnicity() -> Ethnicity {
    Ethnicity {
      ethnicity1: 1,
      ethnicity2: -1,
      ethnicity3: -1,
      ethnicity4: -1,
      ethnicity5: -1,
      other_hispanic_latino: "".to_string(),
      ethnicity_observed: 3, 
    }
  }    
}


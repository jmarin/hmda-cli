pub struct Sex {
    pub sex: u8,
    pub sex_observed: u8,
}

impl Sex {
  pub fn sample_sex() -> Sex {
    Sex {
      sex: 1,
      sex_observed: 3,
    }    
  }
}

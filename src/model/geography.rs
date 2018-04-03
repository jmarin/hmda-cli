use std::fmt;

pub struct Geography {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub county: String,
    pub tract: String,
}

impl Geography {
  pub fn sample_geography() -> Geography {
    Geography {
      street: "123 Main St".to_string(),
      city: "Beverly Hills".to_string(),
      state: "CA".to_string(),
      zip_code: "90210".to_string(),
      county: "06037".to_string(),
      tract: "06037264000".to_string(),
    }    
  }
}

impl fmt::Display for Geography {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}|{}|{}|{}|{}|{}",
      self.street, self.city, self.state, self.zip_code, self.county, self.tract
    )      
  }    
}

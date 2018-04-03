use std::fmt;

pub struct AutomatedUnderwritingSystem {
    pub aus1: i8,
    pub aus2: i8,
    pub aus3: i8,
    pub aus4: i8,
    pub aus5: i8,
    pub other_aus: String,
}

impl AutomatedUnderwritingSystem {
  pub fn sample_aus() -> AutomatedUnderwritingSystem {
      AutomatedUnderwritingSystem {
          aus1: 1,
          aus2: -1,
          aus3: -1,
          aus4: -1,
          aus5: -1,
          other_aus: "".to_string(), 
      }
  }
}

impl fmt::Display for AutomatedUnderwritingSystem {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}|{}|{}|{}|{}|{}",
      self.aus1,self.aus2,self.aus3,self.aus4,self.aus5,self.other_aus
    )    
  }    
}

use std::fmt;

pub struct AutomatedUnderwritingSystemResult {
    pub aus_result1: i8,
    pub aus_result2: i8,
    pub aus_result3: i8,
    pub aus_result4: i8,
    pub aus_result5: i8,
    pub other_aus_result: String,
}

impl AutomatedUnderwritingSystemResult {
  pub fn sample_aus_result() -> AutomatedUnderwritingSystemResult {
    AutomatedUnderwritingSystemResult {
      aus_result1: 1,
      aus_result2: 1,
      aus_result3: 1,
      aus_result4: 1,
      aus_result5: 1,
      other_aus_result: "".to_string(),
    }    
  }
}

impl fmt::Display for AutomatedUnderwritingSystemResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     write!(
       f,
       "{}|{}|{}|{}|{}|{}",
       self.aus_result1,
       self.aus_result2,
       self.aus_result3,
       self.aus_result4,
       self.aus_result5,
       self.other_aus_result
     ) 
  }
}

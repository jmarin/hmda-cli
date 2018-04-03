pub struct Denial {
    pub denial_reason_1: i8,
    pub denial_reason_2: i8,
    pub denial_reason_3: i8,
    pub denial_reason_4: i8,
    pub other_denial_reason: String,
}

impl Denial {
  pub fn sample_denial() -> Denial {
    Denial {
      denial_reason_1: -1,
      denial_reason_2: -1,
      denial_reason_3: -1,
      denial_reason_4: -1,
      other_denial_reason: "".to_string(),
    }    
  }    
}

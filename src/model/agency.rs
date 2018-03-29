pub enum Agency {
  OCC,
  FRS,
  FDIC,
  NCUA,
  HUD,
  CFPB,
  Undetermined
}

impl Agency {
  pub fn value_of(&self) -> i32 {
    match *self {
      Agency::OCC => 1,
      Agency::FRS => 2,
      Agency::FDIC => 3,
      Agency::NCUA => 5,
      Agency::HUD => 7,
      Agency::CFPB => 9,
      Agency::Undetermined => -1
    }
  } 
}

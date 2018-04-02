use std::fmt;

pub enum Agency {
    OCC,
    FRS,
    FDIC,
    NCUA,
    HUD,
    CFPB,
    Undetermined,
}

impl Agency {
    pub fn value_of(&self) -> i8 {
        match *self {
            Agency::OCC => 1,
            Agency::FRS => 2,
            Agency::FDIC => 3,
            Agency::NCUA => 5,
            Agency::HUD => 7,
            Agency::CFPB => 9,
            Agency::Undetermined => -1,
        }
    }
}

impl fmt::Display for Agency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Agency::value_of(&self))
    }
}

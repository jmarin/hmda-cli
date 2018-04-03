pub struct Race {
    pub race1: i8,
    pub race2: i8,
    pub race3: i8,
    pub race4: i8,
    pub race5: i8,
    pub other_native: String,
    pub other_asian: String,
    pub other_pacific_islander: String,
    pub race_observed: u8,
}

impl Race {
    pub fn sample_race() -> Race {
        Race {
            race1: 5,
            race2: 7,
            race3: 7,
            race4: 7,
            race5: 7,
            other_native: "".to_string(),
            other_asian: "".to_string(),
            other_pacific_islander: "".to_string(),
            race_observed: 3,
        }
    }
}

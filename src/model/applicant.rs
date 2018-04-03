use model::ethnicity::Ethnicity;
use model::race::Race;
use model::sex::Sex;

pub struct Applicant {
    ethnicity: Ethnicity,
    race: Race,
    sex: Sex,
    age: u8,
    credit_score: u8,
    credit_score_type: u8,
    other_credit_score_model: u8,
}

use model::ethnicity::Ethnicity;
use model::race::Race;
use model::sex::Sex;

pub struct Applicant {
    pub ethnicity: Ethnicity,
    pub race: Race,
    pub sex: Sex,
    pub age: u8,
    pub credit_score: u8,
    pub credit_score_type: u8,
    pub other_credit_score_model: u8,
}

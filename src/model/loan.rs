pub struct Loan {
    pub uli: String,
    pub application_date: String,
    pub loan_type: i8,
    pub loan_purpose: i8,
    pub construction_method: i8,
    pub occupancy: i8,
    pub amount: f64,
    pub loan_term: String,
    pub rate_spread: String,
    pub interest_rate: String,
    pub prepayment_penalty_term: String,
    pub debt_to_income_ratio: String,
    pub loan_to_value_ratio: String,
    pub introductory_rate_period: String,
}

impl Loan {
pub fn sample_loan() -> Loan {
  Loan {
    uli: "10Bx939c5543TqA1144M999143X38".to_string(),
    application_date: "20180721".to_string(),
    loan_type: 1,
    loan_purpose: 1,
    construction_method: 1,
    occupancy: 1,
    amount: 110500.00,
    loan_term: "360".to_string(),
    rate_spread: "0.428".to_string(),
    interest_rate: "4.125".to_string(),
    prepayment_penalty_term: "NA".to_string(),
    debt_to_income_ratio: "42.95".to_string(),
    loan_to_value_ratio: "80.05".to_string(),
    introductory_rate_period: "NA".to_string(),
  }    
}
}

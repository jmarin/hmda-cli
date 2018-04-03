pub struct LoanDisclosure {
    pub total_loan_costs: String,
    pub total_points_and_fees: String,
    pub origination_charges: String,
    pub discount_points: String,
    pub lender_credits: String,
}

impl LoanDisclosure {
pub fn sample_loan_disclosure() -> LoanDisclosure {
  LoanDisclosure {
    total_loan_costs: "2399.04".to_string(),
    total_points_and_fees: "NA".to_string(),
    origination_charges: "NA".to_string(),
    discount_points: "NA".to_string(),
    lender_credits: "NA".to_string(),
  }    
}
}

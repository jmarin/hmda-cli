use std::fmt;

use model::lar_id::LarIdentifier;
use model::loan::Loan;
use model::lar_action::LarAction;
use model::geography::Geography;
use model::applicant::Applicant;
use model::loan_disclosure::LoanDisclosure;
use model::non_amortizing_features::NonAmortizingFeatures;
use model::property::Property;
use model::aus::AutomatedUnderwritingSystem;
use model::aus_result::AutomatedUnderwritingSystemResult;
use model::denial::Denial;

pub struct LoanApplicationRegister {
  lar_identifier: LarIdentifier,    
  loan: Loan,
  action: LarAction,
  geography: Geography,
  applicant: Applicant,
  co_applicant: Applicant,
  income: String,
  purchaser_type: u8,
  hoepa_status: u8,
  lien_status: u8,
  denial: Denial,
  loan_disclosure: LoanDisclosure,
  non_amortizing_features: NonAmortizingFeatures,
  property: Property,
  application_submission: u8,
  payable_to_institution: u8,
  aus: AutomatedUnderwritingSystem,
  aus_result: AutomatedUnderwritingSystemResult,
  reverse_mortgage: u8,
  line_of_credit: u8,
  business_or_commercial_purpose: u8
}

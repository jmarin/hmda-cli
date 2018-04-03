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
    pub lar_identifier: LarIdentifier,
    pub loan: Loan,
    pub action: LarAction,
    pub geography: Geography,
    pub applicant: Applicant,
    pub co_applicant: Applicant,
    pub income: String,
    pub purchaser_type: u8,
    pub hoepa_status: u8,
    pub lien_status: u8,
    pub denial: Denial,
    pub loan_disclosure: LoanDisclosure,
    pub non_amortizing_features: NonAmortizingFeatures,
    pub property: Property,
    pub application_submission: u8,
    pub payable_to_institution: u8,
    pub aus: AutomatedUnderwritingSystem,
    pub aus_result: AutomatedUnderwritingSystemResult,
    pub reverse_mortgage: u8,
    pub line_of_credit: u8,
    pub business_or_commercial_purpose: u8,
}

impl fmt::Display for LoanApplicationRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lar_identifier.id)
    }
}

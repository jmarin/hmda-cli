pub struct NonAmortizingFeatures {
    pub balloon_payment: i8,
    pub interest_only_payment: i8,
    pub negative_amortization: i8,
    pub other_non_amortizing_features: i8,
}

impl NonAmortizingFeatures {
pub fn sample_non_amortizing_features() -> NonAmortizingFeatures {
  NonAmortizingFeatures {
    balloon_payment: 1,
    interest_only_payment: 2,
    negative_amortization: 1,
    other_non_amortizing_features: 1,
  }    
}
}

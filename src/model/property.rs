pub struct Property {
    pub property_value: String,
    pub manufactured_home_secured_property: i8,
    pub manufactured_home_land_property_interest: i8,
    pub total_units: i32,
    pub multifamily_affordable_units: String,
}

impl Property {
  pub fn sample_property() -> Property {
    Property {
      property_value: "350500".to_string(),
      manufactured_home_secured_property: 1,
      manufactured_home_land_property_interest: 1,
      total_units: 5,
      multifamily_affordable_units: "NA".to_string(),
    }    
  }    
}

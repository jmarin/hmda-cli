use model::agency;

pub struct Address {
  street: String,
  city: String,
  state: String,
  zip_code: String,
}

pub struct Contact {
  name: String,
  phone: String,
  email: String,
  address: Address,
}

pub struct TransmittalSheet {
  id: i32,
  institution_name: String,
  year: i32,
  quarter: i32,
  contact: Contact,
  agency: agency::Agency,
  total_lines: i32,
  tax_id: String,
  lei: String,
}

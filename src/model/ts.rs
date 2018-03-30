use std::fmt;
use model::agency;

pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}",
            self.street, self.city, self.state, self.zip_code
        )
    }
}

pub struct Contact {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub address: Address,
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}",
            self.name, self.phone, self.email, self.address
        )
    }
}

pub struct TransmittalSheet {
    pub id: i32,
    pub institution_name: String,
    pub year: i32,
    pub quarter: i32,
    pub contact: Contact,
    pub agency: agency::Agency,
    pub total_lines: i32,
    pub tax_id: String,
    pub lei: String,
}

pub fn ts_sample(maybe_lei: Option<String>) -> TransmittalSheet {
    TransmittalSheet {
        id: 1,
        institution_name: String::from("Bank 0"),
        year: 2018,
        quarter: 4,
        contact: Contact {
            name: String::from("Jane Smith"),
            phone: String::from("111-111-1111"),
            email: String::from("jane.smith@bank0.com"),
            address: Address {
                street: String::from("123 Main St"),
                city: String::from("Washington"),
                state: String::from("DC"),
                zip_code: String::from("20001"),
            },
        },
        agency: agency::Agency::CFPB,
        total_lines: 1000,
        tax_id: String::from("99-999999"),
        lei: sample_lei(maybe_lei),
    }
}

impl fmt::Display for TransmittalSheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.id,
            self.institution_name,
            self.year,
            self.quarter,
            self.contact,
            self.agency,
            self.total_lines,
            self.tax_id,
            self.lei
        )
    }
}

fn sample_lei(maybe_lei: Option<String>) -> String {
    let default = "10Bx939c5543TqA1144M";
    match maybe_lei {
        None => String::from(default),
        Some(l) => String::from(l),
    }
}

extern crate hmda;

use hmda::*;

#[test]
fn ts_sample() {
    let address = model::ts::Address {
        street: String::from("123 Main St"),
        city: String::from("Washington"),
        state: String::from("DC"),
        zip_code: String::from("20001"),
    };
    assert_eq!(address.street, "123 Main St");
    assert_eq!(address.city, "Washington");
    assert_eq!(address.state, "DC");
    assert_eq!(address.zip_code, "20001");
    let contact = model::ts::Contact {
        name: String::from("Jane Smith"),
        phone: String::from("111-111-1111"),
        email: String::from("jane.smit@bank0.com"),
        address: address,
    };
    let agency = model::agency::Agency::CFPB;
    let ts = model::ts::TransmittalSheet {
        id: 1,
        institution_name: String::from("Bank 0"),
        year: 2018,
        quarter: 4,
        contact: contact,
        agency: agency,
        total_lines: 1000,
        tax_id: String::from("99-99999"),
        lei: String::from("10Bx939c5543TqA1144M"),
    };
    assert_eq!(ts.id, 1);
    assert_eq!(ts.to_string(), "1|Bank 0|2018|4|Jane Smith|111-111-1111|jane.smit@bank0.com|123 Main St|Washington|DC|20001|9|1000|99-99999|10Bx939c5543TqA1144M")
}

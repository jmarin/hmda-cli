extern crate hmda;

use hmda::*;

#[test]
fn value_of() {
    assert_eq!(model::agency::Agency::CFPB.value_of(), 9); 
}

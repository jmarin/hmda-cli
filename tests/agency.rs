extern crate hmda;

use hmda::*;

#[test]
fn test_value_of() {
    assert_eq!(model::agency::Agency::OCC.value_of(), 1);
    assert_eq!(model::agency::Agency::FRS.value_of(), 2);
    assert_eq!(model::agency::Agency::FDIC.value_of(), 3);
    assert_eq!(model::agency::Agency::NCUA.value_of(), 5);
    assert_eq!(model::agency::Agency::HUD.value_of(), 7);
    assert_eq!(model::agency::Agency::CFPB.value_of(), 9);
    assert_eq!(model::agency::Agency::Undetermined.value_of(), -1);
}

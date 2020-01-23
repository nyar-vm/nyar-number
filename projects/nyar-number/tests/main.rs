// use valkyrie_types::{testing::assert_type, ValkyrieID, ValkyrieInterface};

use nyar_number::{NyarNumber, NyarUnsigned, Zero};
use serde::Serialize;
use shredder::Gc;
use std::{collections::BTreeMap, ops::Div};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_primitive() {
    let i1 =
        NyarNumber::parse_integer("145175341267598143125685194855555555555555555666666666666666666666666666666666666666666425")
            .expect("");
    let a2 = NyarNumber::zero();
    let i2 = i1.clone();
    println!("{i1:#?}");
    println!("{i1}");
    println!("{i2:#?}");
    println!("{i2}");
    let i3 = i1.div(a2);
    println!("{i3:#?}");
    println!("{i3}");
}

#[test]
fn test_serde() {
    let mut unsigned = BTreeMap::default();
    unsigned.insert(0, NyarUnsigned::zero().get().clone());
    unsigned.insert(1, NyarUnsigned::one().get().clone());
    unsigned.insert(2, NyarUnsigned::from(u8::MAX));
    unsigned.insert(3, NyarUnsigned::from(u16::MAX));
    unsigned.insert(4, NyarUnsigned::from(u32::MAX));
    unsigned.insert(5, NyarUnsigned::from(u64::MAX));
    unsigned.insert(5, NyarUnsigned::from(u128::MAX));
    let json = serde_json::to_string_pretty(&unsigned).expect("!");
    println!("{}", json)
}

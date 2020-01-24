// use valkyrie_types::{testing::assert_type, ValkyrieID, ValkyrieInterface};

use num::One;
use nyar_number::{NyarInteger, NyarNumber, NyarUnsigned, Zero};
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
fn test_serde_unsigned() {
    let mut raw = BTreeMap::default();
    raw.insert(0, NyarUnsigned::zero().get().clone());
    raw.insert(1, NyarUnsigned::one().get().clone());
    raw.insert(2, NyarUnsigned::from(u8::MAX));
    raw.insert(3, NyarUnsigned::from(u16::MAX));
    raw.insert(4, NyarUnsigned::from(u32::MAX));
    raw.insert(5, NyarUnsigned::from(u64::MAX));
    raw.insert(5, NyarUnsigned::from(u128::MAX));
    let json = serde_json::to_string_pretty(&raw).expect("!");
    println!("{}", json);
    let map: BTreeMap<usize, NyarUnsigned> = serde_json::from_str(&json).expect("!");
    println!("{:#?}", map)
}
#[test]
fn test_serde_integer() {
    let mut raw = BTreeMap::default();
    raw.insert(0, NyarInteger::zero().clone());
    raw.insert(1, NyarInteger::one().clone());
    raw.insert(2, NyarInteger::from(i8::MIN));
    raw.insert(3, NyarInteger::from(i16::MAX));
    raw.insert(4, NyarInteger::from(i32::MIN));
    raw.insert(5, NyarInteger::from(i64::MAX));
    raw.insert(5, NyarInteger::from(i128::MIN));
    let json = serde_json::to_string_pretty(&raw).expect("!");
    println!("{}", json);
    let map: BTreeMap<usize, NyarInteger> = serde_json::from_str(&json).expect("!");
    println!("{:#?}", map)
}

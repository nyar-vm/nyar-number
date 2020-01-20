// use valkyrie_types::{testing::assert_type, ValkyrieID, ValkyrieInterface};

use nyar_number::{NyarInteger, NyarNumber, NyarUnsigned};
use std::{ops::Mul, str::FromStr};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_primitive() {
    let i1 =
        NyarNumber::parse_integer("145175341267598143125685194855555555555555555666666666666666666666666666666666666666666425")
            .expect("");
    let a2 = NyarNumber::from(2);
    let i2 = i1.clone();
    println!("{i1:#?}");
    println!("{i1}");
    println!("{i2:#?}");
    println!("{i2}");
    let i3 = i1.mul(i2);
    println!("{i3:#?}");
    println!("{i3}");
}

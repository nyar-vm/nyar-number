// use valkyrie_types::{testing::assert_type, ValkyrieID, ValkyrieInterface};

use nyar_number::NyarInteger;
use std::str::FromStr;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_primitive() {
    let i =
        NyarInteger::from_str("145175341267598143125685194855555555555555555666666666666666666666666666666666666666666425")
            .expect("");
    println!("{i}")
}

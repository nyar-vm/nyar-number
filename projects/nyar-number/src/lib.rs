#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]
#![feature(lazy_cell)]

mod decimal;
mod integer;
mod number;
mod rational;
mod sign;
// mod complex;
mod unsigned;
pub(crate) mod utils;

pub use self::{
    decimal::NyarDecimal, integer::NyarInteger, number::NyarReal, rational::NyarRational, unsigned::NyarUnsigned,
    utils::NyarNumberError,
};
pub use num::traits::{One, Zero};

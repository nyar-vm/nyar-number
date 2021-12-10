#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]
#![feature(lazy_cell)]

mod complex;
mod decimal;
mod integer;
mod number;
mod rational;
mod sign;
mod unsigned;
pub(crate) mod utils;

pub use self::{decimal::NyarDecimal, integer::NyarInteger, number::NyarReal, rational::NyarRational, unsigned::NyarDigits};
pub use num::traits::{Num, One, ToPrimitive, Zero};
pub use nyar_error::NyarError;

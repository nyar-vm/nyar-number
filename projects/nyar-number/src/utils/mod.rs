use crate::NyarReal;
use num::{bigint::Sign, BigUint};
#[cfg(feature = "serde")]
use serde::de::{MapAccess, Visitor};
#[cfg(feature = "serde")]
use std::fmt::Formatter;

/// A non-number appears in analysis or operation.
pub struct RealVisitor {
    pub r#type: String,
    pub sign: Sign,
    pub digits: Vec<u64>,
    pub numerator: Option<BigUint>,
    pub denominator: Option<BigUint>,
    pub re: Option<NyarReal>,
    pub im: Option<NyarReal>,
}

impl Default for RealVisitor {
    fn default() -> Self {
        Self { r#type: String::new(), sign: Sign::Plus, digits: vec![], numerator: None, denominator: None, re: None, im: None }
    }
}

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for RealVisitor {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        // never fail
        formatter.write_str("")
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(s) = map.next_key::<String>()? {
            match s.as_str() {
                "type" | "typing" => {
                    self.r#type = map.next_value()?;
                }
                "sign" => {
                    self.sign = map.next_value()?;
                }
                "value" => {
                    self.digits = map.next_value()?;
                }
                "numerator" => {
                    self.numerator = map.next_value()?;
                }
                "denominator" => {
                    self.denominator = map.next_value()?;
                }
                "re" => {
                    self.re = map.next_value()?;
                }
                "im" => {
                    self.im = map.next_value()?;
                }
                _ => {

                    // drop
                }
            }
        }
        Ok(self)
    }
}

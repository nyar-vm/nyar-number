use num::{bigint::Sign, BigUint};
#[cfg(feature = "serde")]
use serde::de::{MapAccess, Visitor};
use std::fmt::Formatter;

pub struct RealVisitor {
    pub r#type: String,
    pub sign: Sign,
    pub value: Option<BigUint>,
    pub numerator: Option<BigUint>,
    pub denominator: Option<BigUint>,
}

impl Default for RealVisitor {
    fn default() -> Self {
        Self { r#type: String::new(), sign: Sign::Plus, value: None, numerator: None, denominator: None }
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
                    self.value = map.next_value()?;
                }
                "numerator" => {
                    self.numerator = map.next_value()?;
                }
                "denominator" => {
                    self.denominator = map.next_value()?;
                }
                _ => {
                    // drop
                }
            }
        }
        Ok(self)
    }
}

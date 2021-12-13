use super::*;
use num::ToPrimitive;

impl ToPrimitive for NyarInfinity {
    fn to_i64(&self) -> Option<i64> {
        None
    }

    fn to_u64(&self) -> Option<u64> {
        None
    }
    fn to_f32(&self) -> Option<f32> {
        Some(match self.sign {
            Plus => f32::INFINITY,
            Minus => f32::NEG_INFINITY,
            NoSign => f32::NAN,
        })
    }
    fn to_f64(&self) -> Option<f64> {
        Some(match self.sign {
            Plus => f64::INFINITY,
            Minus => f64::NEG_INFINITY,
            NoSign => f64::NAN,
        })
    }
}

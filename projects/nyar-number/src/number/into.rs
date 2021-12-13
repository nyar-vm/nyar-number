use super::*;
use num::ToPrimitive;

impl ToPrimitive for NyarReal {
    /// Used to act as an array index
    fn to_isize(&self) -> Option<isize> {
        match self {
            NyarReal::Rational(v) => {
                if v.denominator.is_one() {
                    v.numerator.to_isize()
                }
                else {
                    None
                }
            }
            NyarReal::Decimal(_) => None,
            NyarReal::Infinity(_) => None,
        }
    }

    fn to_i8(&self) -> Option<i8> {
        todo!()
    }

    fn to_i16(&self) -> Option<i16> {
        todo!()
    }

    fn to_i32(&self) -> Option<i32> {
        todo!()
    }

    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_i128(&self) -> Option<i128> {
        todo!()
    }
    /// Used to act as an array index
    fn to_usize(&self) -> Option<usize> {
        match self {
            NyarReal::Rational(v) => {
                if v.denominator.is_one() {
                    v.numerator.to_usize()
                }
                else {
                    None
                }
            }
            NyarReal::Decimal(_) => None,
            NyarReal::Infinity(_) => None,
        }
    }

    fn to_u8(&self) -> Option<u8> {
        todo!()
    }

    fn to_u16(&self) -> Option<u16> {
        todo!()
    }

    fn to_u32(&self) -> Option<u32> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }

    fn to_u128(&self) -> Option<u128> {
        todo!()
    }

    fn to_f32(&self) -> Option<f32> {
        match self {
            Self::Rational(v) => v.to_f32(),
            Self::Decimal(v) => v.to_f32(),
            Self::Infinity(v) => v.to_f32(),
        }
    }

    fn to_f64(&self) -> Option<f64> {
        match self {
            Self::Rational(v) => v.to_f64(),
            Self::Decimal(v) => v.to_f64(),
            Self::Infinity(v) => v.to_f64(),
        }
    }
}

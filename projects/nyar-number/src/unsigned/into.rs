use super::*;

impl ToPrimitive for NyarUnsigned {
    fn to_isize(&self) -> Option<isize> {
        self._repr.to_isize()
    }

    fn to_i8(&self) -> Option<i8> {
        self._repr.to_i8()
    }

    fn to_i16(&self) -> Option<i16> {
        self._repr.to_i16()
    }

    fn to_i32(&self) -> Option<i32> {
        self._repr.to_i32()
    }

    fn to_i64(&self) -> Option<i64> {
        self._repr.to_i64()
    }

    fn to_i128(&self) -> Option<i128> {
        self._repr.to_i128()
    }

    fn to_usize(&self) -> Option<usize> {
        self._repr.to_usize()
    }

    fn to_u8(&self) -> Option<u8> {
        self._repr.to_u8()
    }

    fn to_u16(&self) -> Option<u16> {
        self._repr.to_u16()
    }

    fn to_u32(&self) -> Option<u32> {
        self._repr.to_u32()
    }

    fn to_u64(&self) -> Option<u64> {
        self._repr.to_u64()
    }

    fn to_u128(&self) -> Option<u128> {
        self._repr.to_u128()
    }

    fn to_f32(&self) -> Option<f32> {
        self._repr.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self._repr.to_f64()
    }
}

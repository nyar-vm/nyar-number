use super::*;


impl Zero for NyarUnsigned {
    fn zero() -> Self {
        Self { _repr: BigUint::default() }
    }

    fn is_zero(&self) -> bool {
        self._repr.is_zero()
    }
}

impl One for NyarUnsigned {
    fn one() -> Self {
        Self { _repr: BigUint::one() }
    }
}

impl Add for NyarUnsigned {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { _repr: self._repr.add(rhs._repr) }
    }
}
impl Sub for NyarUnsigned {
    type Output = Self;

    fn sub(self, _: Self) -> Self::Output {
        unreachable!()
    }
}

impl Mul for NyarUnsigned {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { _repr: self._repr.mul(rhs._repr) }
    }
}
impl Div for NyarUnsigned {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

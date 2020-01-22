use super::*;

impl NyarUnsigned {
    pub fn zero() -> Gc<Self> {
        ZERO.clone()
    }
    pub fn one() -> Gc<Self> {
        ONE.clone()
    }
}

impl Zero for NyarUnsigned {
    fn zero() -> Self {
        ZERO.get().clone()
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

impl Mul for NyarUnsigned {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { _repr: self._repr.mul(rhs._repr) }
    }
}

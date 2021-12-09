use super::*;

impl NyarUnsigned {
    /// All addresses with a value of 0
    pub fn zero() -> Gc<Self> {
        ZERO.clone()
    }
    /// All addresses with a value of 1
    pub fn one() -> Gc<Self> {
        ONE.clone()
    }
}

impl Zero for NyarUnsigned {
    fn zero() -> Self {
        ZERO.get().clone()
    }

    fn is_zero(&self) -> bool {
        self._repr.get().is_empty()
    }
}

impl One for NyarUnsigned {
    fn one() -> Self {
        Self { _repr: Gc::new(vec![1]) }
    }
}

impl Add for NyarUnsigned {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.delegate().add(rhs.delegate()).into()
    }
}

impl Mul for NyarUnsigned {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.delegate().mul(rhs.delegate()).into()
    }
}

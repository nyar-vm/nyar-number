use super::*;

pub(crate) static ZERO: LazyLock<Gc<Vec<u64>>> = LazyLock::new(|| Gc::new(vec![]));
pub(crate) static ONE: LazyLock<Gc<Vec<u64>>> = LazyLock::new(|| Gc::new(vec![1]));

impl Zero for NyarDigits {
    fn zero() -> Self {
        Self { _repr: ZERO.clone() }
    }

    fn is_zero(&self) -> bool {
        self._repr.get().is_empty()
    }
}

impl One for NyarDigits {
    fn one() -> Self {
        Self { _repr: ONE.clone() }
    }
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self._repr.get()[..] == [1]
    }
}

impl Add for NyarDigits {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.delegate().add(rhs.delegate()).into()
    }
}

impl Mul for NyarDigits {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.delegate().mul(rhs.delegate()).into()
    }
}

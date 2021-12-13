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

impl CheckedSub for NyarDigits {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        match self.cmp(rhs) {
            Ordering::Less => None,
            Ordering::Equal => Some(Self::zero()),
            Ordering::Greater => Some(self.delegate().sub(rhs.delegate()).into()),
        }
    }
}

impl Sub for NyarDigits {
    type Output = Self;

    /// `saturating_sub`, minimum reduction to 0, no loopback.
    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(&rhs).unwrap_or_default()
    }
}

impl Mul for NyarDigits {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.delegate().mul(rhs.delegate()).into()
    }
}

impl Div for NyarDigits {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.delegate().div(&rhs.delegate()).into()
    }
}

impl Rem for NyarDigits {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.delegate().rem(&rhs.delegate()).into()
    }
}

impl Integer for NyarDigits {
    fn div_floor(&self, other: &Self) -> Self {
        self.delegate().div_floor(&other.delegate()).into()
    }

    fn mod_floor(&self, other: &Self) -> Self {
        self.delegate().mod_floor(&other.delegate()).into()
    }

    fn gcd(&self, other: &Self) -> Self {
        self.delegate().gcd(&other.delegate()).into()
    }

    fn lcm(&self, other: &Self) -> Self {
        self.delegate().lcm(&other.delegate()).into()
    }

    fn divides(&self, other: &Self) -> bool {
        self.is_multiple_of(other)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        self.delegate().is_multiple_of(&other.delegate())
    }

    fn is_even(&self) -> bool {
        // take last digit
        match self._repr.get().first() {
            Some(x) => x.is_even(),
            None => true, // zero
        }
    }

    fn is_odd(&self) -> bool {
        // take last digit
        match self._repr.get().first() {
            Some(x) => x.is_odd(),
            None => false, // zero
        }
    }

    fn div_rem(&self, _: &Self) -> (Self, Self) {
        todo!()
    }
}

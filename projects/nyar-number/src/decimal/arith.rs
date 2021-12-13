use super::*;

impl Zero for NyarDecimal {
    fn zero() -> Self {
        Self { sign: Sign::Plus, digits: NyarDigits::zero(), scale: 0 }
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}
impl One for NyarDecimal {
    fn one() -> Self {
        Self { sign: Sign::Plus, digits: NyarDigits::one(), scale: 0 }
    }
}

impl Neg for NyarDecimal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { sign: -self.sign, digits: self.digits, scale: self.scale }
    }
}

impl Add for NyarDecimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.delegate().add(rhs.delegate()).into()
    }
}
impl Sub for NyarDecimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.delegate().sub(rhs.delegate()).into()
    }
}

impl Mul for NyarDecimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.delegate().mul(rhs.delegate()).into()
    }
}

impl Div for NyarDecimal {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.delegate().div(rhs.delegate()).into()
    }
}

impl Rem for NyarDecimal {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.delegate().rem(rhs.delegate()).into()
    }
}

impl Signed for NyarDecimal {
    fn abs(&self) -> Self {
        Self { sign: Sign::Plus, digits: self.digits.clone(), scale: self.scale }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        self.delegate().abs_sub(&other.delegate()).into()
    }

    fn signum(&self) -> Self {
        self.delegate().signum().into()
    }

    fn is_positive(&self) -> bool {
        matches!(self.sign, Sign::Plus)
    }

    fn is_negative(&self) -> bool {
        matches!(self.sign, Sign::Minus)
    }
}
impl NyarDecimal {
    ///
    ///
    /// # Arguments
    ///
    /// * `rhs`:
    ///
    /// returns: NyarReal
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn safe_div(&self, rhs: Self) -> NyarReal {
        if rhs.is_zero() {
            NyarReal::infinity(self.sign)
        }
        else {
            NyarReal::Decimal(self.delegate().div(rhs.delegate()).into())
        }
    }
    ///
    ///
    /// # Arguments
    ///
    /// * `rhs`:
    ///
    /// returns: NyarReal
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn safe_rem(&self, rhs: Self) -> NyarReal {
        if rhs.is_zero() {
            NyarReal::infinity(self.sign)
        }
        else {
            NyarReal::Decimal(self.delegate().rem(rhs.delegate()).into())
        }
    }
}

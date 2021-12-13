use super::*;

impl Neg for NyarInfinity {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.sign {
            Minus => Self { sign: Plus },
            Plus => Self { sign: Minus },
            NoSign => Self { sign: NoSign },
        }
    }
}

impl Add for NyarInfinity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Plus, Plus) => Self { sign: Plus },
            (Minus, Minus) => Self { sign: Minus },

            _ => Self { sign: NoSign },
        }
    }
}

impl Sub for NyarInfinity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Plus, Minus) => Self { sign: Plus },
            (Minus, Plus) => Self { sign: Minus },
            _ => Self { sign: NoSign },
        }
    }
}

impl Mul for NyarInfinity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Plus, Plus) | (Minus, Minus) => Self { sign: Plus },
            (Plus, Minus) | (Minus, Plus) => Self { sign: Minus },
            _ => Self { sign: NoSign },
        }
    }
}

impl Div for NyarInfinity {
    type Output = Self;

    fn div(self, _: Self) -> Self::Output {
        Self { sign: NoSign }
    }
}

impl Rem for NyarInfinity {
    type Output = Self;

    fn rem(self, _: Self) -> Self::Output {
        Self { sign: NoSign }
    }
}

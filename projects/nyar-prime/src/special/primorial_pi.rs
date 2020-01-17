use num::BigInt;

pub struct PrimorialPi {
    index: BigInt,
}

impl PrimorialPi {
    pub fn new<T>(i: T) -> PrimorialPi
    where
        BigInt: From<T>,
    {
        let int = BigInt::from(i);
        PrimorialPi { index: int }
    }
}

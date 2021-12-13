use num::BigInt;

pub struct Primorial {
    index: BigInt,
}

impl Primorial {
    pub fn new<T>(i: T) -> Primorial
    where
        BigInt: From<T>,
    {
        let int = BigInt::from(i);
        Primorial { index: int }
    }
    pub fn unwrap(&self) -> &BigInt{
       & self.index
    }
}

use super::*;


impl Error for NyarNumberError {}
impl Display for NyarNumberError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NyarNumberError::ParseError(v) => Display::fmt(v, f),
        }
    }
}

impl From<ParseBigDecimalError> for NyarNumberError {
    fn from(value: ParseBigDecimalError) -> Self {
        Self::ParseError(value.to_string())
    }
}
impl From<ParseRatioError> for NyarNumberError {
    fn from(value: ParseRatioError) -> Self {
        Self::ParseError(value.to_string())
    }
}

impl From<ParseBigIntError> for NyarNumberError {
    fn from(value: ParseBigIntError) -> Self {
        Self::ParseError(value.to_string())
    }
}

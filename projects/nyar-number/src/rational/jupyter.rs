use bigdecimal::num_bigint::Sign;
use jupyter_types::{Executed, JupyterContext, JupyterTheme};
use crate::{NyarRational, One};
use jupyter_types::third_party::Value;
impl Executed for NyarRational {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, context: &JupyterContext) -> Value {
        let color = match context.theme {
            JupyterTheme::Light => "#986801",
            JupyterTheme::Dark => "#986801",
        };
        let buffer = if self.denominator.is_one() {
            match self.sign {
                Sign::Minus => { format!(r#"<span style="color: {color}">-{}</span>"#, self.numerator) }
                Sign::NoSign => { format!(r#"<span style="color: {color}">0</span>"#) }
                Sign::Plus => { format!(r#"<span style="color: {color}">{}</span>"#, self.numerator) }
            }
        } else {
            match self.sign {
                Sign::Minus => { format!(r#"<span style="color: {color}">-{}/{}</span>"#, self.numerator, self.denominator) }
                Sign::NoSign => { format!(r#"<span style="color: {color}">0</span>"#) }
                Sign::Plus => { format!(r#"<span style="color: {color}">{}/{}</span>"#, self.numerator, self.denominator) }
            }
        };
        Value::String(buffer)
    }
}
use bigdecimal::num_bigint::Sign;
use jupyter_types::{Executed, JupyterContext, JupyterTheme};
use crate::NyarInteger;
use jupyter_types::third_party::Value;
#[cfg(feature = "jupyter-types")]
impl Executed for NyarInteger {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, context: &JupyterContext) -> Value {
        let color = match context.theme {
            JupyterTheme::Light => "#986801",
            JupyterTheme::Dark => "#986801",
        };
        let buffer = match self.sign {
            Sign::Minus => { format!(r#"<span style="color: {color}">-{}</span>"#, self.digits) }
            Sign::NoSign => { format!(r#"<span style="color: {color}">0</span>"#) }
            Sign::Plus => { format!(r#"<span style="color: {color}">{}</span>"#, self.digits) }
        };
        Value::String(buffer)
    }
}


use jupyter_types::{Executed, JupyterContext, JupyterTheme};
use crate::NyarDigits;
use jupyter_types::third_party::Value;
impl Executed for NyarDigits {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, context: &JupyterContext) -> Value {
        let color = match context.theme {
            JupyterTheme::Light => "#986801",
            JupyterTheme::Dark => "#986801",
        };
        let buffer = format!(r#"<span style="color: {color}">{}</span>"#, self.to_string());
        Value::String(buffer)
    }
}
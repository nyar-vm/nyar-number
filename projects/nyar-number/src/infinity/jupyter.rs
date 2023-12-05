use jupyter_types::{Executed, JupyterContext, JupyterTheme};
use crate::infinity::NyarInfinity;
use jupyter_types::third_party::Value;
impl Executed for NyarInfinity {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, context: &JupyterContext) -> Value {
        let color = match context.theme {
            JupyterTheme::Light => "#986801",
            JupyterTheme::Dark => "#986801",
        };
        Value::String(format!(r#"<span style="color: {color}">{}</span>"#, self.to_string()))
    }
}
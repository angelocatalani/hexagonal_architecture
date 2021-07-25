use serde::{Deserialize, Serialize};
#[derive(Serialize)]
pub struct TranslatedInput<'a> {
    pub text: &'a str,
}
#[derive(Deserialize)]
pub struct TranslatedOutput {
    pub translated: String,
}

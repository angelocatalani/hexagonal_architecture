use serde::{Deserialize, Serialize};
#[derive(Serialize)]
pub struct TranslatedInput<'a> {
    pub text: &'a str,
}
#[derive(Deserialize)]
pub struct TranslatedOutput {
    contents: Content,
}

#[derive(Deserialize)]
struct Content {
    translated: String,
}

impl TranslatedOutput {
    pub fn translated(self) -> String {
        self.contents.translated
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Input<'a> {
    pub text: &'a str,
}

#[derive(Deserialize)]
pub struct Output {
    contents: Content,
}

#[derive(Deserialize)]
struct Content {
    translated: String,
}

impl Output {
    pub fn translated(self) -> String {
        self.contents.translated
    }
}

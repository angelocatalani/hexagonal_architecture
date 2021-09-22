#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    habitat: Option<String>,
    is_legendary: bool,
    name: String,
}

impl Pokemon {
    pub fn new(
        description: Option<String>,
        habitat: Option<String>,
        is_legendary: bool,
        name: String,
    ) -> Self {
        Pokemon {
            description,
            habitat,
            is_legendary,
            name,
        }
    }
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn is_cave_or_legendary(&self) -> bool {
        self.habitat.as_deref().eq(&Some("cave")) || self.is_legendary
    }
    pub fn with_description(self, description: String) -> Self {
        Self {
            description: Some(description),
            habitat: self.habitat,
            is_legendary: self.is_legendary,
            name: self.name,
        }
    }
}

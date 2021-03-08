use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub display_name: String,
    #[serde(default)]
    pub has_mode: bool,
}
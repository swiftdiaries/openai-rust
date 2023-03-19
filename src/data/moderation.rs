use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub const MODERATION_ENDPOINT: &str = "https://api.openai.com/v1/moderations";

pub struct RequestParams {}

#[derive(Serialize, Deserialize, Debug, TypedBuilder, Clone)]
pub struct ModerationRequestParams {
    pub input: String,
    #[builder(default = "text-moderation-latest".to_string())]
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub struct ModerationResponseParams {
    pub id: String,
    pub model: String,
    pub results: Vec<ModerationResult>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub struct ModerationResult {
    pub categories: ModerationCategory,
    pub category_scores: ModerationCategoryScores,
    pub flagged: bool,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub struct ModerationCategory {
    pub hate: bool,
    #[serde(alias = "hate/threatening")]
    pub hate_threatening: bool,
    #[serde(alias = "self-harm")]
    pub self_harm: bool,
    pub sexual: bool,
    #[serde(alias = "sexual/minors")]
    pub sexual_minors: bool,
    pub violence: bool,
    #[serde(alias = "violence/graphic")]
    pub violence_graphic: bool,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub struct ModerationCategoryScores {
    pub hate: f64,
    #[serde(alias = "hate/threatening")]
    pub hate_threatening: f64,
    #[serde(alias = "self-harm")]
    pub self_harm: f64,
    pub sexual: f64,
    #[serde(alias = "sexual/minors")]
    pub sexual_minors: f64,
    pub violence: f64,
    #[serde(alias = "violence/graphic")]
    pub violence_graphic: f64,
}

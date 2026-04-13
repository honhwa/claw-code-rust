use std::str::FromStr;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKind {
    Anthropic,
    Openai,
    Ollama,
}

impl ProviderKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ProviderKind::Anthropic => "anthropic",
            ProviderKind::Openai => "openai",
            ProviderKind::Ollama => "ollama",
        }
    }
}

/// OpenAI models support reasoning effort.
/// See <https://platform.openai.com/docs/guides/reasoning?api-mode=responses#get-started-with-reasoning>
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Display,
    JsonSchema,
    EnumIter,
    Hash,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ReasoningEffort {
    None,
    Minimal,
    Low,
    #[default]
    Medium,
    High,
    XHigh,
}

impl FromStr for ReasoningEffort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .map_err(|_| format!("invalid reasoning_effort: {s}"))
    }
}

impl ReasoningEffort {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Minimal => "Minimal",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::XHigh => "XHigh",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::None => "Disable extra reasoning effort",
            Self::Minimal => "Use the lightest supported reasoning effort",
            Self::Low => "Fastest, cheapest, least deliberative",
            Self::Medium => "Balanced speed and deliberation",
            Self::High => "More deliberate for harder tasks",
            Self::XHigh => "Most deliberate, highest effort",
        }
    }
}

/// Maps reasoning efforts onto a stable numeric scale for comparison.
fn effort_rank(effort: ReasoningEffort) -> i32 {
    match effort {
        ReasoningEffort::None => 0,
        ReasoningEffort::Minimal => 1,
        ReasoningEffort::Low => 2,
        ReasoningEffort::Medium => 3,
        ReasoningEffort::High => 4,
        ReasoningEffort::XHigh => 5,
    }
}

/// Picks the supported effort closest to the requested one.
fn nearest_effort(target: ReasoningEffort, supported: &[ReasoningEffort]) -> ReasoningEffort {
    let target_rank = effort_rank(target);
    supported
        .iter()
        .copied()
        .min_by_key(|candidate| (effort_rank(*candidate) - target_rank).abs())
        .unwrap_or(target)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningEffortOption {
    pub effort: ReasoningEffort,
    pub description: String,
}

impl ReasoningEffortOption {
    pub fn new(effort: ReasoningEffort, description: impl Into<String>) -> Self {
        Self {
            effort,
            description: description.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThinkingOption {
    pub label: String,
    pub description: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThinkingCapability {
    /// Model thinking cannot be controlled.
    Disabled,
    /// Model thinking can be toggled on and off.
    Toggle,
    /// Multiple effort levels can be selected for thinking.
    Levels(Vec<ReasoningEffort>),
}

impl ThinkingCapability {
    pub fn options(&self) -> Vec<ThinkingOption> {
        match self {
            ThinkingCapability::Disabled => Vec::new(),
            ThinkingCapability::Toggle => vec![
                ThinkingOption {
                    label: "Off".to_string(),
                    description: "Disable thinking for this turn".to_string(),
                    value: "disabled".to_string(),
                },
                ThinkingOption {
                    label: "On".to_string(),
                    description: "Enable the model's thinking mode".to_string(),
                    value: "enabled".to_string(),
                },
            ],
            ThinkingCapability::Levels(levels) => levels
                .iter()
                .copied()
                .map(|effort| ThinkingOption {
                    label: effort.label().to_string(),
                    description: effort.description().to_string(),
                    value: effort.label().to_lowercase(),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Low,
    Medium,
    High,
}

impl Default for Verbosity {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InputModality {
    Text,
    Image,
}

impl Default for InputModality {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelVisibility {
    Visible,
    Hidden,
    Experimental,
}

impl Default for ModelVisibility {
    fn default() -> Self {
        Self::Visible
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TruncationPolicyConfig {
    pub default_max_chars: usize,
    pub tool_output_max_chars: usize,
    pub user_input_max_chars: usize,
    pub binary_placeholder: String,
    pub preserve_json_shape: bool,
}

impl Default for TruncationPolicyConfig {
    fn default() -> Self {
        Self {
            default_max_chars: 8_000,
            tool_output_max_chars: 16_000,
            user_input_max_chars: 32_000,
            binary_placeholder: "[binary]".into(),
            preserve_json_shape: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ModelConfig {
    pub slug: String,
    pub display_name: String,
    pub provider: ProviderKind,
    pub description: Option<String>,
    pub default_reasoning_effort: ReasoningEffort,
    pub supported_reasoning_efforts: Vec<ReasoningEffort>,
    pub thinking_capability: Option<ThinkingCapability>,
    pub base_instructions: String,
    pub context_window: u32,
    pub effective_context_window_percent: u8,
    pub auto_compact_token_limit: Option<u32>,
    pub truncation_policy: TruncationPolicyConfig,
    pub input_modalities: Vec<InputModality>,
    pub supports_image_detail_original: bool,
    pub visibility: ModelVisibility,
    pub supported_in_api: bool,
    pub priority: i32,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            slug: String::new(),
            display_name: String::new(),
            provider: ProviderKind::Anthropic,
            description: None,
            default_reasoning_effort: ReasoningEffort::default(),
            supported_reasoning_efforts: vec![ReasoningEffort::default()],
            thinking_capability: None,
            base_instructions: String::new(),
            context_window: 200_000,
            effective_context_window_percent: 90,
            auto_compact_token_limit: None,
            truncation_policy: TruncationPolicyConfig::default(),
            input_modalities: vec![InputModality::default()],
            supports_image_detail_original: false,
            visibility: ModelVisibility::default(),
            supported_in_api: true,
            priority: 0,
        }
    }
}

impl ModelConfig {
    pub fn reasoning_effort_options(&self) -> Vec<ReasoningEffortOption> {
        let mut options = Vec::new();
        let mut seen = std::collections::HashSet::new();

        let push_effort =
            |effort: &ReasoningEffort,
             options: &mut Vec<ReasoningEffortOption>,
             seen: &mut std::collections::HashSet<ReasoningEffort>| {
                if seen.insert(*effort) {
                    options.push(ReasoningEffortOption::new(*effort, effort.description()));
                }
            };

        push_effort(&self.default_reasoning_effort, &mut options, &mut seen);
        for effort in &self.supported_reasoning_efforts {
            push_effort(effort, &mut options, &mut seen);
        }
        options
    }

    pub fn effective_thinking_capability(&self) -> ThinkingCapability {
        self.thinking_capability
            .clone()
            .unwrap_or_else(|| ThinkingCapability::Levels(self.supported_reasoning_efforts.clone()))
    }

    pub fn nearest_supported_reasoning_effort(&self, target: ReasoningEffort) -> ReasoningEffort {
        nearest_effort(target, &self.supported_reasoning_efforts)
    }
}

/// Provides read-only access to model definitions and turn-resolution behavior.
pub trait ModelCatalog: Send + Sync {
    fn list_visible(&self) -> Vec<&ModelConfig>;
    fn get(&self, slug: &str) -> Option<&ModelConfig>;
    fn resolve_for_turn(&self, requested: Option<&str>) -> Result<&ModelConfig, ModelConfigError>;
}

#[derive(Debug, Clone)]
pub struct InMemoryModelCatalog {
    models: Vec<ModelConfig>,
}

impl InMemoryModelCatalog {
    pub fn new(models: Vec<ModelConfig>) -> Self {
        Self { models }
    }
}

impl ModelCatalog for InMemoryModelCatalog {
    fn list_visible(&self) -> Vec<&ModelConfig> {
        self.models
            .iter()
            .filter(|model| model.visibility == ModelVisibility::Visible)
            .collect()
    }

    fn get(&self, slug: &str) -> Option<&ModelConfig> {
        self.models.iter().find(|model| model.slug == slug)
    }

    fn resolve_for_turn(&self, requested: Option<&str>) -> Result<&ModelConfig, ModelConfigError> {
        if let Some(slug) = requested {
            return self
                .get(slug)
                .ok_or_else(|| ModelConfigError::ModelNotFound {
                    slug: slug.to_string(),
                });
        }

        self.list_visible()
            .into_iter()
            .max_by_key(|model| model.priority)
            .ok_or(ModelConfigError::NoVisibleModels)
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ModelConfigError {
    #[error("model not found: {slug}")]
    ModelNotFound { slug: String },
    #[error("no visible models available")]
    NoVisibleModels,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::{
        InMemoryModelCatalog, InputModality, ModelCatalog, ModelConfig, ModelVisibility,
        ProviderKind, ReasoningEffort, TruncationPolicyConfig,
    };

    fn model(slug: &str, priority: i32, visibility: ModelVisibility) -> ModelConfig {
        ModelConfig {
            slug: slug.into(),
            display_name: slug.into(),
            provider: ProviderKind::Anthropic,
            description: None,
            default_reasoning_effort: ReasoningEffort::Medium,
            supported_reasoning_efforts: vec![ReasoningEffort::Medium],
            thinking_capability: None,
            base_instructions: String::new(),
            context_window: 200_000,
            effective_context_window_percent: 90,
            auto_compact_token_limit: None,
            truncation_policy: TruncationPolicyConfig {
                default_max_chars: 8_000,
                tool_output_max_chars: 16_000,
                user_input_max_chars: 32_000,
                binary_placeholder: "[binary]".into(),
                preserve_json_shape: true,
            },
            input_modalities: vec![InputModality::Text],
            supports_image_detail_original: false,
            visibility,
            supported_in_api: true,
            priority,
        }
    }

    #[test]
    fn resolve_for_turn_uses_highest_priority_visible_default() {
        let catalog = InMemoryModelCatalog::new(vec![
            model("hidden", 100, ModelVisibility::Hidden),
            model("visible-low", 1, ModelVisibility::Visible),
            model("visible-high", 10, ModelVisibility::Visible),
        ]);

        let resolved = catalog.resolve_for_turn(None).expect("resolve default");
        assert_eq!(resolved.slug, "visible-high");
    }

    #[test]
    fn resolve_for_turn_honors_requested_slug() {
        let catalog = InMemoryModelCatalog::new(vec![model("test", 1, ModelVisibility::Visible)]);
        let resolved = catalog
            .resolve_for_turn(Some("test"))
            .expect("resolve explicit");
        assert_eq!(resolved.slug, "test");
    }
}

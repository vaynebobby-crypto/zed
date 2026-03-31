use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const MOONSHOT_API_URL: &str = "https://api.moonshot.cn/v1";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "moonshot-v1-8k")]
    MoonshotV1_8k,
    #[serde(rename = "moonshot-v1-32k")]
    MoonshotV1_32k,
    #[serde(rename = "moonshot-v1-128k")]
    MoonshotV1_128k,
    #[serde(rename = "kimi-k2-0711-preview")]
    KimiK2_0711Preview,
    #[serde(rename = "kimi-k2-turbo-preview")]
    KimiK2TurboPreview,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        max_completion_tokens: Option<u64>,
        supports_images: Option<bool>,
        supports_tools: Option<bool>,
        parallel_tool_calls: Option<bool>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Self::KimiK2TurboPreview
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "moonshot-v1-8k" => Ok(Self::MoonshotV1_8k),
            "moonshot-v1-32k" => Ok(Self::MoonshotV1_32k),
            "moonshot-v1-128k" => Ok(Self::MoonshotV1_128k),
            "kimi-k2-0711-preview" => Ok(Self::KimiK2_0711Preview),
            "kimi-k2-turbo-preview" => Ok(Self::KimiK2TurboPreview),
            _ => anyhow::bail!("invalid model id '{id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::MoonshotV1_8k => "moonshot-v1-8k",
            Self::MoonshotV1_32k => "moonshot-v1-32k",
            Self::MoonshotV1_128k => "moonshot-v1-128k",
            Self::KimiK2_0711Preview => "kimi-k2-0711-preview",
            Self::KimiK2TurboPreview => "kimi-k2-turbo-preview",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::MoonshotV1_8k => "MoonShot V1 8K",
            Self::MoonshotV1_32k => "MoonShot V1 32K",
            Self::MoonshotV1_128k => "MoonShot V1 128K",
            Self::KimiK2_0711Preview => "Kimi K2 0711 Preview",
            Self::KimiK2TurboPreview => "Kimi K2 Turbo Preview",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::MoonshotV1_8k => 8_192,
            Self::MoonshotV1_32k => 32_768,
            Self::MoonshotV1_128k => 131_072,
            Self::KimiK2_0711Preview | Self::KimiK2TurboPreview => 131_072,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::MoonshotV1_8k => Some(4_096),
            Self::MoonshotV1_32k => Some(8_192),
            Self::MoonshotV1_128k | Self::KimiK2_0711Preview | Self::KimiK2TurboPreview => {
                Some(16_384)
            }
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }

    pub fn max_completion_tokens(&self) -> Option<u64> {
        match self {
            Self::MoonshotV1_8k
            | Self::MoonshotV1_32k
            | Self::MoonshotV1_128k
            | Self::KimiK2_0711Preview
            | Self::KimiK2TurboPreview => None,
            Self::Custom {
                max_completion_tokens,
                ..
            } => *max_completion_tokens,
        }
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::MoonshotV1_8k
            | Self::MoonshotV1_32k
            | Self::MoonshotV1_128k
            | Self::KimiK2_0711Preview
            | Self::KimiK2TurboPreview => true,
            Self::Custom {
                parallel_tool_calls: Some(support),
                ..
            } => *support,
            Self::Custom { .. } => false,
        }
    }

    pub fn supports_prompt_cache_key(&self) -> bool {
        false
    }

    pub fn supports_tool(&self) -> bool {
        match self {
            Self::MoonshotV1_8k
            | Self::MoonshotV1_32k
            | Self::MoonshotV1_128k
            | Self::KimiK2_0711Preview
            | Self::KimiK2TurboPreview => true,
            Self::Custom {
                supports_tools: Some(support),
                ..
            } => *support,
            Self::Custom { .. } => false,
        }
    }

    pub fn supports_images(&self) -> bool {
        match self {
            Self::MoonshotV1_8k
            | Self::MoonshotV1_32k
            | Self::MoonshotV1_128k
            | Self::KimiK2_0711Preview
            | Self::KimiK2TurboPreview => false,
            Self::Custom {
                supports_images: Some(support),
                ..
            } => *support,
            Self::Custom { .. } => false,
        }
    }
}

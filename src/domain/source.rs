use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    Git,
    Zip,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceRegistration {
    pub name: String,
    pub root: String,
    pub source_type: SourceType,
}

pub const APPROVED_SKILL_PATHS: [&str; 3] = ["skills", ".agents/skills", ".claude/skills"];

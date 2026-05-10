use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillsLockfile {
    pub version: u32,
    pub skills: BTreeMap<String, SkillLockEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillLockEntry {
    pub source: String,
    #[serde(rename = "sourceType")]
    pub source_type: String,
    #[serde(rename = "skillPath")]
    pub skill_path: String,
    #[serde(rename = "computedHash")]
    pub computed_hash: String,
}

impl SkillsLockfile {
    pub fn ensure_defaults(mut self) -> Self {
        if self.version == 0 {
            self.version = 1;
        }
        self
    }
}

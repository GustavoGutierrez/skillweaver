use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockReason {
    Traversal,
    InvalidMetadata,
    UnsupportedStructure,
    Conflict,
}

#[derive(Debug, Clone)]
pub struct PlannedSkill {
    pub name: String,
    pub source: String,
    pub source_type: String,
    pub skill_path: String,
    pub target_dir: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct InstallPlan {
    pub can_install: bool,
    pub blocked_by: Vec<BlockReason>,
    pub skills: Vec<PlannedSkill>,
    pub rules_markdown: String,
}

impl InstallPlan {
    pub fn blocked(reason: BlockReason) -> Self {
        Self {
            can_install: false,
            blocked_by: vec![reason],
            ..Default::default()
        }
    }
}

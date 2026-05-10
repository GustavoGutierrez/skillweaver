use serde::{Deserialize, Serialize};

use crate::{AppResult, error::AppError};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: String,
    pub triggers: Option<Vec<String>>,
}

pub fn parse_skill_frontmatter(skill_md: &str) -> AppResult<SkillFrontmatter> {
    let mut lines = skill_md.lines();
    if lines.next() != Some("---") {
        return Err(AppError::Blocked("missing YAML frontmatter".into()));
    }
    let mut yaml = String::new();
    for line in lines {
        if line == "---" {
            break;
        }
        yaml.push_str(line);
        yaml.push('\n');
    }
    let parsed: SkillFrontmatter = serde_yaml::from_str(&yaml)?;
    if parsed.name.trim().is_empty() || parsed.description.trim().is_empty() {
        return Err(AppError::Blocked(
            "frontmatter requires non-empty name and description".into(),
        ));
    }
    Ok(parsed)
}

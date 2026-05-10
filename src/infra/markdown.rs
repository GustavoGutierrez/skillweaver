use std::fs;
use std::path::Path;

use crate::AppResult;

pub const BEGIN: &str = "<!-- BEGIN SKILLWEAVER RULES -->";
pub const END: &str = "<!-- END SKILLWEAVER RULES -->";

pub fn apply_managed_rules(path: &Path, rendered_rules: &str) -> AppResult<()> {
    let existing = if path.exists() {
        fs::read_to_string(path)?
    } else {
        String::new()
    };
    let block = format!("{BEGIN}\n{rendered_rules}\n{END}");
    let output = if let (Some(start), Some(end)) = (existing.find(BEGIN), existing.find(END)) {
        let after = end + END.len();
        format!("{}{}{}", &existing[..start], block, &existing[after..])
    } else if existing.trim().is_empty() {
        format!("# Project Agent Instructions\n\n{block}\n")
    } else {
        format!("{existing}\n\n{block}\n")
    };
    fs::write(path, output)?;
    Ok(())
}

pub fn render_rules(rules: &[String]) -> String {
    let mut out = String::from("## Installed Rules\n");
    for r in rules {
        out.push_str(&format!("- {r}\n"));
    }
    out
}

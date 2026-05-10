use std::path::{Path, PathBuf};

pub fn resolve_managed_file(project_root: &Path) -> PathBuf {
    let agents = project_root.join("AGENTS.md");
    if agents.exists() {
        return agents;
    }
    let claude = project_root.join("CLAUDE.md");
    if claude.exists() {
        return claude;
    }
    agents
}

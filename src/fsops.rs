use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Atomically write bytes by creating a temporary file in-place and renaming.
pub fn atomic_write_bytes(path: &Path, content: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .with_context(|| format!("path has no parent: {}", path.display()))?;
    fs::create_dir_all(parent)
        .with_context(|| format!("failed to create parent dir {}", parent.display()))?;

    let stem = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("file")
        .to_string();
    let tmp_path = parent.join(format!(".{stem}.tmp"));
    if tmp_path.exists() {
        let _ = fs::remove_file(&tmp_path);
    }
    fs::write(&tmp_path, content)
        .with_context(|| format!("failed writing temp file {}", tmp_path.display()))?;
    fs::rename(&tmp_path, path).with_context(|| {
        format!(
            "failed renaming {} -> {}",
            tmp_path.display(),
            path.display()
        )
    })?;
    Ok(())
}

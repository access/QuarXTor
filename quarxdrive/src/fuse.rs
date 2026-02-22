use std::path::Path;

pub fn fuse_available() -> bool {
    false
}

pub fn mount_read_only(_repo_path: &Path, _mount_dir: &Path, _foreground: bool) -> Result<(), String> {
    Err("fuse not enabled at build time".to_string())
}

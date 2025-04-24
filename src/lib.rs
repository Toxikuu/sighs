#![doc = include_str!("../README.md")]

use std::os::unix::fs::MetadataExt;
use std::path::Path;
use walkdir::WalkDir;

/// # Returns the size in bytes of all files in a directory.
/// Recurses.
pub fn dirsize<P: AsRef<Path>>(dir: P) -> u64 {
    WalkDir::new(dir)
        .into_iter()
        .map_while(Result::ok)
        .map(|f| filesize(f.path()))
        .sum()
}

/// # Returns the size in bytes of a file, unwrapping to 0.
pub fn filesize<P: AsRef<Path>>(f: P) -> u64 {
    let f = f.as_ref();
    if f.is_dir() {
        0
    } else {
        f.metadata().map(|m| m.size()).unwrap_or(0)
    }
}

/// # Convenience function to choose return the size of a path (directory or file).
pub fn sighs<P: AsRef<Path>>(p: P) -> u64 {
    let p = p.as_ref();
    if p.is_dir() { dirsize(p) } else { filesize(p) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    static PROJECT_ROOT: LazyLock<PathBuf> =
        LazyLock::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));

    #[test]
    fn size_of_license() {
        assert_eq!(35149, filesize(PROJECT_ROOT.join("LICENSE")))
    }

    #[test]
    fn sighs_file() {
        assert_eq!(35149, sighs(PROJECT_ROOT.join("LICENSE")))
    }

    #[test]
    fn size_of_tests() {
        assert_eq!(26, dirsize(PROJECT_ROOT.join("tests")))
    }

    #[test]
    fn sighs_dir() {
        assert_eq!(26, sighs(PROJECT_ROOT.join("tests")))
    }
}

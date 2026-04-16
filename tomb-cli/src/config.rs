use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use etcetera::{BaseStrategy, choose_base_strategy, home_dir};
use serde::Deserialize;

use crate::error::{Result, TombError};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum FileEntry {
    Path { path: PathBuf, context: Option<String> },
    Glob { glob: String, context: Option<String> },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InboxEntry {
    pub path: PathBuf,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Config {
    pub files: Vec<FileEntry>,
    pub inbox: Vec<InboxEntry>,
    /// Directory containing the config file, used as base for relative paths
    #[serde(skip)]
    pub root_dir: Option<PathBuf>,
}

/// Intermediate struct for TOML deserialization (allows missing sections)
#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
struct TomlConfig {
    #[serde(default)]
    files: Vec<FileEntry>,
    #[serde(default)]
    inbox: Vec<InboxEntry>,
}

const TOMB_CONFIG_FILE_NAME: &str = ".tomb.toml";
const TOMB_CONFIG_FILE_NAME_XDG: &str = "tomb/config.toml";

/// Resolve a config by searching for config files.
///
/// Search order:
/// 1. Walk up from `start_dir` looking for `.tomb.toml`
///    - stop at root marker
/// 2. `~/.tomb.toml`
/// 3. `~/.config/tomb/config.toml` (XDG/platform config dir)
/// 4. If nothing found, return a default empty config
pub fn resolve_config(start_dir: &Path, root_markers: &[&str]) -> Result<Config> {
    // Find the first existing config file path from the search locations.
    // Use Path::ancestors() to walk up from start_dir.
    let repo_config = start_dir
        .ancestors()
        .take_while(|dir| {
            !root_markers.iter().any(|marker| dir.join(marker).exists()) || dir.join(TOMB_CONFIG_FILE_NAME).is_file()
        })
        .find_map(|dir| {
            let candidate = dir.join(TOMB_CONFIG_FILE_NAME);
            candidate.is_file().then_some(candidate)
        });

    let home_config = home_dir().map(|home_dir| home_dir.join(TOMB_CONFIG_FILE_NAME)).ok();
    let xdg_config = choose_base_strategy()
        .map(|strategy| strategy.config_dir().join(TOMB_CONFIG_FILE_NAME_XDG))
        .ok();

    let config_path = [repo_config, home_config, xdg_config]
        .into_iter()
        .flatten()
        .find(|path| path.is_file())
        .ok_or(TombError::Config(String::from("Could not resolve config")))?;

    toml::from_str::<TomlConfig>(&fs::read_to_string(&config_path)?)
        .map(|config| Config {
            files: config.files,
            inbox: config.inbox,
            root_dir: config_path.parent().map(|parent| parent.to_path_buf()),
        })
        .map_err(|err| TombError::Config(err.message().to_string()))
}

/// Expand `~` prefix to the user's home directory and resolve relative
/// paths against `base`.
///
/// - `~/foo` becomes `/home/user/foo`
/// - `foo/bar` becomes `base/foo/bar`
/// - `/absolute/path` stays as-is
pub fn resolve_path(base: &Path, raw: &Path) -> PathBuf {
    if let Ok(suffix) = raw.strip_prefix("~") {
        home_dir()
            .map(|home_dir| home_dir.join(suffix))
            .unwrap_or(raw.to_path_buf())
    } else if raw.is_relative() {
        base.join(raw)
    } else {
        raw.to_path_buf()
    }
}

/// Apply path resolution to all entries in a parsed config.
/// Call resolve_path on every FileEntry::Path and InboxEntry path
/// using config.root_dir as the base.
pub fn resolve_paths(config: &mut Config) {
    let base = config.root_dir.clone().unwrap_or_default();

    for entry in &mut config.files {
        if let FileEntry::Path { path, .. } = entry {
            *path = resolve_path(&base, path)
        }
    }

    for entry in &mut config.inbox {
        entry.path = resolve_path(&base, &entry.path)
    }
}

impl Config {
    /// Return the first configured inbox entry, or None.
    pub fn inbox(&self) -> Option<&InboxEntry> {
        self.inbox.first()
    }

    /// Expand all file entries into resolved paths.
    /// Explicit Path entries are resolved directly.
    /// Glob entries are expanded using the glob crate.
    /// Returns a deduplicated list where explicit entries win over glob matches.
    pub fn resolve_files(&self) -> Vec<ResolvedFile> {
        let mut files = self
            .files
            .iter()
            .flat_map(|entry| match entry {
                FileEntry::Path { path, context } => {
                    vec![ResolvedFile {
                        path: path.clone(),
                        context: context.clone(),
                    }]
                }
                FileEntry::Glob { glob: pattern, context } => glob::glob(pattern)
                    .into_iter()
                    .flatten()
                    .filter_map(|p| p.ok())
                    .map(|path| ResolvedFile {
                        path,
                        context: context.clone(),
                    })
                    .collect(),
            })
            .collect::<Vec<_>>();

        let mut seen = HashSet::new();
        files.retain(|f| seen.insert(f.path.clone()));
        files
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedFile {
    pub path: PathBuf,
    pub context: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::result;
    use tempfile::tempdir;

    #[test]
    fn finds_tomb_toml_in_ancestor_directory() -> result::Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempdir()?;
        let root_dir = tmp_dir.path();
        let start_dir = root_dir.join("a/b/c");

        fs::write(
            root_dir.join(".tomb.toml"),
            r#"
              [[files]]
              path = "notes.md"

              [[files]]
              path = "~/home_notes.md"

              [[files]]
              glob = "*.md"
              context = "docs"

              [[inbox]]
              path = "inbox.md"
        "#,
        )?;
        fs::create_dir_all(&start_dir)?;

        let config = resolve_config(&start_dir, &[])?;

        assert_eq!(config.root_dir.unwrap(), root_dir);
        assert_eq!(
            config.files,
            vec![
                FileEntry::Path {
                    path: PathBuf::from("notes.md"),
                    context: None
                },
                FileEntry::Path {
                    path: PathBuf::from("~/home_notes.md"),
                    context: None
                },
                FileEntry::Glob {
                    glob: String::from("*.md"),
                    context: Some(String::from("docs"))
                }
            ]
        );
        assert_eq!(
            config.inbox,
            vec![InboxEntry {
                path: PathBuf::from("inbox.md")
            }]
        );

        Ok(())
    }

    #[test]
    fn resolve_files_expands_paths_and_globs() -> result::Result<(), Box<dyn std::error::Error>> {
        let tmp = tempdir()?;
        let root = tmp.path();

        fs::write(root.join("notes.md"), "")?;
        fs::write(root.join("todo.md"), "")?;
        fs::write(root.join("readme.txt"), "")?;

        let config = Config {
            files: vec![
                FileEntry::Path {
                    path: root.join("notes.md"),
                    context: Some("personal".into()),
                },
                FileEntry::Glob {
                    glob: root.join("*.md").to_string_lossy().into(),
                    context: None,
                },
            ],
            inbox: vec![],
            root_dir: Some(root.to_path_buf()),
        };

        let files = config.resolve_files();

        assert_eq!(files.iter().filter(|f| f.path == root.join("notes.md")).count(), 1);
        assert!(files.iter().any(|f| f.path == root.join("notes.md")));
        assert!(files.iter().any(|f| f.path == root.join("todo.md")));
        assert!(!files.iter().any(|f| f.path.extension().is_some_and(|e| e == "txt")));
        let notes = files.iter().find(|f| f.path == root.join("notes.md")).unwrap();
        assert_eq!(notes.context, Some("personal".into()));

        Ok(())
    }

    #[test]
    fn tilde_expansion() {
        let path = resolve_path(Path::new("/tmp"), Path::new("~/foo"));
        assert!(!path.starts_with("~"));
        assert!(path.starts_with(home_dir().unwrap()));
        assert!(path.ends_with("foo"))
    }
}

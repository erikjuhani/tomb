use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use crate::{
    error::{Result, TombError},
    model::ManagedFile,
    parser::parse_managed_file,
    renderer::render_managed_file,
};

pub fn read_source(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    file.lock_shared()?;

    let mut source = String::new();
    file.read_to_string(&mut source)?;
    Ok(source)
}

pub fn read_managed_file(path: &Path) -> Result<(ManagedFile, String)> {
    let source = read_source(path)?;

    let managed_file = parse_managed_file(&source)?;
    Ok((managed_file, source))
}

pub fn atomic_write(path: &Path, contents: &str) -> Result<()> {
    let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    let tmp = path.with_extension(format!("{}.tmp", ext));

    if let Err(error) = fs::write(&tmp, contents) {
        _ = fs::remove_file(&tmp);
        return Err(error.into());
    };

    fs::rename(tmp, path)?;
    Ok(())
}

pub fn write_managed_file(path: &Path, original_source: &str, managed_file: ManagedFile) -> Result<()> {
    let current_source = read_source(path)?;
    if current_source != original_source {
        return Err(TombError::Conflict(format!(
            "Source content was changed in {}",
            path.to_string_lossy()
        )));
    }

    let contents = render_managed_file(&managed_file);
    atomic_write(path, &contents)?;

    Ok(())
}

pub fn mutate_managed_file(path: &Path, mutate_fn: impl FnOnce(&mut ManagedFile) -> usize) -> Result<ManagedFile> {
    let (mut managed_file, source) = read_managed_file(path)?;
    mutate_fn(&mut managed_file);
    write_managed_file(path, &source, managed_file.clone())?;
    Ok(managed_file)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, result};

    use indoc::indoc;
    use tempfile::tempdir;

    use super::*;
    use crate::model::TaskStatus;

    const SAMPLE: &str = indoc! { r#"---
    tomb_mode: managed
    tomb_version: 1
    ---

    ## Today

    - [ ] Write the parser ^a1b2
    "#};

    fn write_file(dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, body).unwrap();
        path
    }

    #[test]
    fn reads_and_parses_a_file() -> result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = write_file(dir.path(), "tasks.md", SAMPLE);

        let (managed_file, source) = read_managed_file(&path)?;

        assert_eq!(source, SAMPLE);
        assert_eq!(managed_file.sections[0].tasks[0].id.as_deref(), Some("a1b2"));
        Ok(())
    }

    #[test]
    fn write_round_trips() -> result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = write_file(dir.path(), "tasks.md", SAMPLE);

        let (managed_file, source) = read_managed_file(&path)?;
        write_managed_file(&path, &source, managed_file.clone())?;

        assert_eq!(fs::read_to_string(&path)?, SAMPLE);
        let (reread, _) = read_managed_file(&path)?;
        assert_eq!(managed_file, reread);
        Ok(())
    }

    #[test]
    fn stale_source_reports_conflict() -> result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = write_file(dir.path(), "tasks.md", SAMPLE);

        let (managed_file, source) = read_managed_file(&path)?;
        fs::write(&path, format!("{SAMPLE}- [ ] Sneaky edit ^z9z9\n"))?;

        let result = write_managed_file(&path, &source, managed_file);

        assert!(matches!(result, Err(TombError::Conflict(_))));
        Ok(())
    }

    #[test]
    fn mutate_applies_closure_and_persists() -> result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = write_file(dir.path(), "tasks.md", SAMPLE);

        let updated = mutate_managed_file(&path, |file| {
            file.sections[0].tasks[0].status = TaskStatus::Done;
            0
        })?;

        assert_eq!(updated.sections[0].tasks[0].status, TaskStatus::Done);
        let (reread, _) = read_managed_file(&path)?;
        assert_eq!(reread.sections[0].tasks[0].status, TaskStatus::Done);
        Ok(())
    }

    #[test]
    #[cfg(unix)]
    fn atomic_write_preserves_original_on_failure() -> result::Result<(), Box<dyn std::error::Error>> {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir()?;
        let path = write_file(dir.path(), "tasks.md", SAMPLE);
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o555))?;

        let result = atomic_write(&path, "replacement");
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o755))?;

        assert!(result.is_err());
        assert_eq!(fs::read_to_string(&path)?, SAMPLE);
        Ok(())
    }
}

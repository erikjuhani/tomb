use std::{collections::HashSet, iter::repeat_with, path::PathBuf};

use rand::{Rng, RngExt, distr::Alphanumeric};

use crate::{
    config::Config,
    error::{Result, TombError},
    io::read_managed_file,
    model::{ManagedFile, Task},
};

pub(crate) fn generate_id(rng: &mut impl Rng, existing: &HashSet<String>) -> String {
    repeat_with(|| {
        rng.sample_iter(Alphanumeric)
            .take(4)
            .map(char::from)
            .collect::<String>()
    })
    .find(|id| !existing.contains(id))
    .expect("yield until new id is found")
}

pub(crate) fn resolve_prefix<'a>(prefix: &str, all_ids: &[&'a str]) -> Result<&'a str> {
    let mut candidates: Vec<&str> = all_ids.iter().copied().filter(|id| id.starts_with(prefix)).collect();
    candidates.sort();

    match candidates.as_slice() {
        [] => Err(TombError::IdNotFound(prefix.to_string())),
        [single] => Ok(*single),
        ids => ids
            .iter()
            .copied()
            .find(|id| *id == prefix)
            .ok_or_else(|| TombError::AmbiguousId {
                prefix: prefix.to_string(),
                matches: ids.iter().copied().map(String::from).collect(),
            }),
    }
}

pub(crate) fn collect_ids(file: &ManagedFile) -> Vec<String> {
    fn task_ids(tasks: &[Task]) -> Vec<String> {
        tasks
            .iter()
            .flat_map(|task| {
                task.id
                    .as_deref()
                    .into_iter()
                    .map(String::from)
                    .chain(task_ids(&task.children))
            })
            .collect()
    }

    file.sections
        .iter()
        .flat_map(|section| task_ids(&section.tasks))
        .collect()
}

pub(crate) fn resolve_id_across_files(prefix: &str, config: &Config) -> Result<(PathBuf, String)> {
    let managed_files = config
        .resolve_files()
        .iter()
        .flat_map(|resolved_file| {
            read_managed_file(&resolved_file.path).map(|(managed_file, _)| (resolved_file.path.clone(), managed_file))
        })
        .collect::<Vec<_>>();

    let entries: Vec<(PathBuf, String)> = managed_files
        .into_iter()
        .flat_map(|(path, file)| collect_ids(&file).into_iter().map(move |id| (path.clone(), id)))
        .collect();

    let ids: Vec<&str> = entries.iter().map(|(_, id)| id.as_str()).collect();
    let full = resolve_prefix(prefix, &ids)?;

    let (path, id) = entries
        .iter()
        .find(|(_, id)| id == full)
        .map(|(path, id)| (path.clone(), id.clone()))
        .expect("resolve_prefix returns an id present in entries");

    Ok((path, id))
}

pub(crate) fn assign_missing_ids(file: &mut ManagedFile) -> usize {
    let mut existing: HashSet<String> = collect_ids(file).into_iter().collect();

    file.sections
        .iter_mut()
        .map(|section| assign_ids(&mut section.tasks, &mut existing))
        .sum()
}

fn assign_ids(tasks: &mut [Task], existing: &mut HashSet<String>) -> usize {
    let mut assigned = 0;
    for task in tasks {
        if task.id.is_none() {
            let id = generate_id(&mut rand::rng(), existing);
            existing.insert(id.clone());
            task.id = Some(id);
            assigned += 1;
        }
        assigned += assign_ids(&mut task.children, existing);
    }
    assigned
}

pub(crate) fn find_duplicates(file: &ManagedFile) -> Vec<(String, usize)> {
    let mut ids = collect_ids(file);
    ids.sort();
    ids.chunk_by(|a, b| a == b)
        .filter(|chunk| chunk.len() > 1)
        .filter_map(|chunk| chunk.first().map(|id| (id.to_string(), chunk.len())))
        .collect()
}

pub(crate) fn repair_duplicates(file: &mut ManagedFile) -> usize {
    let mut seen: HashSet<String> = HashSet::new();
    file.sections
        .iter_mut()
        .map(|section| repair_tasks(&mut section.tasks, &mut seen))
        .sum()
}

fn repair_tasks(tasks: &mut [Task], seen: &mut HashSet<String>) -> usize {
    let mut reassigned = 0;
    for task in tasks {
        let duplicate = task.id.as_deref().is_some_and(|id| seen.contains(id));
        if duplicate {
            let new_id = generate_id(&mut rand::rng(), seen);
            seen.insert(new_id.clone());
            task.id = Some(new_id);
            reassigned += 1;
        } else if let Some(id) = &task.id {
            seen.insert(id.clone());
        }
        reassigned += repair_tasks(&mut task.children, seen);
    }
    reassigned
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs};

    use indoc::indoc;
    use rand::{SeedableRng, rngs::StdRng};
    use tempfile::tempdir;

    use crate::{
        config::{Config, FileEntry},
        error::TombError,
        id::{
            assign_missing_ids, collect_ids, find_duplicates, generate_id, repair_duplicates, resolve_id_across_files,
            resolve_prefix,
        },
        parser::parse_managed_file,
    };

    #[test]
    fn finds_ids_appearing_more_than_once() {
        let file = parse_managed_file(indoc! { r#"## Today

      - [ ] First ^dup
        - [ ] Inner duplicate ^dup
      - [ ] Another duplicate ^dup
      - [ ] Pair ^pair
      - [ ] Other pair ^pair
      - [ ] Unique ^uniq
      "#})
        .unwrap();

        assert_eq!(
            find_duplicates(&file),
            vec![(String::from("dup"), 3), (String::from("pair"), 2)]
        );
    }

    #[test]
    fn assigns_ids_to_tasks_missing_one_and_leaves_the_rest() {
        let mut file = parse_managed_file(indoc! { r#"## Today

      - [ ] Keeps its id ^keep
        - [ ] Nested without id
      - [ ] Sibling without id
      "#})
        .unwrap();

        let assigned = assign_missing_ids(&mut file);

        let ids = collect_ids(&file);
        assert_eq!(assigned, 2);
        assert!(ids.contains(&String::from("keep")));
        assert_eq!(ids.len(), 3, "every task should now have an id");
        assert_eq!(
            ids.iter().collect::<HashSet<_>>().len(),
            3,
            "assigned ids should be unique",
        );
    }

    #[test]
    fn collects_ids_depth_first_skipping_missing() -> Result<(), Box<dyn std::error::Error>> {
        let file = parse_managed_file(indoc! { r#"## Today

          - [ ] Parent ^p1
            - [ ] Child ^c1
              - [ ] Grandchild ^g1
          - [ ] Untracked task
          - [ ] Sibling ^s1

          ## Backlog

          - [ ] Backlog task ^b1
          "#})?;

        assert_eq!(collect_ids(&file), ["p1", "c1", "g1", "s1", "b1"]);

        Ok(())
    }

    #[test]
    fn test_generate_id() {
        let mut rng = StdRng::seed_from_u64(0);
        assert_eq!(generate_id(&mut rng, &HashSet::new()), "zujx")
    }

    #[test]
    fn test_generate_next_id_if_exists() {
        let mut rng = StdRng::seed_from_u64(0);
        assert_eq!(
            generate_id(&mut rng, &HashSet::from_iter([String::from("zujx")])),
            "zBql"
        )
    }

    #[test]
    fn exact_match_resolves() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(resolve_prefix("t3k9", &["a1b2", "t3k9"])?, "t3k9");
        Ok(())
    }

    #[test]
    fn unique_prefix_resolves() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(resolve_prefix("t3", &["a1b2", "t3k9"])?, "t3k9");
        Ok(())
    }

    #[test]
    fn ambiguous_prefix_reports_matches() {
        let err = resolve_prefix("a1", &["a1b2", "a1c3", "t3k9"]).unwrap_err();
        assert!(
            matches!(err, TombError::AmbiguousId { ref prefix, ref matches }if prefix == "a1" && matches == &["a1b2".to_string(), "a1c3".to_string()])
        );
    }

    #[test]
    fn missing_prefix_returns_not_found() {
        let err = resolve_prefix("xyz", &["a1b2", "t3k9"]).unwrap_err();
        assert!(matches!(err, TombError::IdNotFound(ref p) if p == "xyz"));
    }

    #[test]
    fn exact_match_wins_when_prefix_of_longer_ids() -> Result<(), Box<dyn std::error::Error>> {
        // Without the exact-first check this would be AmbiguousId.
        assert_eq!(resolve_prefix("t3k9", &["t3k9", "t3k9x"])?, "t3k9");
        Ok(())
    }

    #[test]
    fn find_duplicates_returns_empty_when_all_ids_unique() {
        let file = parse_managed_file(indoc! { r#"## Today

      - [ ] First ^a1b2
      - [ ] Second ^c3d4
      - [ ] Third ^e5f6
      "#})
        .unwrap();

        assert!(find_duplicates(&file).is_empty());
    }

    #[test]
    fn repair_keeps_first_occurrence_and_reassigns_subsequent_duplicates() {
        let mut file = parse_managed_file(indoc! { r#"## Today

      - [ ] First ^dup
        - [ ] Inner duplicate ^dup
      - [ ] Another duplicate ^dup
      - [ ] Unique ^uniq
      "#})
        .unwrap();

        let reassigned = repair_duplicates(&mut file);
        let ids = collect_ids(&file);

        assert_eq!(reassigned, 2);
        assert_eq!(ids[0], "dup", "first occurrence kept");
        assert!(ids.contains(&String::from("uniq")), "non-duplicate ids untouched");
        assert_eq!(
            ids.iter().collect::<HashSet<_>>().len(),
            ids.len(),
            "all ids now unique",
        );
    }

    #[test]
    fn resolves_id_across_multiple_files() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path_a = dir.path().join("a.md");
        let path_b = dir.path().join("b.md");
        fs::write(&path_a, "## Today\n\n- [ ] Alpha ^a1b2\n")?;
        fs::write(&path_b, "## Today\n\n- [ ] Beta ^b3c4\n")?;

        let config = Config {
            files: vec![
                FileEntry::Path {
                    path: path_b.clone(),
                    context: None,
                },
                FileEntry::Path {
                    path: path_a.clone(),
                    context: None,
                },
            ],
            inbox: vec![],
            root_dir: None,
        };

        let (path, id) = resolve_id_across_files("a1", &config)?;

        assert_eq!(path, path_a);
        assert_eq!(id, "a1b2");
        Ok(())
    }

    #[test]
    fn ambiguous_id_across_files_reports_matches() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path_a = dir.path().join("a.md");
        let path_b = dir.path().join("b.md");
        fs::write(&path_a, "## Today\n\n- [ ] Alpha ^a1b2\n")?;
        fs::write(&path_b, "## Today\n\n- [ ] Also ^a1c3\n")?;

        let config = Config {
            files: vec![
                FileEntry::Path {
                    path: path_a,
                    context: None,
                },
                FileEntry::Path {
                    path: path_b,
                    context: None,
                },
            ],
            inbox: vec![],
            root_dir: None,
        };

        let err = resolve_id_across_files("a1", &config).unwrap_err();

        assert!(matches!(err, TombError::AmbiguousId { .. }));
        Ok(())
    }
}

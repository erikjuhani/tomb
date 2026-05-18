use std::cmp::Reverse;
use std::fmt::Write;

use chrono::NaiveDate;

use crate::model::{FileMode, Frontmatter, ManagedFile, Section, SectionKind, Task, TaskStatus};

pub fn render_managed_file(file: &ManagedFile) -> String {
    let mut blocks: Vec<String> = vec![];

    if let Some(frontmatter) = render_frontmatter(&file.frontmatter) {
        blocks.push(frontmatter);
    }

    let mut sections: Vec<&Section> = file.sections.iter().collect();
    sections.sort_by_key(|section| section_sort_key(&section.kind));

    for section in sections {
        blocks.push(render_section(section));
    }

    blocks.join("\n")
}

fn section_sort_key(kind: &SectionKind) -> (u8, Option<Reverse<NaiveDate>>) {
    match kind {
        SectionKind::Today => (0, None),
        SectionKind::Backlog => (1, None),
        SectionKind::Date(d) => (2, Some(Reverse(*d))),
    }
}

fn render_frontmatter(frontmatter: &Frontmatter) -> Option<String> {
    let has_any = frontmatter.mode.is_some()
        || frontmatter.version.is_some()
        || frontmatter.context.is_some()
        || frontmatter.last_rollover.is_some()
        || !frontmatter.extras.is_empty();
    if !has_any {
        return None;
    }

    let mut out = String::from("---\n");
    if let Some(mode) = &frontmatter.mode {
        let value = match mode {
            FileMode::Managed => "managed",
            FileMode::Tracked => "tracked",
        };
        writeln!(out, "tomb_mode: {value}").unwrap();
    }
    if let Some(version) = frontmatter.version {
        writeln!(out, "tomb_version: {version}").unwrap();
    }
    if let Some(context) = &frontmatter.context {
        writeln!(out, "context: {context}").unwrap();
    }
    if let Some(last_rollover) = frontmatter.last_rollover {
        writeln!(out, "last_rollover: {}", last_rollover.format("%Y-%m-%d")).unwrap();
    }
    for (key, value) in &frontmatter.extras {
        writeln!(out, "{key}: {value}").unwrap();
    }
    out.push_str("---\n");
    Some(out)
}

fn render_section(section: &Section) -> String {
    let heading = match &section.kind {
        SectionKind::Today => String::from("Today"),
        SectionKind::Backlog => String::from("Backlog"),
        SectionKind::Date(d) => d.format("%Y-%m-%d").to_string(),
    };

    let mut out = format!("## {heading}\n");
    if section.tasks.is_empty() {
        return out;
    }
    out.push('\n');
    for task in &section.tasks {
        render_task(&mut out, task, 0);
    }
    out
}

fn render_task(out: &mut String, task: &Task, depth: usize) {
    let indent = "  ".repeat(depth);
    let marker = match task.status {
        TaskStatus::Todo => ' ',
        TaskStatus::InProgress => '/',
        TaskStatus::Done => 'x',
        TaskStatus::Cancelled => '-',
    };

    write!(out, "{indent}- [{marker}] {}", task.title).unwrap();
    if let Some(link) = &task.link {
        write!(out, " [{}]({})", link.label, link.url).unwrap();
    }
    if let Some(id) = &task.id {
        write!(out, " ^{id}").unwrap();
    }
    out.push('\n');

    if !task.description.is_empty() {
        render_description(out, &task.description, depth);
    }
    if !task.children.is_empty() {
        if !task.description.is_empty() {
            out.push('\n');
        }
        for child in &task.children {
            render_task(out, child, depth + 1);
        }
    }
}

fn render_description(out: &mut String, description: &str, depth: usize) {
    let cont_indent = "  ".repeat(depth + 1);
    for (i, line) in description.split('\n').enumerate() {
        if line.is_empty() {
            out.push('\n');
        } else if i == 0 {
            out.push_str(&cont_indent);
            out.push_str(line);
            out.push('\n');
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::model::{FileMode, Section, SectionKind, Task, TaskLink, TaskStatus};
    use crate::parser::parse_managed_file;

    use super::*;

    const ROUND_TRIP_STUB: &str = indoc! { r#"---
    tomb_mode: managed
    tomb_version: 1
    context: work
    last_rollover: 2026-02-25
    ---

    ## Today

    - [/] Design the API [org/basalt#42](https://github.com/org/basalt/issues/42) ^t3k9
      We need to finalize the REST API before the frontend team starts.
      Targeting OpenAPI 3.1 spec.

      ```yaml
      openapi: 3.1.0
      info:
        version: 1.0.0
        title: Example API
      ```

      - [x] Define endpoints ^e2f1
      - [ ] Write OpenAPI spec ^g3h4
        Should include request/response examples for every endpoint.

        - [x] Research format ^r1s2
        - [ ] Draft schema ^d3e4
      - [ ] Review with team ^i5j6
    - [ ] Write the parser ^a1b2

    ## Backlog

    - [ ] Write man page ^m2v7

    ## 2026-02-28

    - [ ] Prepare demo ^d4e5
      **IMPORTANT**: Book the large conference room.

    ## 2026-02-24

    - [x] Set up repository ^p8q1
    "#};

    #[test]
    fn round_trips_byte_for_byte() {
        let file = parse_managed_file(ROUND_TRIP_STUB).unwrap();
        assert_eq!(render_managed_file(&file), ROUND_TRIP_STUB);
    }

    #[test]
    fn round_trips_model_equality() {
        let parsed = parse_managed_file(ROUND_TRIP_STUB).unwrap();
        let rendered = render_managed_file(&parsed);
        let reparsed = parse_managed_file(&rendered).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn renders_no_frontmatter_when_all_fields_empty() {
        let file = ManagedFile {
            frontmatter: Frontmatter::default(),
            sections: vec![Section {
                kind: SectionKind::Today,
                tasks: vec![],
            }],
            source: String::new(),
        };
        assert_eq!(render_managed_file(&file), "## Today\n");
    }

    #[test]
    fn emits_unknown_frontmatter_keys_after_known_ones() {
        let mut extras = std::collections::BTreeMap::new();
        extras.insert(String::from("custom_key"), String::from("some value"));
        extras.insert(String::from("another_one"), String::from("42"));

        let file = ManagedFile {
            frontmatter: Frontmatter {
                mode: Some(FileMode::Managed),
                extras,
                ..Default::default()
            },
            sections: vec![],
            source: String::new(),
        };
        // BTreeMap iterates sorted: another_one < custom_key.
        assert_eq!(
            render_managed_file(&file),
            indoc! { r#"---
            tomb_mode: managed
            another_one: 42
            custom_key: some value
            ---
            "#}
        );
    }

    #[test]
    fn sorts_sections_today_backlog_then_dates_descending() {
        let file = ManagedFile {
            frontmatter: Frontmatter::default(),
            sections: vec![
                Section {
                    kind: SectionKind::Date(NaiveDate::from_str("2026-02-20").unwrap()),
                    tasks: vec![],
                },
                Section {
                    kind: SectionKind::Backlog,
                    tasks: vec![],
                },
                Section {
                    kind: SectionKind::Date(NaiveDate::from_str("2026-02-28").unwrap()),
                    tasks: vec![],
                },
                Section {
                    kind: SectionKind::Today,
                    tasks: vec![],
                },
            ],
            source: String::new(),
        };
        assert_eq!(
            render_managed_file(&file),
            indoc! { r#"## Today

            ## Backlog

            ## 2026-02-28

            ## 2026-02-20
            "#}
        );
    }

    #[test]
    fn renders_external_link_between_title_and_id() {
        let file = ManagedFile {
            frontmatter: Frontmatter::default(),
            sections: vec![Section {
                kind: SectionKind::Today,
                tasks: vec![Task {
                    id: Some(String::from("bug1")),
                    title: String::from("Fix the bug"),
                    link: Some(TaskLink {
                        label: String::from("proj#42"),
                        url: String::from("https://example.com/issues/42"),
                    }),
                    status: TaskStatus::Todo,
                    description: String::new(),
                    children: vec![],
                    source_range: 0..0,
                }],
            }],
            source: String::new(),
        };
        assert_eq!(
            render_managed_file(&file),
            indoc! { r#"## Today

            - [ ] Fix the bug [proj#42](https://example.com/issues/42) ^bug1
            "#}
        );
    }

    #[test]
    fn renders_only_present_frontmatter_fields() {
        let file = ManagedFile {
            frontmatter: Frontmatter {
                mode: Some(FileMode::Managed),
                context: Some(String::from("work")),
                ..Default::default()
            },
            sections: vec![],
            source: String::new(),
        };
        assert_eq!(
            render_managed_file(&file),
            indoc! { r#"---
            tomb_mode: managed
            context: work
            ---
            "#}
        );
    }

    #[test]
    fn round_trips_three_level_nesting_with_descriptions_at_each_level() {
        let source = indoc! { r#"## Today

        - [ ] Level one ^l1
          Description for level one.

          - [/] Level two ^l2
            Description for level two.

            - [x] Level three ^l3
              Description for level three.
        "#};

        let file = parse_managed_file(source).unwrap();
        assert_eq!(render_managed_file(&file), source);
    }
}

use std::ops::Range;

use chrono::NaiveDate;
use pulldown_cmark::{Event, MetadataBlockKind, Options, Tag};

use crate::{
    error::Result,
    model::{FileMode, Frontmatter, ManagedFile, Section, SectionKind, Task},
    task_parser::TaskParser,
};

pub fn parse_managed_file(input: &str) -> Result<ManagedFile> {
    Parser::new(input).run()
}

struct Parser<'a, I> {
    iter: I,
    source: &'a str,
    frontmatter: Frontmatter,
    sections: Vec<Section>,
    stack: Option<Section>,
}

pub(crate) fn extract_id(title: &mut String) -> Option<String> {
    let start_pattern = " ^";
    let pos = title.rfind(start_pattern)? + start_pattern.len();
    let id: String = title[pos..].chars().take_while(|c| c.is_alphanumeric()).collect();

    if id.is_empty() {
        None
    } else {
        title.truncate(pos.saturating_sub(start_pattern.len()));
        Some(id)
    }
}

type CmarkIter<'a> = pulldown_cmark::TextMergeWithOffset<'a, pulldown_cmark::OffsetIter<'a>>;

impl<'a> Parser<'a, CmarkIter<'a>> {
    pub fn new(source: &'a str) -> Self {
        let mut options = Options::all();
        options.remove(Options::ENABLE_SMART_PUNCTUATION);

        let iter = pulldown_cmark::TextMergeWithOffset::new(
            pulldown_cmark::Parser::new_ext(source, options).into_offset_iter(),
        );

        Self {
            iter,
            source,
            frontmatter: Frontmatter::default(),
            sections: vec![],
            stack: None,
        }
    }

    fn run(mut self) -> Result<ManagedFile> {
        while let Some((event, range)) = self.iter.next() {
            match event {
                Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => self.parse_frontmatter(),
                Event::Start(Tag::Heading { .. }) => self.parse_heading(),
                Event::Start(Tag::Item) => {
                    if let Some((task, section)) = self.parse_task(range).zip(self.stack.as_mut()) {
                        section.tasks.push(task);
                    }
                }
                _ => {}
            }
        }
        self.finish_section();

        Ok(ManagedFile {
            frontmatter: self.frontmatter,
            source: self.source.to_string(),
            sections: self.sections,
        })
    }

    fn finish_section(&mut self) {
        if let Some(section) = self.stack.take() {
            self.sections.push(section);
        }
    }

    fn parse_date_heading(heading_str: &str) -> Option<NaiveDate> {
        // FIXME: Use result instead
        NaiveDate::parse_from_str(heading_str, "%Y-%m-%d").ok()
    }

    fn classify_heading(heading_str: &str) -> Option<SectionKind> {
        match heading_str {
            "Today" => Some(SectionKind::Today),
            "Backlog" => Some(SectionKind::Backlog),
            str => Parser::parse_date_heading(str).map(SectionKind::Date),
        }
    }

    fn parse_task(&mut self, range: Range<usize>) -> Option<Task> {
        TaskParser::new(self.source).parse(&mut self.iter, range)
    }

    fn parse_heading(&mut self) {
        self.finish_section();
        for (event, _) in self.iter.by_ref() {
            match event {
                Event::Text(text) => {
                    if let Some(kind) = Parser::classify_heading(text.as_ref()) {
                        self.stack = Some(Section { kind, tasks: vec![] });
                    }
                }
                Event::End(_) => break,
                _ => {}
            }
        }
    }

    fn parse_frontmatter(&mut self) {
        let mut yaml = String::new();

        for (event, _) in self.iter.by_ref() {
            match event {
                Event::Text(text) => yaml.push_str(&text),
                Event::End(_) => break,
                _ => {}
            }
        }

        for line in yaml.lines() {
            let Some((key, value)) = line.split_once(':').map(|(key, value)| (key.trim(), value.trim())) else {
                continue;
            };

            match key {
                "tomb_mode" => {
                    // TODO: impl From?
                    self.frontmatter.mode = match value {
                        "managed" => Some(FileMode::Managed),
                        "tracked" => Some(FileMode::Tracked),
                        _ => None,
                    };
                }
                "tomb_version" => self.frontmatter.version = value.parse().ok(),
                "context" => self.frontmatter.context = Some(value.to_string()),
                "last_rollover" => self.frontmatter.last_rollover = value.parse().ok(),
                other => {
                    self.frontmatter.extras.insert(other.to_string(), value.to_string());
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, result, str::FromStr};

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::model::{TaskLink, TaskStatus};

    use super::*;

    const TEST_MD_STUB: &str = indoc! { r#"---
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
      Invite list: engineering, product, design.

      ```sql
      SELECT * FROM rooms WHERE capacity > 20;
      ```

      - [ ] Book room ^b1r2

    ## 2026-02-24

    - [x] Set up repository ^p8q1
    "#};

    #[test]
    fn test_parser_run() -> result::Result<(), Box<dyn std::error::Error>> {
        let mut options = Options::all();
        options.remove(Options::ENABLE_SMART_PUNCTUATION);
        let file = Parser::new(TEST_MD_STUB).run()?;

        assert_eq!(
            file.frontmatter,
            Frontmatter {
                mode: Some(FileMode::Managed),
                version: Some(1),
                context: Some(String::from("work")),
                last_rollover: Some(NaiveDate::from_str("2026-02-25")?),
                ..Default::default()
            }
        );

        assert_eq!(
            file.sections,
            vec![
                Section {
                    kind: SectionKind::Today,
                    tasks: vec![
                        Task {
                            id: Some(String::from("t3k9")),
                            title: String::from("Design the API"),
                            link: Some(TaskLink {
                                label: String::from("org/basalt#42"),
                                url: String::from("https://github.com/org/basalt/issues/42"),
                            }),
                            status: TaskStatus::InProgress,
                            description: String::from(
                                "We need to finalize the REST API before the frontend team starts.\n  Targeting OpenAPI 3.1 spec.\n\n  ```yaml\n  openapi: 3.1.0\n  info:\n    version: 1.0.0\n    title: Example API\n  ```"
                            ),
                            children: vec![
                                Task {
                                    id: Some(String::from("e2f1")),
                                    title: String::from("Define endpoints"),
                                    link: None,
                                    status: TaskStatus::Done,
                                    description: String::new(),
                                    children: vec![],
                                    source_range: 363..392,
                                },
                                Task {
                                    id: Some(String::from("g3h4")),
                                    title: String::from("Write OpenAPI spec"),
                                    link: None,
                                    status: TaskStatus::Todo,
                                    description: String::from(
                                        "Should include request/response examples for every endpoint."
                                    ),
                                    children: vec![
                                        Task {
                                            id: Some(String::from("r1s2")),
                                            title: String::from("Research format"),
                                            link: None,
                                            status: TaskStatus::Done,
                                            description: String::new(),
                                            children: vec![],
                                            source_range: 495..523,
                                        },
                                        Task {
                                            id: Some(String::from("d3e4")),
                                            title: String::from("Draft schema"),
                                            link: None,
                                            status: TaskStatus::Todo,
                                            description: String::new(),
                                            children: vec![],
                                            source_range: 527..552,
                                        },
                                    ],
                                    source_range: 394..552,
                                },
                                Task {
                                    id: Some(String::from("i5j6")),
                                    title: String::from("Review with team"),
                                    link: None,
                                    status: TaskStatus::Todo,
                                    description: String::new(),
                                    children: vec![],
                                    source_range: 554..583,
                                },
                            ],
                            source_range: 94..583,
                        },
                        Task {
                            id: Some(String::from("a1b2")),
                            title: String::from("Write the parser"),
                            link: None,
                            status: TaskStatus::Todo,
                            description: String::new(),
                            children: vec![],
                            source_range: 583..613,
                        },
                    ]
                },
                Section {
                    kind: SectionKind::Backlog,
                    tasks: vec![Task {
                        id: Some(String::from("m2v7")),
                        title: String::from("Write man page"),
                        link: None,
                        status: TaskStatus::Todo,
                        description: String::new(),
                        children: vec![],
                        source_range: 625..653,
                    }]
                },
                Section {
                    kind: SectionKind::Date(NaiveDate::from_str("2026-02-28")?),
                    tasks: vec![Task {
                        id: Some(String::from("d4e5")),
                        title: String::from("Prepare demo"),
                        link: None,
                        status: TaskStatus::Todo,
                        description: String::from(
                            "**IMPORTANT**: Book the large conference room.\n  Invite list: engineering, product, design.\n\n  ```sql\n  SELECT * FROM rooms WHERE capacity > 20;\n  ```"
                        ),
                        children: vec![Task {
                            id: Some(String::from("b1r2")),
                            title: String::from("Book room"),
                            link: None,
                            status: TaskStatus::Todo,
                            description: String::new(),
                            children: vec![],
                            source_range: 849..872,
                        }],
                        source_range: 668..872,
                    }]
                },
                Section {
                    kind: SectionKind::Date(NaiveDate::from_str("2026-02-24")?),
                    tasks: vec![Task {
                        id: Some(String::from("p8q1")),
                        title: String::from("Set up repository"),
                        link: None,
                        status: TaskStatus::Done,
                        description: String::new(),
                        children: vec![],
                        source_range: 887..917,
                    }]
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn infers_managed_mode_from_headings() {
        let source = indoc! { r#"## Today

        - [ ] Write the parser ^a1b2
        "#};

        let file = Parser::new(source).run().unwrap();
        assert_eq!(file.frontmatter.mode, None);
        assert_eq!(file.mode(), FileMode::Managed);
    }

    #[test]
    fn falls_back_to_tracked_mode() {
        let source = indoc! { r#"# Some notes

        Just a plain markdown file with no tomb sections.
        "#};

        let file = Parser::new(source).run().unwrap();
        assert_eq!(file.frontmatter.mode, None);
        assert_eq!(file.mode(), FileMode::Tracked);
    }

    #[test]
    fn explicit_mode_overrides_inference() {
        let source = indoc! { r#"---
        tomb_mode: tracked
        ---

        ## Today
        "#};

        let file = Parser::new(source).run().unwrap();
        assert_eq!(file.frontmatter.mode, Some(FileMode::Tracked));
        assert_eq!(file.mode(), FileMode::Tracked);
    }

    fn parse(source: &str) -> crate::model::ManagedFile {
        Parser::new(source).run().unwrap()
    }

    #[test]
    fn missing_frontmatter_yields_default_fields() {
        let file = parse("Just some text, no tomb structure.\n");

        // No `tomb_mode:` was written, so the field stays `None` in the
        // model. Effective mode comes from `file.mode()`.
        assert_eq!(file.frontmatter, Frontmatter::default());
        assert_eq!(file.mode(), FileMode::Tracked);
    }

    #[test]
    fn captures_unknown_frontmatter_keys() -> result::Result<(), Box<dyn std::error::Error>> {
        let source = indoc! { r#"---
        tomb_mode: managed
        custom_key: some value
        another_one: 42
        ---
        "#};

        let file = Parser::new(source).run()?;
        assert_eq!(
            file.frontmatter.extras,
            BTreeMap::from([
                (String::from("custom_key"), String::from("some value")),
                (String::from("another_one"), String::from("42")),
            ])
        );
        Ok(())
    }

    #[test]
    fn parses_all_task_markers() {
        let file = parse(indoc! { r#"## Today

        - [ ] Todo task ^t001
        - [/] In progress task ^t002
        - [x] Done task ^t003
        - [-] Cancelled task ^t004
        "#});

        let statuses: Vec<_> = file.sections[0].tasks.iter().map(|task| task.status.clone()).collect();
        assert_eq!(
            statuses,
            vec![
                TaskStatus::Todo,
                TaskStatus::InProgress,
                TaskStatus::Done,
                TaskStatus::Cancelled,
            ]
        );
    }

    #[test]
    fn extracts_block_ids() {
        let file = parse(indoc! { r#"## Today

        - [ ] Task with id ^abc123
        - [ ] Task without id
        "#});

        let tasks = &file.sections[0].tasks;
        assert_eq!(tasks[0].id.as_deref(), Some("abc123"));
        assert_eq!(tasks[0].title, "Task with id");
        assert_eq!(tasks[1].id, None);
        assert_eq!(tasks[1].title, "Task without id");
    }

    #[test]
    fn extracts_external_link() {
        let file = parse(indoc! { r#"## Today

        - [ ] Fix the bug [proj#42](https://example.com/issues/42) ^bug1
        "#});

        let task = &file.sections[0].tasks[0];
        assert_eq!(task.title, "Fix the bug");
        assert_eq!(task.id.as_deref(), Some("bug1"));
        assert_eq!(
            task.link,
            Some(TaskLink {
                label: String::from("proj#42"),
                url: String::from("https://example.com/issues/42"),
            })
        );
    }

    #[test]
    fn parses_three_level_nesting_with_descriptions() {
        let file = parse(indoc! { r#"## Today

        - [ ] Level one ^l1
          Description for level one.

          - [/] Level two ^l2
            Description for level two.

            - [x] Level three ^l3
              Description for level three.
        "#});

        let l1 = &file.sections[0].tasks[0];
        assert_eq!(l1.title, "Level one");
        assert_eq!(l1.description, "Description for level one.");

        let l2 = &l1.children[0];
        assert_eq!(l2.title, "Level two");
        assert_eq!(l2.status, TaskStatus::InProgress);
        assert_eq!(l2.description, "Description for level two.");

        let l3 = &l2.children[0];
        assert_eq!(l3.title, "Level three");
        assert_eq!(l3.status, TaskStatus::Done);
        assert_eq!(l3.description, "Description for level three.");
        assert!(l3.children.is_empty());
    }

    #[test]
    fn parses_code_block_in_description() {
        let file = parse(indoc! { r#"## Today

        - [ ] Task with code ^code1
          Here is some code:

          ```rust
          fn main() {}
          ```
        "#});

        let task = &file.sections[0].tasks[0];
        assert_eq!(task.title, "Task with code");
        assert_eq!(
            task.description,
            "Here is some code:\n\n  ```rust\n  fn main() {}\n  ```"
        );
        assert!(task.children.is_empty());
    }

    #[test]
    fn parses_multi_paragraph_description() {
        let file = parse(indoc! { r#"## Today

        - [ ] Multi paragraph ^mp1
          First paragraph of the description.

          Second paragraph after a blank line.
        "#});

        let task = &file.sections[0].tasks[0];
        assert_eq!(
            task.description,
            "First paragraph of the description.\n\n  Second paragraph after a blank line."
        );
    }

    #[test]
    fn source_ranges_map_back_to_original() {
        let source = indoc! { r#"## Today

        - [ ] Standalone task ^solo
        - [/] Parent task ^par1
          - [x] Child task ^chld
        "#};

        let file = Parser::new(source).run().unwrap();
        let tasks = &file.sections[0].tasks;

        assert_eq!(&source[tasks[0].source_range.clone()], "- [ ] Standalone task ^solo\n");

        let parent = &tasks[1];
        assert_eq!(
            &source[parent.source_range.clone()],
            "- [/] Parent task ^par1\n  - [x] Child task ^chld\n"
        );

        let child = &parent.children[0];
        assert_eq!(&source[child.source_range.clone()], "- [x] Child task ^chld\n");
    }
}

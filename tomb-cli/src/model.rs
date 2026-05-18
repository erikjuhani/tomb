use std::collections::BTreeMap;
use std::ops::Range;

use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaskLink {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: Option<String>,
    pub title: String,
    pub link: Option<TaskLink>,
    pub status: TaskStatus,
    pub description: String,
    pub children: Vec<Task>,
    pub source_range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SectionKind {
    Today,
    Backlog,
    Date(NaiveDate),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    pub kind: SectionKind,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileMode {
    Managed,
    Tracked,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Frontmatter {
    pub mode: Option<FileMode>,
    pub version: Option<u8>,
    pub context: Option<String>,
    pub last_rollover: Option<NaiveDate>,
    pub extras: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ManagedFile {
    pub frontmatter: Frontmatter,
    pub sections: Vec<Section>,
    // TODO: Rope?
    pub source: String,
}

impl ManagedFile {
    pub fn mode(&self) -> FileMode {
        if let Some(mode) = &self.frontmatter.mode {
            return mode.clone();
        }
        if self.sections.is_empty() {
            FileMode::Tracked
        } else {
            FileMode::Managed
        }
    }
}

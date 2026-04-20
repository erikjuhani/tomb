use std::ops::Range;

use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskMarker {
    Todo,
    InProgress,
    Done,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    id: Option<String>,
    title: String,
    marker: TaskMarker,
    description: Vec<String>,
    children: Vec<Task>,
    source_range: Option<Range<usize>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SectionKind {
    Today,
    Backlog,
    Date(NaiveDate),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    kind: SectionKind,
    tasks: Vec<Task>,
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
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ManagedFile {
    pub frontmatter: Frontmatter,
    pub sections: Vec<Section>,
    // TODO: Rope?
    pub source: String,
}

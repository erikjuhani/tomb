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

#[derive(Debug, Clone, PartialEq)]
pub struct Frontmatter {
    mode: Option<FileMode>,
    version: Option<u8>,
    context: Option<String>,
    last_rollover: Option<NaiveDate>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ManagedFile {
    frontmatter: Frontmatter,
    sections: Vec<Section>,
    // TODO: Rope?
    source: String,
}

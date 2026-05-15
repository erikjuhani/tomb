use std::ops::Range;

use pulldown_cmark::{Event, Tag, TagEnd};

use crate::{
    model::{Task, TaskLink, TaskStatus},
    parser::extract_id,
};

#[derive(Clone, Copy, Debug)]
enum Phase {
    Marker,
    Title,
    Description,
    Children,
}

type CmarkIter<'a> = pulldown_cmark::TextMergeWithOffset<'a, pulldown_cmark::OffsetIter<'a>>;

pub struct TaskParser<'a> {
    source: &'a str,
    marker: Option<TaskStatus>,
    title: String,
    link: Option<TaskLink>,
    children: Vec<Task>,
    desc_start: Option<usize>,
    desc_end: usize,
    source_range: Range<usize>,
}

impl<'a> TaskParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            marker: None,
            title: String::new(),
            link: None,
            children: vec![],
            desc_start: None,
            desc_end: 0,
            source_range: 0..0,
        }
    }

    pub fn parse(mut self, iter: &mut CmarkIter, item_range: Range<usize>) -> Option<Task> {
        self.source_range = item_range;

        let mut phase = Phase::Marker;
        let mut in_link = false;
        while let Some((event, range)) = iter.next() {
            match (phase, event) {
                (_, Event::End(TagEnd::Item)) => break,

                (Phase::Marker, Event::ExtendedTaskListMarker(c)) => {
                    self.marker = match c {
                        ' ' => Some(TaskStatus::Todo),
                        '/' => Some(TaskStatus::InProgress),
                        'x' | 'X' => Some(TaskStatus::Done),
                        '-' => Some(TaskStatus::Cancelled),
                        _ => None,
                    };
                    phase = Phase::Title;
                }
                (Phase::Marker, Event::TaskListMarker(done)) => {
                    self.marker = Some(if done { TaskStatus::Done } else { TaskStatus::Todo });
                    phase = Phase::Title;
                }

                (Phase::Title, Event::Text(text)) => match self.link {
                    Some(ref mut link) if in_link => link.label.push_str(&text),
                    _ => self.title.push_str(&text),
                },
                (Phase::Title, Event::Start(Tag::Link { dest_url, .. })) => {
                    self.link = Some(TaskLink {
                        label: String::new(),
                        url: dest_url.to_string(),
                    });
                    in_link = true;
                }
                (Phase::Title, Event::End(TagEnd::Link)) => in_link = false,
                (Phase::Title, Event::SoftBreak | Event::HardBreak | Event::End(TagEnd::Paragraph)) => {
                    phase = Phase::Description;
                }
                (Phase::Title, Event::Start(Tag::List(_))) => phase = Phase::Children,

                (Phase::Description, Event::Start(Tag::List(_))) => phase = Phase::Children,
                (Phase::Description, Event::Start(Tag::Paragraph) | Event::End(TagEnd::Paragraph)) => {}
                (Phase::Description, _) => {
                    self.desc_start.get_or_insert(range.start);
                    self.desc_end = self.desc_end.max(range.end);
                }

                (Phase::Children, Event::Start(Tag::Item)) => {
                    if let Some(child) = TaskParser::new(self.source).parse(iter, range) {
                        self.children.push(child);
                    }
                }
                (Phase::Children, Event::End(TagEnd::List(_))) => phase = Phase::Description,

                _ => {}
            }
        }

        self.build_task()
    }

    fn build_task(self) -> Option<Task> {
        let status = self.marker?;
        let mut title = self.title;
        let id = extract_id(&mut title);
        let description = self
            .desc_start
            .map(|start| self.source[start..self.desc_end].trim().to_string())
            .unwrap_or_default();

        Some(Task {
            title: title.trim().to_string(),
            link: self.link,
            status,
            id,
            description,
            children: self.children,
            source_range: self.source_range,
        })
    }
}

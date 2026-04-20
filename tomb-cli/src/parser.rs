use std::ops::Range;

use pulldown_cmark::{Event, MetadataBlockKind, Options, Tag};

use crate::{
    error::Result,
    model::{FileMode, Frontmatter, ManagedFile, Section},
};

struct Parser<'a, I> {
    iter: I,
    source: &'a str,
    frontmatter: Frontmatter,
    sections: Vec<Section>,
}

type CmarkIter<'a> = pulldown_cmark::TextMergeWithOffset<'a, pulldown_cmark::OffsetIter<'a>>;

impl<'a> Parser<'a, CmarkIter<'a>> {
    fn new(source: &'a str) -> Self {
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
        }
    }

    fn run(mut self) -> Result<ManagedFile> {
        while let Some((event, _)) = self.iter.next() {
            match event {
                Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => self.parse_frontmatter(),
                _ => {}
            }
        }
        Ok(ManagedFile {
            frontmatter: self.frontmatter,
            source: self.source.to_string(),
            sections: vec![],
        })
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
            let Some((key, value)) = line.split_once(':') else {
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
                _ => {}
            };
        }
    }
}

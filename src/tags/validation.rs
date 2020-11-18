use crate::diagnostic::Diagnostic;

use super::{Tag, TagType};
use std::collections::HashMap;

static MUTUALLY_EXCLUSIVE: &[(TagType, TagType)] = &[
    // Kind tags
    (TagType::Property, TagType::Function),
    (TagType::Property, TagType::Class),
    (TagType::Function, TagType::Class),
    // Class
    (TagType::Class, TagType::Within),
    // Param doesn't work with kinds other than function
    (TagType::Param, TagType::Property),
    (TagType::Param, TagType::Class),
    (TagType::Param, TagType::Type),
    // Return doesn't work with kinds other than function
    (TagType::Return, TagType::Property),
    (TagType::Return, TagType::Class),
    (TagType::Return, TagType::Type),
    // Field is exclusive with function
    (TagType::Field, TagType::Function),
    // Properties can't error
    (TagType::Error, TagType::Property),
];

#[allow(unused)]
static ALLOW_MULTIPLE: &[TagType] = &[
    TagType::Param,
    TagType::Return,
    TagType::Tag,
    TagType::Field,
];

pub fn validate_tags(tags: &[Tag]) -> Vec<Diagnostic> {
    let mut tag_map: HashMap<TagType, usize> = HashMap::new();

    for tag in tags {
        let entry = tag_map.entry(tag.tag_type());

        *entry.or_insert(0usize) += 1;
    }

    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    for (left, right) in MUTUALLY_EXCLUSIVE {
        if tag_map.get(left).is_some() && tag_map.get(right).is_some() {
            let mut iter = tags.iter().filter(|tag| {
                let tag_type = tag.tag_type();

                &tag_type == left || &tag_type == right
            });
            let first = iter.next().unwrap();
            let other_tags: Vec<_> = iter.collect();

            let mut diagnostic = first.diagnostic("This tag is mutually exclusive...");

            for tag in other_tags {
                diagnostic.attach_diagnostic(tag.diagnostic("...with this tag."));
            }

            diagnostics.push(diagnostic);
        }
    }

    diagnostics
}

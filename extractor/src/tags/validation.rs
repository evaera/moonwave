use crate::diagnostic::Diagnostic;

use super::{Tag, TagType};
use std::collections::{BTreeMap, HashMap};

static MUTUALLY_EXCLUSIVE: &[(TagType, TagType)] = &[
    // Kind tags
    (TagType::Property, TagType::Function),
    (TagType::Property, TagType::Class),
    (TagType::Function, TagType::Class),
    // Classes aren't within other classes
    (TagType::Class, TagType::Within),
    // __index only works on classes
    (TagType::Index, TagType::Property),
    (TagType::Index, TagType::Function),
    (TagType::Index, TagType::Type),
    // Param doesn't work with kinds other than function
    (TagType::Param, TagType::Property),
    (TagType::Param, TagType::Class),
    (TagType::Param, TagType::Type),
    // Return doesn't work with kinds other than function
    (TagType::Return, TagType::Property),
    (TagType::Return, TagType::Class),
    (TagType::Return, TagType::Type),
    // Field is exclusive with function
    // (TagType::Field, TagType::Function),
    // Properties can't error or yield
    (TagType::Error, TagType::Property),
    (TagType::Yields, TagType::Property),
    // Classes can't error or yield
    (TagType::Error, TagType::Class),
    (TagType::Yields, TagType::Class),
    // Can't be unreleased and released at the same time
    (TagType::Unreleased, TagType::Since),
    (TagType::Unreleased, TagType::Deprecated),
    // Readonly doesn't make sense on a function
    (TagType::Function, TagType::ReadOnly),
];

static DEPENDENT_TAGS: &[(TagType, TagType)] = &[
    (TagType::Property, TagType::Within),
    (TagType::Type, TagType::Within),
];

static ALLOW_MULTIPLE: &[TagType] = &[
    TagType::Param,
    TagType::Return,
    TagType::Custom,
    TagType::Field,
    TagType::Error,
    TagType::External,
];

fn build_diagnostic(
    tags: &[Tag],
    types: &[&TagType],
    primary: &str,
    secondary: &str,
) -> Diagnostic {
    let mut iter = tags.iter().filter(|tag| types.contains(&&tag.tag_type()));

    let first = iter.next().unwrap();
    let other_tags: Vec<_> = iter.collect();

    let mut diagnostic = first.diagnostic(primary);

    if secondary.is_empty() {
        return diagnostic;
    }

    for tag in other_tags {
        diagnostic.attach_diagnostic(tag.diagnostic(secondary));
    }

    diagnostic
}

pub fn validate_global_tags(tags: &[Tag]) -> Vec<Diagnostic> {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    let mut name_occurrences: BTreeMap<(TagType, String), (usize, Vec<Tag>)> = BTreeMap::new();

    for tag in tags {
        let name = match tag {
            Tag::External(external_tag) => external_tag.name,
            Tag::Class(class_tag) => class_tag.name,
            _ => continue,
        }
        .to_string();

        let entry = name_occurrences
            .entry((tag.tag_type(), name))
            .or_insert((0usize, vec![]));
        entry.0 += 1;
        entry.1.push(tag.clone());
    }

    for ((tag_type, _), (count, tags)) in name_occurrences {
        if count <= 1usize {
            continue;
        }

        diagnostics.push(build_diagnostic(
            &tags,
            &[&tag_type],
            "This tag cannot be used multiple times with the same name.",
            "Appears here",
        ))
    }

    diagnostics
}

pub fn validate_tags(tags: &[Tag]) -> Vec<Diagnostic> {
    let mut tag_map: HashMap<TagType, usize> = HashMap::new();

    for tag in tags {
        let entry = tag_map.entry(tag.tag_type());

        *entry.or_insert(0usize) += 1;
    }

    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    for (left, right) in MUTUALLY_EXCLUSIVE {
        if tag_map.contains_key(left) && tag_map.contains_key(right) {
            diagnostics.push(build_diagnostic(
                tags,
                &[left, right],
                "This tag is mutually exclusive...",
                "...with this tag.",
            ));
        }
    }

    for (depender, dependee) in DEPENDENT_TAGS {
        if tag_map.contains_key(depender) && !tag_map.contains_key(dependee) {
            diagnostics.push(build_diagnostic(
                tags,
                &[depender, dependee],
                &format!(
                    "The @{} tag must also be present when using this tag.",
                    format!("{:?}", dependee).to_ascii_lowercase()
                ),
                "",
            ))
        }
    }

    for (tag_type, occurrences) in tag_map {
        if occurrences > 1 && !ALLOW_MULTIPLE.contains(&tag_type) {
            diagnostics.push(build_diagnostic(
                tags,
                &[&tag_type],
                "This tag cannot appear multiple times in a single doc entry.",
                "Appears here",
            ))
        }
    }

    diagnostics
}

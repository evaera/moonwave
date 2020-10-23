#![feature(bool_to_option)]

use anyhow::Result;
use full_moon::{
    self,
    ast::*,
    node::Node,
    tokenizer::{Token, TokenType},
};
use std::{borrow::Cow, convert::TryFrom, fs};
use walkdir::{self, WalkDir};

mod doc_entry;
mod parse_error;
mod tags;

use doc_entry::{DocEntry, DocEntryKind};

fn extract_doc_comments<'a>(stmt: &'a Stmt<'a>) -> Vec<DocEntry> {
    let mut tokens: Vec<Cow<Token>> = stmt.surrounding_trivia().0;

    tokens.retain(|t| matches!(t.token_type(), TokenType::MultiLineComment { blocks: 1, .. }));

    tokens
        .iter()
        .map(|t| match t.token_type() {
            TokenType::MultiLineComment { comment, .. } => comment,
            _ => panic!("Can't deal with this type of token"),
        })
        .map(|comment| {
            DocEntry::parse(
                comment.clone().into_owned(),
                DocEntryKind::try_from(stmt).ok(),
            )
            .unwrap()
        })
        .collect::<Vec<DocEntry>>()
}

fn generate_for_file(source_code: &str) -> Result<()> {
    let ast = full_moon::parse(&source_code).unwrap();

    let mut comments: Vec<DocEntry> = vec![];

    for stmt in ast.nodes().iter_stmts() {
        comments.append(&mut extract_doc_comments(stmt));
    }

    dbg!(comments);

    Ok(())
}

fn generate_docs() -> Result<()> {
    let walker = WalkDir::new("test-input").follow_links(true).into_iter();
    for entry in walker
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".lua"))
    {
        generate_for_file(&fs::read_to_string(entry.path())?)?;
    }

    Ok(())
}

fn main() {
    generate_docs().expect("Oh");
}

use anyhow::Result;
use full_moon::{
    self,
    ast::*,
    node::Node,
    tokenizer::{Token, TokenType},
};
use parse_error::{ParseError, ParseErrors};
// TODO: Don't blob import
use std::{borrow::Cow, convert::TryFrom, fs};
use walkdir::{self, WalkDir};

mod doc_entry;
mod parse_error;
mod tags;

use doc_entry::DocEntry;

fn extract_doc_comments<'a>(stmt: &'a Stmt<'a>) -> Vec<Result<DocEntry, ParseErrors>> {
    let tokens: Vec<Cow<Token>> = stmt.surrounding_trivia().0;

    tokens
        .iter()
        .filter_map(|t| match t.token_type() {
            TokenType::MultiLineComment { comment, blocks: 1 } => Some(comment),
            _ => None,
        })
        .map(|comment| DocEntry::parse(comment.clone().into_owned(), &stmt))
        .collect::<Vec<_>>()
}

fn generate_for_file(source_code: &str) -> Result<()> {
    let ast = full_moon::parse(&source_code).unwrap();

    let mut comments: Vec<Result<DocEntry, ParseErrors>> = vec![];

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
    match generate_docs() {
        Ok(_) => return,
        Err(error) => eprintln!("{}", error),
    };

    std::process::exit(1);
}

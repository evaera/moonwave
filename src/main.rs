use full_moon::{
    self,
    ast::*,
    node::Node,
    tokenizer::{Token, TokenType},
};
use parse_error::ParseErrors;
// TODO: Don't blob import
use codespan_reporting::files::SimpleFiles;
use std::{borrow::Cow, fmt, fs};
use walkdir::{self, WalkDir};

mod doc_comment;
mod doc_entry;
mod parse_error;
mod tags;

use doc_comment::DocComment;
use doc_entry::DocEntry;

#[derive(Debug)]
pub enum Error {
    ParseErrors(ParseErrors),
    ReadError(std::io::Error),
    FullMoonError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseErrors(parse_error) => write!(formatter, "{}", parse_error),
            Error::ReadError(read_error) => write!(formatter, "{}", read_error),
            Error::FullMoonError(full_moon_error) => write!(formatter, "{}", full_moon_error),
        }
    }
}

impl std::error::Error for Error {}

fn extract_doc_comments<'a>(stmt: &'a Stmt<'a>) -> Vec<Result<DocEntry, ParseErrors>> {
    let tokens: Vec<Cow<Token>> = stmt.surrounding_trivia().0;

    tokens
        .into_iter()
        .filter_map(|t| match t.token_type() {
            TokenType::MultiLineComment { blocks: 1, .. } => Some(DocComment::new(t, stmt, 1usize)),
            _ => None,
        })
        .map(|comment| DocEntry::parse(&comment))
        .collect::<Vec<_>>()
}

fn generate_for_file<'a>(
    files: &'a SimpleFiles<String, String>,
    file_id: usize,
) -> Result<(), Error> {
    let source = files.get(file_id).unwrap().source();
    let ast = full_moon::parse(source).map_err(|e| Error::FullMoonError(e.to_string()))?;

    let mut comments: Vec<Result<DocEntry, ParseErrors>> = vec![];

    for stmt in ast.nodes().iter_stmts() {
        comments.append(&mut extract_doc_comments(stmt));
    }

    dbg!(comments);

    Ok(())
}

fn generate_docs() -> Result<(), Error> {
    let mut files = SimpleFiles::new();

    let walker = WalkDir::new("test-input").follow_links(true).into_iter();
    for entry in walker
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".lua"))
    {
        let path = entry.path();
        let contents = fs::read_to_string(path).map_err(Error::ReadError)?;

        let file_id = files.add(path.to_string_lossy().to_string(), contents);
        generate_for_file(&files, file_id)?;
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

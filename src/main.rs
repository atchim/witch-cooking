//! Experimental multilingual code formatter based on [Tree-Sitter]'s [query].
//!
//! [Tree-Sitter]: https://tree-sitter.github.io/tree-sitter
//! [query]: https://tree-sitter.github.io/tree-sitter/using-parsers#query-syntax

#![allow(dead_code)]
#![deny(elided_lifetimes_in_paths, missing_docs)]

#[cfg(not(any(feature = "rust")))]
compile_error!("no language to support");

mod cli;
mod editor;
mod err;
mod node_utils;
mod settings;

use {
  self::{
    cli::Opts,
    editor::Editor,
    err::Error,
    node_utils::{Matches, Provider as NodesProvider},
  },
  error_stack::{bail, IntoReport, Result, ResultExt},
  ropey::{iter::Chunks, Rope, RopeSlice},
  std::{fs, io, process::ExitCode},
  tree_sitter::{
    Language,
    Node,
    Parser,
    Query,
    QueryCursor,
    TextProvider,
    Tree,
  },
};

#[inline]
fn text(opts: &Opts) -> Result<Rope, Error> {
  Ok(match &opts.src {
    None => match atty::isnt(atty::Stream::Stdin) {
      false => bail!(Error::Pipe),
      true => Rope::from_reader(io::BufReader::new(io::stdin()))
        .report()
        .attach_printable("failed to create rope")
        .change_context(Error::Other)?,
    },
    Some(path) => Rope::from_reader(io::BufReader::new(
      fs::File::open(path).report().change_context(Error::SrcFile)?,
    ))
    .report()
    .attach_printable("failed to create rope")
    .change_context(Error::Other)?,
  })
}

#[inline]
fn lang(opts: &Opts) -> Result<Language, Error> {
  Ok(Language::from(match opts.lang {
    None => match opts.src.as_ref().and_then(detect_lang::from_path) {
      Some(lang) => {
        log::info!("auto detected {} language", lang.name());
        lang.try_into().report()?
      }
      _ => bail!(Error::LangUndetected),
    },
    Some(lang) => lang,
  }))
}

#[inline]
fn parse(text: &Rope, parser: &mut Parser) -> Result<Tree, Error> {
  parser
    .parse_with(
      &mut |byte_ix, _| {
        let (s, chunk_byte_ix, ..) = text.chunk_at_byte(byte_ix);
        &s[byte_ix - chunk_byte_ix..]
      },
      None,
    )
    .ok_or(Error::Other)
    .report()
    .attach_printable("failed to parse source")
}

#[inline]
fn query(opts: &Opts, lang: Language) -> Result<Query, Error> {
  let src = fs::read_to_string(&opts.query)
    .report()
    .change_context(Error::QueryFile)?;
  Ok(Query::new(lang, &src).map_err(Error::Query)?)
}

struct ChunksBytes<'a>(Chunks<'a>);

impl<'a> Iterator for ChunksBytes<'a> {
  type Item = &'a [u8];

  fn next(&mut self) -> Option<Self::Item> { self.0.next().map(str::as_bytes) }
}

#[derive(Clone)]
struct RopeProvider<'a>(RopeSlice<'a>);

impl<'a> TextProvider<'a> for RopeProvider<'a> {
  type I = ChunksBytes<'a>;

  fn text(&mut self, node: Node<'_>) -> Self::I {
    ChunksBytes(self.0.byte_slice(node.byte_range()).chunks())
  }
}

#[inline]
fn cook(opts: &Opts) -> Result<Rope, Error> {
  let lang = lang(opts)?;
  let mut parser = Parser::new();
  parser.set_language(lang).map_err(Error::Lang)?;
  let text = text(opts)?;
  let tree = parse(&text, &mut parser)?;
  let query = query(opts, lang)?;
  let mut cursor = QueryCursor::new();
  let editor = Editor::from(text.clone());

  let matches = Matches::from(cursor.matches(
    &query,
    tree.root_node(),
    RopeProvider(text.slice(..)),
  ));

  for (pat_ix, cap_ix_to_nodes_slice) in matches.iter() {
    log::trace!("applying pattern #{pat_ix}");
    for _nodes_provider in
      cap_ix_to_nodes_slice.iter().map(|cap_ix_to_nodes| {
        NodesProvider::new(cap_ix_to_nodes, matches.id_to_node())
      })
    {
      for _query_prop in query.property_settings(pat_ix).iter() {}

      for _query_predicate in query.general_predicates(pat_ix) {}
    }
  }

  Ok(editor.into())
}

#[inline]
fn handle(res: Result<Rope, Error>) -> ExitCode {
  match res {
    Err(err) => {
      eprintln!("{err}");
      log::error!("{err:?}");
      ExitCode::FAILURE
    }
    Ok(text) => {
      println!("{text}");
      ExitCode::SUCCESS
    }
  }
}

fn main() -> ExitCode {
  env_logger::init();
  let opts = <Opts as clap::Parser>::parse();
  handle(cook(&opts))
}

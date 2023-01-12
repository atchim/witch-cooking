mod cpl;
pub mod indent_rule;
mod parser;

pub use {cpl::Cpl, indent_rule::IndentRule, parser::Error};
use {
  fnv::FnvHashMap,
  std::{collections::hash_map::Entry, marker::PhantomData},
  tree_sitter::Node,
};

#[derive(Default)]
pub struct Global<'a> {
  cpl: Cpl,
  indent_style: Option<&'a str>,
}

impl<'a> Global<'a> {
  #[inline]
  pub fn cpl(&self) -> Cpl { self.cpl }

  #[inline]
  pub fn set_cpl(&mut self, cpl: impl Into<Cpl>) -> Cpl {
    let old_cpl = self.cpl;
    self.cpl = cpl.into();
    old_cpl
  }

  #[inline]
  pub fn indent_style(&self) -> Option<&'a str> { self.indent_style }

  #[inline]
  pub fn set_indent_style(&mut self, style: &'a str) -> Option<&'a str> {
    self.indent_style.replace(style)
  }
}

#[derive(Default)]
pub struct Local<'a> {
  cpl: Cpl,
  ignore_query: Option<&'a str>,
  indent_style: Option<&'a str>,
}

impl<'a> Local<'a> {
  #[inline]
  pub fn cpl(&self) -> Cpl { self.cpl }

  #[inline]
  pub fn set_cpl(&mut self, cpl: impl Into<Cpl>) -> Cpl {
    let old_cpl = self.cpl;
    self.cpl = cpl.into();
    old_cpl
  }

  #[inline]
  pub fn ignore_query(&self) -> Option<&'a str> { self.ignore_query }

  #[inline]
  pub fn set_ignore_query(&mut self, style: &'a str) -> Option<&'a str> {
    self.ignore_query.replace(style)
  }

  #[inline]
  pub fn indent_style(&self) -> Option<&'a str> { self.indent_style }

  #[inline]
  pub fn set_indent_style(&mut self, style: &'a str) -> Option<&'a str> {
    self.indent_style.replace(style)
  }
}

#[derive(Default)]
pub struct NodeSettings<'tree> {
  ignored: bool,
  indent_rule: Option<IndentRule>,
  _phantom: PhantomData<&'tree ()>,
}

impl<'tree> NodeSettings<'tree> {
  #[inline]
  pub fn ignored(&self) -> bool { self.ignored }

  #[inline]
  pub fn ignore(&mut self, cond: bool) -> bool {
    let old_val = self.ignored;
    self.ignored = cond;
    old_val
  }

  #[inline]
  pub fn indent_rule(&self) -> Option<IndentRule> { self.indent_rule }

  #[inline]
  pub fn set_indent_rule(&mut self, rule: IndentRule) -> Option<IndentRule> {
    self.indent_rule.replace(rule)
  }
}

#[derive(Default)]
pub struct NodeToSettings<'tree>(FnvHashMap<usize, NodeSettings<'tree>>);

impl<'tree> NodeToSettings<'tree> {
  #[inline]
  pub fn entry(
    &mut self,
    node: &Node<'tree>,
  ) -> Entry<'_, usize, NodeSettings<'tree>> {
    self.0.entry(node.id())
  }

  #[inline]
  pub fn get(&self, node: &Node<'tree>) -> Option<&NodeSettings<'tree>> {
    self.0.get(&node.id())
  }
}

enum Scope {
  Global,
  Local,
}

#[derive(Default)]
pub struct Settings<'a, 'tree> {
  global: Global<'a>,
  local: Local<'a>,
  node_to_settings: NodeToSettings<'tree>,
}

impl<'a, 'tree> Settings<'a, 'tree> {
  #[inline]
  pub fn cpl(&self) -> Cpl {
    let cpl = self.local.cpl();
    match cpl.value() {
      None => self.global.cpl(),
      Some(_) => cpl,
    }
  }

  #[inline]
  fn set_cpl(&mut self, cpl: impl Into<Cpl>, scope: Scope) -> Cpl {
    match scope {
      Scope::Global => self.global.set_cpl(cpl),
      Scope::Local => self.local.set_cpl(cpl),
    }
  }

  #[inline]
  pub fn indent_style(&self) -> Option<&'a str> {
    self.local.indent_style().or_else(|| self.global.indent_style())
  }

  #[inline]
  fn set_indent_style(
    &mut self,
    style: &'a str,
    scope: Scope,
  ) -> Option<&'a str> {
    match scope {
      Scope::Global => self.global.set_indent_style(style),
      Scope::Local => self.local.set_indent_style(style),
    }
  }
}

mod err;
mod ignored;
mod indent_rule;
mod indent_style;
mod setting;

use {
  crate::node_utils::Provider as NodesProvider,
  fnv::FnvHashMap,
  std::{collections::hash_map::Entry, marker::PhantomData},
  tree_sitter::{Node, QueryProperty},
};
pub use {
  err::Error,
  indent_rule::{IndentRule, IndentRuleErr},
  setting::Setting,
};

#[derive(Default)]
pub struct MatchSettings {
  indent_style: Option<String>,
}

impl MatchSettings {
  #[inline]
  pub fn indent_style(&self) -> Option<&str> { self.indent_style.as_deref() }

  #[inline]
  pub fn set_indent_style(&mut self, style: &str) -> Option<String> {
    self.indent_style.replace(style.into())
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

type SettingsInner = FnvHashMap<&'static str, &'static dyn Setting>;

pub struct Settings(SettingsInner);

impl Settings {
  pub fn apply<'tree>(
    &self,
    query_prop: &QueryProperty,
    nodes_provider: &NodesProvider<'_, 'tree>,
    node_to_settings: &mut NodeToSettings<'tree>,
    match_settings: &mut MatchSettings,
  ) -> Result<(), Error> {
    let key = query_prop.key.as_ref();
    self.0.get(key).ok_or_else(|| Error::key(key))?.apply(
      query_prop,
      nodes_provider,
      node_to_settings,
      match_settings,
    )
  }
}

impl Default for Settings {
  fn default() -> Self {
    let mut inner = SettingsInner::default();

    macro_rules! insert_settings {
      ($($setting:path),+ $(,)?) => {
        $({
          let setting = &$setting;
          inner.insert(setting.name(), setting);
        })+
      };
    }

    insert_settings!(
      ignored::Ignored,
      indent_rule::IndentRuleSetting,
      indent_style::IndentStyle,
    );

    Settings(inner)
  }
}

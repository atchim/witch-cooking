use tree_sitter::{Node, TreeCursor};

pub struct Item<'tree> {
  node: Node<'tree>,
  depth: usize,
  field: Option<&'static str>,
}

impl<'tree> Item<'tree> {
  #[inline]
  pub fn depth(&self) -> usize {
    self.depth
  }

  #[inline]
  pub fn node(&self) -> &Node<'tree> {
    &self.node
  }

  #[inline]
  pub fn field(&self) -> Option<&'static str> {
    self.field
  }
}

impl<'tree> From<Item<'tree>> for Node<'tree> {
  #[inline]
  fn from(item: Item<'tree>) -> Self {
    item.node
  }
}

#[allow(clippy::from_over_into)]
impl<'tree> Into<(Node<'tree>, usize, Option<&'static str>)> for Item<'tree> {
  fn into(self) -> (Node<'tree>, usize, Option<&'static str>) {
    (self.node, self.depth, self.field)
  }
}

#[allow(clippy::from_over_into)]
impl<'a, 'tree> Into<(&'a Node<'tree>, usize, Option<&'static str>)>
  for &'a Item<'tree>
{
  fn into(self) -> (&'a Node<'tree>, usize, Option<&'static str>) {
    (&self.node, self.depth, self.field)
  }
}

pub struct Walker<'cursor, 'tree> {
  cursor: &'cursor mut TreeCursor<'tree>,
  exhausted: bool,
  depth: usize,
}

impl<'cursor, 'tree> Walker<'cursor, 'tree> {
  #[inline]
  fn up_next(&mut self) {
    let index = self.cursor.node().end_byte() + 1;
    loop {
      if self.depth == 0 {
        self.exhausted = true;
        break;
      }

      if self.cursor.goto_parent() {
        self.depth -= 1;
        if self.cursor.goto_first_child_for_byte(index).is_some() {
          self.depth += 1;
          break;
        }
      }
    }
  }

  #[inline]
  fn walk(&mut self) {
    if self.cursor.goto_first_child() {
      self.depth += 1;
    } else if self.depth == 0 {
      self.exhausted = true;
    } else if self.cursor.goto_next_sibling() {
    } else {
      self.up_next();
    }
  }
}

impl<'cursor, 'tree> From<&'cursor mut TreeCursor<'tree>>
  for Walker<'cursor, 'tree>
{
  #[inline]
  fn from(cursor: &'cursor mut TreeCursor<'tree>) -> Self {
    Walker { cursor, exhausted: false, depth: 0 }
  }
}

impl<'cursor, 'tree> Iterator for Walker<'cursor, 'tree> {
  type Item = Item<'tree>;
  fn next(&mut self) -> Option<Self::Item> {
    match self.exhausted {
      false => {
        let node = self.cursor.node();
        let field = self.cursor.field_name();
        let depth = self.depth;
        let item = Item { node, depth, field };
        self.walk();
        Some(item)
      }
      true => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use {super::*, tree_sitter::Parser};

  #[test]
  fn walker() {
    let text = "fn foo() { bar(); }";

    let mut parser = Parser::new();
    parser.set_language(tree_sitter_rust::language()).unwrap();
    let tree = parser.parse(text, None).unwrap();
    let mut cursor = tree.walk();
    let mut walker = Walker::from(&mut cursor);

    let mut test_next = |kind, depth, field| {
      let item = walker.next().unwrap();
      assert_eq!(kind, item.node().kind());
      assert_eq!(depth, item.depth());
      assert_eq!(field, item.field());
    };

    test_next("source_file", 0, None);
    test_next("function_item", 1, None);
    test_next("fn", 2, None);
    test_next("identifier", 2, Some("name"));
    test_next("parameters", 2, Some("parameters"));
    test_next("(", 3, None);
    test_next(")", 3, None);
    test_next("block", 2, Some("body"));
    test_next("{", 3, None);
    test_next("expression_statement", 3, None);
    test_next("call_expression", 4, None);
    test_next("identifier", 5, Some("function"));
    test_next("arguments", 5, Some("arguments"));
    test_next("(", 6, None);
    test_next(")", 6, None);
    test_next(";", 4, None);
    test_next("}", 3, None);

    assert!(walker.next().is_none());
  }
}

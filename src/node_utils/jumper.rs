use {std::collections::VecDeque, tree_sitter::Node};

pub struct Item<'tree> {
  node: Node<'tree>,
  depth: usize,
}

impl<'tree> Item<'tree> {
  #[inline]
  pub fn depth(&self) -> usize { self.depth }

  #[inline]
  pub fn node(&self) -> &Node<'tree> { &self.node }
}

impl<'tree> From<Item<'tree>> for Node<'tree> {
  #[inline]
  fn from(item: Item<'tree>) -> Self { item.node }
}

#[allow(clippy::from_over_into)]
impl<'tree> Into<(Node<'tree>, usize)> for Item<'tree> {
  fn into(self) -> (Node<'tree>, usize) { (self.node, self.depth) }
}

#[allow(clippy::from_over_into)]
impl<'a, 'tree> Into<(&'a Node<'tree>, usize)> for &'a Item<'tree> {
  fn into(self) -> (&'a Node<'tree>, usize) { (&self.node, self.depth) }
}

pub struct Jumper<'tree> {
  node: Node<'tree>,
  children: VecDeque<(Node<'tree>, usize)>,
  depth: usize,
  exhausted: bool,
}

impl<'tree> Jumper<'tree> {
  #[inline]
  fn walk(&mut self) {
    if let Some(child) = self.node.child(0) {
      self.children.push_back((child, self.depth + 1));
    }

    if let Some(sib) = self.node.next_sibling() {
      self.node = sib;
    } else if let Some((child, depth)) = self.children.pop_front() {
      self.node = child;
      self.depth = depth;
    } else {
      self.exhausted = true;
    }
  }
}

impl<'tree> From<Node<'tree>> for Jumper<'tree> {
  #[inline]
  fn from(cursor: Node<'tree>) -> Self {
    Jumper {
      node: cursor,
      children: VecDeque::default(),
      depth: 0,
      exhausted: false,
    }
  }
}

impl<'tree> Iterator for Jumper<'tree> {
  type Item = Item<'tree>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.exhausted {
      false => {
        let node = self.node;
        let depth = self.depth;
        let item = Item { node, depth };
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
  fn jumper() {
    let text = "fn foo() { bar(); }";

    let mut parser = Parser::new();
    parser.set_language(tree_sitter_rust::language()).unwrap();
    let tree = parser.parse(text, None).unwrap();
    let mut walker = Jumper::from(tree.root_node());

    let mut test_next = |kind, depth| {
      let item = walker.next().unwrap();
      assert_eq!(kind, item.node().kind());
      assert_eq!(depth, item.depth());
    };

    test_next("source_file", 0);
    test_next("function_item", 1);
    test_next("fn", 2);
    test_next("identifier", 2);
    test_next("parameters", 2);
    test_next("block", 2);
    test_next("(", 3);
    test_next(")", 3);
    test_next("{", 3);
    test_next("expression_statement", 3);
    test_next("}", 3);
    test_next("call_expression", 4);
    test_next(";", 4);
    test_next("identifier", 5);
    test_next("arguments", 5);
    test_next("(", 6);
    test_next(")", 6);

    assert!(walker.next().is_none());
  }
}

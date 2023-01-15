use {
  ropey::{Rope, RopeSlice},
  tree_sitter::{InputEdit, Node, Point, Range},
};

#[inline]
pub fn end_point(
  chars: impl Iterator<Item = char>,
  start: Option<&Point>,
) -> Point {
  let mut end = start.copied().unwrap_or_default();
  chars.for_each(|ch| match ch {
    '\n' => {
      end.row += 1;
      end.column = 0;
    }
    _ => end.column += ch.len_utf8(),
  });
  end
}

#[derive(Clone)]
pub struct Editor {
  src: Rope,
  edits: Vec<InputEdit>,
}

impl Editor {
  #[inline]
  fn push(&mut self, edit: InputEdit) { self.edits.push(edit); }

  pub fn insert(
    &mut self,
    start_byte: usize,
    start_position: &Point,
    s: &str,
  ) {
    let edit = InputEdit {
      start_byte,
      old_end_byte: start_byte,
      new_end_byte: start_byte + s.len(),
      start_position: *start_position,
      old_end_position: *start_position,
      new_end_position: end_point(s.chars(), Some(start_position)),
    };
    self.push(edit);
    let char_ix = self.src.byte_to_char(start_byte);
    self.src.insert(char_ix, s);
  }

  pub fn remove(&mut self, range: &Range) {
    let start_byte = range.start_byte;
    let old_end_byte = range.end_byte;
    let edit = InputEdit {
      start_byte,
      old_end_byte,
      new_end_byte: range.start_byte,
      start_position: range.start_point,
      old_end_position: range.end_point,
      new_end_position: range.start_point,
    };
    self.push(edit);
    let start_char = self.src.byte_to_char(start_byte);
    let end_char = self.src.byte_to_char(old_end_byte);
    self.src.remove(start_char..end_char);
  }

  pub fn replace(&mut self, range: &Range, s: &str) {
    let start_byte = range.start_byte;
    let start_position = range.start_point;
    let edit = InputEdit {
      start_byte,
      old_end_byte: range.end_byte,
      new_end_byte: start_byte + s.len(),
      start_position,
      old_end_position: range.end_point,
      new_end_position: end_point(s.chars(), Some(&start_position)),
    };
    self.push(edit);
    let start_char = self.src.byte_to_char(range.start_byte);
    let end_char = self.src.byte_to_char(range.end_byte);
    self.src.remove(start_char..end_char);
    self.src.insert(start_char, s);
  }

  #[inline]
  pub fn src(&self) -> RopeSlice<'_> { self.src.slice(..) }

  #[inline]
  pub fn sync(&self, node: &mut Node<'_>) {
    self.edits.iter().for_each(|edit| node.edit(edit));
  }

  #[inline]
  pub fn sync_last(&self, node: &mut Node<'_>) {
    if let Some(edit) = self.edits.last() {
      node.edit(edit);
    }
  }

  #[inline]
  pub fn sync_non_last(&self, node: &mut Node<'_>) {
    if self.edits.len() > 1 {
      self
        .edits
        .iter()
        .take(self.edits.len() - 1)
        .for_each(|edit| node.edit(edit));
    }
  }
}

impl From<Editor> for Rope {
  fn from(editor: Editor) -> Self { editor.src }
}

impl From<Rope> for Editor {
  fn from(src: Rope) -> Self { Self { src, edits: vec![] } }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    crate::node_utils::Walker,
    std::mem::zeroed,
    tree_sitter::{Parser, Tree, TreeCursor},
    tree_sitter_rust::language as rust_lang,
  };

  #[test]
  fn end_point_() {
    let test = |s: &str, end| assert_eq!(end_point(s.chars(), None), end);
    test("💣💥", Point { row: 0, column: 8 });
    test("\n", Point { row: 1, column: 0 });
    test("\r\u{b}\u{c}\u{85}\u{2028}\u{2029}", Point { row: 0, column: 11 });
  }

  fn with_src<F>(src: &str, f: F)
  where
    for<'tree> F: FnOnce(&mut TreeCursor<'tree>, &'tree Tree, &mut Editor),
  {
    let mut parser = Parser::new();
    parser.set_language(rust_lang()).unwrap();
    let tree = parser.parse(src, None).unwrap();
    let mut cursor = unsafe { zeroed::<TreeCursor<'_>>() };
    let mut editor = Editor::from(Rope::from_str(src));
    f(&mut cursor, &tree, &mut editor);
  }

  fn test_op<'tree, N, O>(
    cursor: &mut TreeCursor<'tree>,
    root_node: Node<'tree>,
    editor: &mut Editor,
    op: O,
    byte_ix: usize,
    node_ix: N,
    src: &str,
  ) where
    N: Fn(&Rope, &Node<'tree>) -> (usize, Point),
    O: FnOnce(&mut Editor),
  {
    cursor.reset(root_node);
    let old_rope = editor.src.clone();
    op(editor);
    Walker::from(cursor)
      .filter_map(|item| {
        let mut node = item.into();
        editor.sync_non_last(&mut node);
        (node.start_byte() >= byte_ix).then_some(node)
      })
      .for_each(|mut node| {
        let old_node = node;
        editor.sync_last(&mut node);
        let (node_start_byte, node_start_point) =
          node_ix(&old_rope, &old_node);
        assert_eq!(node_start_byte, node.start_byte());
        assert_eq!(node_start_point, node.start_position());
      });
    assert_eq!(src, editor.src);
  }

  #[test]
  fn editor_insert() {
    with_src("fn f()\n{}", |cursor, tree, editor| {
      let mut test_insert = |start_byte, start_point, s, src| {
        test_op(
          cursor,
          tree.root_node(),
          editor,
          |editor| editor.insert(start_byte, start_point, s),
          start_byte,
          |old_src, old_node| {
            let old_node_start_byte = old_node.start_byte();
            let node_start_byte = old_node_start_byte + s.len();
            let chars = s.chars().chain(
              old_src.byte_slice(start_byte..old_node_start_byte).chars(),
            );
            let node_start_point = end_point(chars, Some(start_point));
            (node_start_byte, node_start_point)
          },
          src,
        );
      };

      test_insert(
        5,
        &Point { row: 0, column: 5 },
        "bar: Bar",
        "fn f(bar: Bar)\n{}",
      );

      test_insert(
        4,
        &Point { row: 0, column: 4 },
        "oo",
        "fn foo(bar: Bar)\n{}",
      );

      test_insert(
        18,
        &Point { row: 1, column: 1 },
        " baz(); ",
        "fn foo(bar: Bar)\n{ baz(); }",
      );

      test_insert(
        0,
        &Point { row: 0, column: 0 },
        "pub\n",
        "pub\nfn foo(bar: Bar)\n{ baz(); }",
      );
    });
  }

  #[test]
  fn editor_remove() {
    with_src("pub\nfn foo(bar: Bar)\n{ baz(); }", |cursor, tree, editor| {
      let mut test_remove = |range: &Range, src| {
        test_op(
          cursor,
          tree.root_node(),
          editor,
          |editor| editor.remove(range),
          range.end_byte,
          |old_src, old_node| {
            let diff = range.end_byte - range.start_byte;
            let old_node_start_byte = old_node.start_byte();
            let node_start_byte = old_node_start_byte - diff;
            let chars =
              old_src.byte_slice(range.end_byte..old_node_start_byte).chars();
            let node_start_point = end_point(chars, Some(&range.start_point));
            (node_start_byte, node_start_point)
          },
          src,
        );
      };

      test_remove(
        &Range {
          start_byte: 11,
          end_byte: 19,
          start_point: Point { row: 1, column: 7 },
          end_point: Point { row: 1, column: 15 },
        },
        "pub\nfn foo()\n{ baz(); }",
      );

      test_remove(
        &Range {
          start_byte: 14,
          end_byte: 22,
          start_point: Point { row: 2, column: 1 },
          end_point: Point { row: 2, column: 9 },
        },
        "pub\nfn foo()\n{}",
      );

      test_remove(
        &Range {
          start_byte: 0,
          end_byte: 4,
          start_point: Point { row: 0, column: 0 },
          end_point: Point { row: 1, column: 0 },
        },
        "fn foo()\n{}",
      );

      test_remove(
        &Range {
          start_byte: 4,
          end_byte: 6,
          start_point: Point { row: 0, column: 4 },
          end_point: Point { row: 0, column: 6 },
        },
        "fn f()\n{}",
      );
    });
  }

  #[test]
  fn editor_replace() {
    with_src("pub\nfn foo(bar: Bar)\n{ baz(); }", |cursor, tree, editor| {
      let mut test_replace = |range: &Range, s, src| {
        test_op(
          cursor,
          tree.root_node(),
          editor,
          |editor| editor.replace(range, s),
          range.end_byte,
          |old_src, old_node| {
            let old_node_start_byte = old_node.start_byte();
            let diff = range.end_byte - range.start_byte;
            let slen = s.len();
            let node_start_byte = match slen > diff {
              false => old_node_start_byte - (diff - slen),
              true => old_node_start_byte + slen - diff,
            };
            let chars = s.chars().chain(
              old_src.byte_slice(range.end_byte..old_node_start_byte).chars(),
            );
            let node_start_point = end_point(chars, Some(&range.start_point));
            (node_start_byte, node_start_point)
          },
          src,
        );
      };

      test_replace(
        &Range {
          start_byte: 11,
          end_byte: 19,
          start_point: Point { row: 1, column: 7 },
          end_point: Point { row: 1, column: 15 },
        },
        "",
        "pub\nfn foo()\n{ baz(); }",
      );

      test_replace(
        &Range {
          start_byte: 15,
          end_byte: 21,
          start_point: Point { row: 2, column: 2 },
          end_point: Point { row: 2, column: 8 },
        },
        "\"foo\"",
        "pub\nfn foo()\n{ \"foo\" }",
      );

      test_replace(
        &Range {
          start_byte: 12,
          end_byte: 13,
          start_point: Point { row: 1, column: 8 },
          end_point: Point { row: 2, column: 0 },
        },
        " -> &'static str ",
        "pub\nfn foo() -> &'static str { \"foo\" }",
      );

      test_replace(
        &Range {
          start_byte: 0,
          end_byte: 4,
          start_point: Point { row: 0, column: 0 },
          end_point: Point { row: 1, column: 0 },
        },
        "",
        "fn foo() -> &'static str { \"foo\" }",
      );
    });
  }
}

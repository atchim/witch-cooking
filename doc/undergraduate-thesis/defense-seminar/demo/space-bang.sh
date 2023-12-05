EXT=rs

SRC=$(cat <<EOF
fn x_plus_y() -> u32 {
  let x = 5; let y = 11;

  x + y
}
EOF
)

QUERY=$(cat <<EOF
(function_item
  body: (block (_) @item . (_) @next)
  (#space! "\n" 2 @item @next))
EOF
)
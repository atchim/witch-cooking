EXT=rs

SRC=$(cat <<EOF
fn x_plus_y() -> u32 {
  let x = 5; let y = 11;

  x + y
}
EOF
)

QUERY=$(cat <<EOF
(#set! indent-style "  ")

(function_item
  body: (block (_) @item . (_) @next)
  (#space! "\n" 2 @item @next)
  (#set! @next indent-rule "+1")
  (#indent! @next))
EOF
)
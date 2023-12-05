EXT=rs
SRC='fn foo() {bar()}'

QUERY=$(cat <<EOF
(#set! indent-style "    ")

( (function_item
    body: (block (_) @item "}" @close)) @fn
  (#set! @item indent-rule "+1")
  (#indent-offset! @close @fn)
  (#indent! @item @close))
EOF
)
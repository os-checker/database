include "./file-tree/shared";

# 按照 target_triple 分组
. as $root | gen_key | group_by(.target_triple) | map({
    target_triple: .[0].target_triple,
    data: basic($root) | { kinds_order: $root | kinds_order, data: . }
  })

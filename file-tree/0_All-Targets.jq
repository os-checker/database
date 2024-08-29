include "./file-tree/shared";

# 不按照 target_triple 分组
. as $root | gen_key | basic($root) | { kinds_order: $root | kinds_order, data: . }

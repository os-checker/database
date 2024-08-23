include "./home/shared";

# 按照 target_triple 分类的总计数
. as $x | extract_kind_count | group_by(.target_triple) | map({
  target_triple: .[0].target_triple,
  data: group_by_package | group_by_repo | epilogue($x)
})


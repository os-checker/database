include "./home/shared";

# 不按照 target_triple 分类的总计数
. as $x | extract_kind_count | group_by_package | group_by_repo | epilogue($x)


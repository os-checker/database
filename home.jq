import "./jq/utils" as utils;

def extract_kind_count: . as $x | .data | map({key: {cmd_idx, kind}}) | group_by(.key) # 按诊断类型分组
| map({
    key: .[0].key,
    count: . | length,
  } | . + { cmd: $x.cmd[.key.cmd_idx] }
    | . + { pkg: $x.env.packages[.cmd.package_idx] }
    | {
      key: {
        user: .pkg.repo.user,
        repo: .pkg.repo.repo,
        package: .pkg.name,
      },
      kind: .key.kind,
      count,
    }
);

def group_by_package: . | group_by(.key) | map({
  key1: .[0].key,
  total_count: map(.count) | add,
  kinds: map({kind, count})
} # + (map({kind, count} | {(.kind): .count}) | add) 
  | . + { key2: { user: .key1.user, repo: .key1.repo } }
);

def group_by_repo: . | group_by(.key2) | map({
  children: map(del(.key2)),
  total_count: map(.total_count) | add
} + .[0].key2 # 从每个 children 数组元素中删除 key2，并把它展开到 children 数组外
  | . + {
    # 聚合所有 children （也就是 packages）的 kind 及其计数：
    # .children | map(.kinds) => 筛选 kinds，得到二维数组，最外层表示每个 package，最内层表示每个 package 的诊断
    # . | add => 把二维数组合并到一维，即所有诊断
    # . | group_by*(.kind) => 按照 .kind 聚合
    # (1) .map(...) => 对每个 kind 得到的数组进行操作，每个数组具有相同的 kind
    # (2) (.[0].kind) => 选取数组第一个元素的 kind 作为键（聚合数组的键总是相同的，并且至少由一个元素，因此 .[0].kind 总是有效的）
    # (3) map(.count) | add => 选取所有 count 并计总（在聚合数组中，已经保证了相同的 kind）
    kinds: .children | map(.kinds) | add | group_by(.kind) | map({kind: .[0].kind, count: map(.count) | add})
  }
);

# 由于 sort_by 只能指定字段排序，因此从数组转换到对象
def gen_sorting_keys(x): . | map(utils::zero(x) + {(.kind): .count}) | add;

def add_key:
# 给数组的每个元素添加位置数字索引 key_repo，它代表了保持原来的顺序（group_by 有自己的顺序)，也表示一个仓库
. | to_entries | map(.value + {key_repo: .key}) | map(
  . as {data: $a, key_repo: $key_repo} # 提取 a，因为需要把 a 复制进数组 b 的每个元素作为键
    | .children       # 提取数组 b
    | map(. + {key_repo: $key_repo, tag: 2, a: $a}) # 对数组 b 的每个元素添加聚合键
    | [{a: $a, key_repo: $key_repo, tag: 1}] + .    # 将 a 插入到数组 b 的开头，并给 a 添加聚合键
  ) # 此时是完全基于 b 的二维数组
  | flatten(2) # 展开成一维数组
  | to_entries | map(.value + { key }) # 添加位置索引：这是整个函数最重要的信息
  | group_by(.key_repo) # 按照仓库键聚合
  | map(group_by(.tag)) # 按照 tag 的值聚合
  | map(. as [[$a], $b] | { # 解构聚合后的数组：我们知道 a 只有一个
     key: $a.key, data: $a.a, children: $b | map({ key, data })
  }) # 删除聚合键，并恢复源数据结构，但保留了 key
;

def merge_kinds: .kinds | map({(.kind): .count}) | add;

def flatten_kinds:
. as $x
| map(.data |= . + merge_kinds)
| map(.data |= del(.kinds))
| map(.children |= map(.data |= . + merge_kinds))
| map(.children |= map(.data |= del(.kinds)))
;

# 重新排列字段，以及按照计数排序
def epilogue(x): . | map({
  data: { user, repo, total_count, kinds },
  sorting: .kinds | gen_sorting_keys(x),
  children: .children | map({
    data: {
      user: .key1.user,
      repo: .key1.repo,
      package: .key1.package,
      total_count,
      kinds,
    },
    sorting: .kinds | gen_sorting_keys(x)
  }) | utils::sort_by_kind_count(.data.total_count)
}) | utils::sort_by_kind_count(.data.total_count) | add_key | flatten_kinds;

. as $x | extract_kind_count | group_by_package | group_by_repo | epilogue($x)


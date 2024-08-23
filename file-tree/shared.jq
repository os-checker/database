import "./jq/utils" as utils;

# 填充 package 信息
def gen_pkg(root): root.env.packages[.pkg_idx] | {
  user: .repo.user,
  repo: .repo.repo,
  package: .name,
};

# 按照 package 和 file count 降序
def sort_by_count: . | sort_by(-.count);

# 生成聚合键
def gen_key:
. as $x
| .data
| map({
  key: {
    file,
    kind,
    pkg_idx: $x.cmd[.cmd_idx].package_idx,
  },
  item: . ,
  target_triple: $x.cmd[.cmd_idx].target_triple,
});

# 生成数据
def basic(root):
.
| group_by(.key.pkg_idx)
| map({
    pkg_idx: .[0].key.pkg_idx,
    raw_reports: . | group_by(.key.file) | map({
      file: .[0].key.file,
      arr: . | group_by(.key.kind) | map({kind: .[0].key.kind, raw: map(.item.raw)})
    })
  })
| map(.raw_reports |= (map(
      { file, count: .arr | map(.raw | length) | add }
      + { kinds: .arr | map({(.kind): .raw}) | add }
      + { sorting: .arr | map({kind, count: .raw | length} | utils::zero(root) + {(.kind): .count}) | add } # 用于内部排序
    ) | utils::sort_by_kind_count(.count) )
  )
| map(
    (. | gen_pkg(root))
    + { count: .raw_reports | map(.count) | add, raw_reports }
  )
| sort_by_count;

# 诊断类别的顺序
def kinds_order: .env.kinds.order;


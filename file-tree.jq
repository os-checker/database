# 填充 package 信息
def gen_pkg(x): x.env.packages[.pkg_idx] | {
  user: .repo.user,
  repo: .repo.repo,
  package: .name,
};

# 按照 package 和 file count 降序
def sort_by_count: . | sort_by(-.count);
def sort_by_kind_count: . | sort_by(
  -.count,
  -.sorting["Clippy(Error)"],
  -.sorting["Clippy(Warn)"],
  -.sorting["Unformatted"]
) | map(del(.sorting)); # 最后删除排序键

# 由于 sort_by 不允许对 null 值排序，所以给默认值；
# 必须放置在已有值之前，并通过 + 连接，因为 + 会让右边的键覆盖左边的键
def zero: {
  "Clippy(Error)": 0,
  "Clippy(Warn)": 0,
  Unformatted: 0,
};

# 抽取骨架
def basic:
. as $x | .data | map({ key: { file, kind, pkg_idx: $x.cmd[.cmd_idx].package_idx }, item: . })
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
      + { sorting: .arr | map({kind, count: .raw | length} | zero + {(.kind): .count}) | add } # 用于内部排序
      + (.arr | map({(.kind): .raw}) | add)
    ) | sort_by_kind_count)
  )
| map(
    (. | gen_pkg($x))
    + { count: .raw_reports | map(.count) | add, raw_reports }
  )
| sort_by_count;

. | basic 

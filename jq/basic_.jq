
def colname(order):
order
| map({(.): .}) 
| add 
| . + { # 覆盖 kinds 的名称
    Unformatted: "未格式化"
  }
| to_entries
| map({
    field: .key,
    header: .value,
  })
;

def gen:
. as $x
| .cmd
| group_by(.target_triple)
| {
    targets: . | map({
        triple: .[0].target_triple,
        count: map(.count) | add,
      }) | sort_by(-.count),
    kinds: ($x.env.kinds + {columns: colname($x.env.kinds.order) }),
  }
| .targets |= [{triple: "All-Targets", count: map(.count) | add }] + . # 把 All-Targets 放到最前面
;

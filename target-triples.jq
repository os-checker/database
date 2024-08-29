# 主要用于主页，生成基于 kinds 的列名：增加总计数、把某些类别翻译成中文
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

. as $x | .cmd | map(.target_triple) | {
  targets: . | unique,
  kinds: ($x.env.kinds + {columns: colname($x.env.kinds.order) }),
}

### jq 技巧：给嵌套/树形结构添加位置索引

PrimeVue 树形结构需要唯一的键，那么可以使用展开的位置索引

```jq
. | map(
  . as {a: $a} # 提取 a，因为需要把 a 复制进数组 b 的每个元素作为键
    | .b       # 提取数组 b
    | map(. + {tag: 2, a: $a}) # 对数组 b 的每个元素添加聚合键
    | [{a: $a, tag: 1}] + .    # 将 a 插入到数组 b 的开头，并给 a 添加聚合键
  ) # 此时是完全基于 b 的二维数组
  | flatten(2) # 展开成一维数组
  | to_entries | map(.value + {key}) # 给每个元素添加位置数字索引 key
  | group_by(.a)        # 按照 a 的值聚合
  | map(group_by(.tag)) # 按照 tag 的值聚合
  | map(. as [[$a], $b] | { # 解构聚合后的数组：我们知道 a 只有一个
      a: $a | del(.tag), b: $b | map(del(.a, .tag))
  }) # 删除聚合键，并恢复源数据结构，但保留了 key
```

<details>

<summary>jq 执行的 JSON 结果</summary>

```json
[
  {
    "a": {
      "a": 1,
      "key": 0
    },
    "b": [
      {
        "c": 2,
        "key": 1
      },
      {
        "c": 3,
        "key": 2
      }
    ]
  },
  {
    "a": {
      "a": 4,
      "key": 3
    },
    "b": [
      {
        "c": 5,
        "key": 4
      },
      {
        "c": 6,
        "key": 5
      }
    ]
  }
]
```

</details>

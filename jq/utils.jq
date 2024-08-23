
# 由于 sort_by 不允许对 null 值排序，所以给默认值；
# 必须放置在已有值之前，并通过 + 连接，因为 + 会让右边的键覆盖左边的键
# e.g. { "Clippy(Error)": 0, "Clippy(Warn)": 0, Unformatted: 0}
# 注意这个类型顺序是 os-checker 指定的
def zero(x): x | .env.kinds.order | map({(.): 0}) | add;
# 为了方便前端避免处理空值，这里填充空数组来避免空值
# e.g. { "Clippy(Error)": [], "Clippy(Warn)": [], "Unformatted": [] }
# def empty(x): x | .env.kinds.order | map({(.): []}) | add;


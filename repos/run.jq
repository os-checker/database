# 分割 json 中的仓库到单独的 json 文件
. as $x
| .data 
| group_by({user, repo}) # 按照 user/repo 分组，jq 会保留原排序的逻辑（比如先按 total_count 降序）
| .[] # 把每个 repo 分割开
| { kinds_order: $x.kinds_order, data: . } # 还原类似 file-tree 的结构
| "\(.data[0].user + "/" + .data[0].repo)\t\(.)" # 把每个仓库数据的 repo 抽离，并按照 \t 供 awk 分割
# jq 命令行使用 -c 将每个 repo 压缩成一行；使用 -r 去除引号转义

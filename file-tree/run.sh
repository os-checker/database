# This script should be run in root dir.

set +e

mkdir -p ui/file-tree/split/

cmd="jq -f file-tree/0_All-Targets.jq $UI_JSON >ui/file-tree/split/All-Targets.json"
echo "$cmd"
eval "$cmd"

cmd="jq -f file-tree/1_array-of-target-triples.jq $UI_JSON >ui/file-tree/array-of-target-triples.json"
echo "$cmd"
eval "$cmd"

# 将 array-of-target-triples.json 按照 target_triple 切分成一个个 JSON 文件
echo "切分 ui/file-tree/array-of-target-triples.json 文件"
jq -cr '.[] | {key:.target_triple,data} | "\(.key)\t\(.data)"' ui/file-tree/array-of-target-triples.json |
  awk -F\\t '{ print $2 >"ui/file-tree/split/" $1 ".json" }'

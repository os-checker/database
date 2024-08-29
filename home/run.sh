# This script should be run in root dir.

set +e

mkdir -p ui/home/split/

cmd="jq -f home/0_All-Targets.jq $UI_JSON >ui/home/split/All-Targets.json"
echo "$cmd"
eval "$cmd"

cmd="jq -f home/1_array-of-target-triples.jq $UI_JSON >ui/home/array-of-target-triples.json"
echo "$cmd"
eval "$cmd"

# 将 array-of-target-triples.json 按照 target_triple 切分成一个个 JSON 文件
echo "切分 ui/home/array-of-target-triples.json 文件"
jq -cr '.[] | {key:.target_triple,data} | "\(.key)\t\(.data)"' ui/home/array-of-target-triples.json |
  awk -F\\t '{ print $2 >"ui/home/split/" $1 ".json" }'

# This script should be run in root dir.

set +e

mkdir -p ui/home/split/

echo "jq -f home/0_all-target-triples.jq $UI_JSON >ui/home/all-target-triples.json"
jq -f home/0_all-target-triples.jq "$UI_JSON" >ui/home/all-target-triples.json

echo "jq -f home/1_array-of-target-triples.jq $UI_JSON >ui/home/array-of-target-triples.json"
jq -f home/1_array-of-target-triples.jq "$UI_JSON" >ui/home/array-of-target-triples.json

# 将 array-of-target-triples.json 按照 target_triple 切分成一个个 JSON 文件
echo "切分 ui/home/array-of-target-triples.json 文件"
jq -cr '.[] | {key:.target_triple,data} | "\(.key)\t\(.data)"' ui/home/array-of-target-triples.json |
  awk -F\\t '{ print $2 >"ui/home/split/" $1 ".json" }'

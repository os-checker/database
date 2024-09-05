# This script should be run in root dir.

set +e

export home=ui/home
export split=$home/split

rm -r "$split" || true
echo "移除旧的 $split 数据"
mkdir -p "$split"
echo "创建 $split"

cmd="jq -f home/0_All-Targets.jq $UI_JSON >$split/All-Targets.json"
echo "$cmd"
eval "$cmd"

cmd="jq -f home/1_array-of-target-triples.jq $UI_JSON >$home/array-of-target-triples.json"
echo "$cmd"
eval "$cmd"

# 将 array-of-target-triples.json 按照 target_triple 切分成一个个 JSON 文件
echo "切分 $home/array-of-target-triples.json 文件"
jq -cr '.[] | {key:.target_triple,data} | "\(.key)\t\(.data)"' $home/array-of-target-triples.json |
  awk -F\\t '{ split_dir = ENVIRON["split"]; print $2 > split_dir "/" $1 ".json" }'

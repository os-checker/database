repo=os-checker/database
test=tmp.md
src=tmp.json

title="jq 技巧：给嵌套/树形结构添加位置索引"
description="PrimeVue 树形结构需要唯一的键，那么可以使用展开的位置索引"

q=$(cat tmp.jq)

out=$(cat $src | jq "$q" | jsonxf)

cat >$test <<EOF
### $title

$description

<details>

<summary>JSON 输入</summary>

\`\`\`json
$(cat $src)
\`\`\`

</details>

\`\`\`jq
$q
\`\`\`

<details>

<summary>jq 执行的 JSON 结果</summary>

\`\`\`json
$out
\`\`\`

</details>
EOF

jless <<EOF
$out
EOF

# 环境变量控制是否发布 issue
if [ -n "$PUBLISH" ]; then
  gh issue create -R $repo --title "$title" -F tmp.md
fi

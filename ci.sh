#!/bin/bash

# 注意：如果检查结果和上次一样，那么无法提交（也不应该提交）

jq -f home.jq $UI_JSON >ui/home.json

git status
git add .
echo "正在提交：$(git status --porcelain)"
git commit -m "[bot] update $UI_JSON and friends from WebUI repo"
echo "提交成功，正在推送到 database 仓库"
git push
echo "成功推送到 database 仓库"

# # 检查是否有未暂存的更改
# git_status_output=$(git status --porcelain)
#
# if echo "$git_status_output" | grep -q "$UI_JSON"; then
#   echo "$UI_JSON 被添加到暂存"
#   git add $UI_JSON
# fi
#
# git_diff_output=$(git diff --cached --name-only | tr '\n' ' ' | sed -e 's/ *$//')
# if [ -n "$git_diff_output" ]; then
#   echo "正在提交 $git_diff_output"
#   git commit -m "[bot] update $UI_JSON from WebUI repo"
#   git push⋅
#   echo "已推送 $git_diff_output"
# else
#   echo "工作目录干净，无需提交 $UI_JSON"
# fi

# jq -f home.jq os-checker.json >ui/home.json

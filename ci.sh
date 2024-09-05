#!/bin/bash

# 如果 jq 或者 git 命令执行出错，则马上退出
set +e

if [ -z "$UI_JSON" ]; then
  export UI_JSON=ui.json
fi

echo "jq: $(which jq) | version: $(jq --version)"

echo "生成目标架构等基本信息：ui/basic.json"
jq -fc basic.jq $UI_JSON >ui/basic.json

sh ./home/run.sh
sh ./file-tree/run.sh

# 从 file-tree 的结果中生成仓库级别的信息
sh ./repos/run.sh

if [ -n "$BOT" ]; then
  echo "bot!"
  git status
  git add .
  echo "正在提交：$(git status --porcelain)"
  git commit -m "[bot] update $UI_JSON and friends from WebUI repo"
  echo "提交成功，正在推送到 database 仓库"
  git push
  echo "成功推送到 database 仓库"
fi

echo 🎇

# 注意：如果检查结果和上次一样，那么无法提交（也不应该提交）
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

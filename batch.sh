#!/bin/bash
set +e

if [ -n "$BOT" ]; then
  echo "bot!"
  git status
  git add .
  echo "正在提交：$(git status --porcelain)"
  git commit -m "[bot] update batch json and friends"
  echo "提交成功，正在推送到 database 仓库"
  git push
  echo "成功推送到 database 仓库"
fi

echo 🎇

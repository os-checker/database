if [ -n "$BOT" ]; then
  export branch=$(git branch --show-current)

  echo "bot!"
  git status
  git add .
  echo "正在提交：$(git status --porcelain)"
  git commit -m "[bot] update batch dir from os-checker CLI repo"
  echo "提交成功，正在推送到 database 仓库（分支：$(branch)）"
  git push
  echo "成功推送到 database 仓库（分支：$(branch)）"
fi

echo 🎇

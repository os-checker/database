export repos=ui/repos
export split=ui/file-tree/split

rm -r "$repos" || true
echo "移除旧的 $repos 数据"
mkdir -p "$repos"
echo "创建 $repos"

for file in "$split"/*; do
  export target=$(basename $file) # 含 .json
  echo "正在处理 \033[1m$target\033[0m ($file) => $repos/"
  jq -fcr repos/run.jq $file |
    awk -F\\t '{
      repos = ENVIRON["repos"]
      json = ENVIRON["target"]
      dir = repos "/" $1
      system("mkdir -p " dir)
      print $2 > dir "/" json
      print "成功生成   \033[3m" $1 "\033[0m" "/" json
    }'
done

jq -frc repos/basic.jq ui.json |
  awk -F\\t '{
    repos = ENVIRON["repos"]
    dir = repos "/" $1
    system("mkdir -p " dir)
    path = dir "/basic.json"
    print $2 > path 
    print "生成 " path
  }'

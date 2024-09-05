for file in ui/file-tree/split/*; do
  export target=$(basename $file) # 含 .json
  jq -fcr repos/run.jq $file |
    awk -F\\t '{
      json = ENVIRON["target"]
      dir = "ui/repos/" $1 "/" $2
      system("mkdir -p " dir)
      print $3 > dir "/" json
    }'
done

#!/bin/bash

sql=sql
ui=ui
db=raw_reports.db3

# 重新生成数据（目前不考虑支持历史数据)
rm $db -f
rm $ui -rf
mkdir -p $ui

sql_files=($(ls $sql))
for item in "${sql_files[@]}"; do
  in_sql="$sql/$item"
  out_json=$(echo $item | sed -E "s/[0-9]+_(.*)\.sql/$ui\/\1.json/")
  echo "正在运行 $in_sql"
  if [[ ! $item =~ ^3 ]]; then
    # 不要打印原始输出文本
    sqlite3 -table $db <$in_sql
  fi
  if [[ ! $item =~ ^0 ]]; then
    # 避免重复录入数据
    sqlite3 -json $db <$in_sql >$out_json
    echo "成功运行 $in_sql 并保留数据到 $out_json：$(ls -alh $out_json)"
  fi
done

# 不保留数据库文件
rm $db -f

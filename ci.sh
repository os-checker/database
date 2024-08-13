#!/bin/bash

sql=sql
ui=ui
db=raw_reports.db3

# 重新生成数据（目前不考虑支持历史数据)
rm $db -f
rm $ui
mkdir -p $ui

echo "准备存储 test_raw_reports.json 数据到 $db"
sqlite3 $db <$sql/read-json.sql
echo "成功存储 JSON 数据到 $db"

f1=user-repo-package-count
echo "运行 $f1.sql"
sqlite3 -json $db <$sql/$f1.sql >$ui/$f1.json
sqlite3 -table $db <$sql/$f1.sql

f2=user-repo-package-file-count
echo "运行 $f2.sql"
sqlite3 -json $db <$sql/$f2.sql >$ui/$f2.json
sqlite3 -table $db <$sql/$f2.sql

# 不保留数据库文件
rm $db -f

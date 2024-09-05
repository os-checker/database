# 主要用于主页，生成基于 kinds 的列名：增加总计数、把某些类别翻译成中文
import "./jq/basic_" as b;

. | b::gen

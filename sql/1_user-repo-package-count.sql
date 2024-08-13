-- 基于 user/repo#package 统计问题数量和问题文件数量
-- 如果 count 为 0，依然保留记录
SELECT
  json_extract (value, '$.user') AS user,
  json_extract (value, '$.repo') AS repo,
  json_extract (value, '$.package') AS package,
  json_extract (value, '$.count') AS kind_count,
  json_array_length (json_extract (value, '$.raw_reports')) AS file_count
FROM
  t,
  json_each (raw);

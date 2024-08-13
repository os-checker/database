-- 基于 user/repo#package/file 统计问题数量
-- 如果 count 为 0，则无记录
With
  report AS (
    SELECT
      json_extract (value, '$.user') AS user,
      json_extract (value, '$.repo') AS repo,
      json_extract (value, '$.package') AS package,
      -- json_extract (value, '$.count') AS count,
      json_extract (value, '$.raw_reports') AS reports
    FROM
      t,
      json_each (raw)
  ),
  files AS (
    SELECT
      *,
      json_extract (value, '$.file') AS file,
      json_array_length (json_extract (value, '$.fmt')) AS fmt,
      json_array_length (json_extract (value, '$.clippy_warn')) AS clippy_warn,
      json_array_length (json_extract (value, '$.clippy_error')) AS clippy_error
    FROM
      report,
      json_each (reports)
  )
SELECT
  user,
  repo,
  package,
  file,
  (fmt+clippy_warn+clippy_error) AS count,
  fmt,
  clippy_warn,
  clippy_error
FROM
  files;

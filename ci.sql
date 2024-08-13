CREATE TEMP TABLE IF NOT EXISTS t (id INTEGER PRIMARY KEY, raw TEXT) STRICT;

-- 把最新的 JSON 数据写成一行
INSERT INTO
  t (raw)
SELECT
  json (readfile ('test_raw_reports.json'));

SELECT
  id,
  json_array_length (raw)
FROM
  t;

-- 基于 user/repo#package 统计问题数量和问题文件数量
SELECT
  json_extract (value, '$.user') AS user,
  json_extract (value, '$.repo') AS repo,
  json_extract (value, '$.package') AS package,
  json_extract (value, '$.count') AS kind_count,
  json_array_length (json_extract (value, '$.raw_reports')) AS file_count
FROM
  t,
  json_each (raw);

-- 基于 user/repo#package/file 统计问题数量
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

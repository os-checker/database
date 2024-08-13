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

SELECT
  json_extract (value, '$.user') AS user,
  json_extract (value, '$.repo') AS repo,
  json_extract (value, '$.package') AS package,
  json_extract (value, '$.count') AS count
FROM
  t,
  json_each (raw);

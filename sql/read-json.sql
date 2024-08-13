CREATE TABLE IF NOT EXISTS t (id INTEGER PRIMARY KEY, raw TEXT) STRICT;

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

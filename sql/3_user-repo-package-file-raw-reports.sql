-- 基于 user/repo#package/file 统计问题数量
-- 如果 count 为 0，则无记录
CREATE VIEW IF NOT EXISTS file AS
SELECT
  *,
  json_extract (value, '$.file') AS file,
  json_extract (value, '$.fmt') AS fmt,
  json_extract (value, '$.clippy_warn') AS clippy_warn,
  json_extract (value, '$.clippy_error') AS clippy_error
FROM
  report,
  json_each (reports);

CREATE VIEW IF NOT EXISTS file_raw AS
With
  files AS (
    -- 如果使用 *，那么正式查询时无法通过 table.col 指定，因此必须在这使用别名；
    -- 此外，使用 * ，会在具有相同列名时，只保留 JOIN 关键字之前的列，比如 raw JOIN count_on_file，则 fmt 来自 raw
    SELECT
      file.user AS user,
      file.repo AS repo,
      file.package AS package,
      file.file AS file,
      file.fmt AS fmt,
      file.clippy_warn AS clippy_warn,
      file.clippy_error AS clippy_error,
      count_on_file.count,
      count_on_file.fmt AS c_fmt,
      count_on_file.clippy_warn AS c_clippy_warn,
      count_on_file.clippy_error AS c_clippy_error
    FROM
      count_on_file
      JOIN file ON file.user=count_on_file.user
      AND file.repo=count_on_file.repo
      AND file.package=count_on_file.package
      AND file.file=count_on_file.file
  )
SELECT
  *
FROM
  files;

SELECT
  user,
  repo,
  package,
  file,
  count,
  fmt,
  clippy_warn,
  clippy_error
FROM
  file_raw
ORDER BY
  count DESC,
  c_clippy_error DESC,
  c_clippy_warn DESC,
  c_fmt DESC;

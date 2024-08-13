-- UPDATE users
-- SET data = json_object(
--     keys(data),
--     json_insert(
--         data,
--         '$.age',
--         (json_extract(data, '$.age') + 1)::integer
--     )
-- )
-- WHERE id = 1;
-- .mode json
--SELECT id, name, json_extract(data, '$.age') AS age, json_extract(data, '$.email') AS email
-- SELECT name, data
-- FROM users;
CREATE TABLE IF NOT EXISTS users4 (id INTEGER PRIMARY KEY, name TEXT, data BLOB) STRICT;

INSERT INTO
  users4 (name, data)
SELECT
  json_extract (value, '$.name'),
  jsonb (json_extract (value, '$.data'))
FROM
  json_each (readfile ('test.json'));

.mode json
SELECT * FROM users4;

-- .mode table
-- SELECT id, name, json_extract(data, '$.age') AS age, json_extract(data, '$.email') AS email
-- FROM users;

src: KIMI (AI)

SQLite的JSON1扩展提供了丰富的函数来处理JSON数据。以下是一个使用SQLite和JSON1扩展的完整示例，包括创建表、插入JSON数据、查询和更新JSON数据。

### 1. 准备环境

首先，确保你的SQLite支持JSON1扩展。大多数现代的SQLite版本都内置了JSON1扩展。

### 2. 创建表

创建一个表，其中包含一个用于存储JSON数据的列。

```sql
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    data JSON NOT NULL
);
```

### 3. 插入JSON数据

使用`json`函数将JSON文本转换为SQLite的JSON类型，并插入到表中。

```sql
INSERT INTO users (id, name, data) VALUES
(1, 'Alice', json('{"age": 30, "email": "alice@example.com"}')),
(2, 'Bob', json('{"age": 24, "email": "bob@example.com"}'));
```

### 4. 查询JSON数据

使用`json_extract`函数提取JSON对象中的值。

```sql
SELECT id, name, json_extract(data, '$.age') AS age, json_extract(data, '$.email') AS email
FROM users;
```

### 5. 更新JSON数据

使用`json_object`和`json_extract`函数更新JSON数据。

```sql
UPDATE users
SET data = json_object(
    keys(data),
    json_insert(
        data,
        '$.age',
        (json_extract(data, '$.age') + 1)::integer
    )
)
WHERE id = 1;
```

上面的SQL语句将用户Alice的年龄增加1岁。

### 6. 删除JSON数据

使用`json_remove`函数从JSON对象中删除一个键。

```sql
UPDATE users
SET data = json_remove(data, '$.email')
WHERE id = 2;
```

上面的SQL语句将用户Bob的电子邮件地址从JSON数据中删除。

### 7. 处理JSON数组

假设我们更新`users`表，添加一个JSON数组来存储用户的兴趣爱好。

```sql
ALTER TABLE users ADD COLUMN hobbies JSON;

UPDATE users
SET hobbies = json_array('reading', 'traveling')
WHERE id = 1;

UPDATE users
SET hobbies = json_array_append(hobbies, 'coding')
WHERE id = 1;
```

上面的SQL语句首先为`users`表添加一个`hobbies`列来存储JSON数组，然后为Alice和Bob分别设置兴趣爱好，并且为Alice添加了一个新的爱好“coding”。

### 8. 查询JSON数组

查询用户的兴趣爱好。

```sql
SELECT id, name, json_extract(hobbies, '$[0]') AS hobby1, json_extract(hobbies, '$[1]') AS hobby2
FROM users;
```

请注意，这个示例假设你使用的SQLite版本已经内置了JSON1扩展，并且你已经启用了它。如果你的SQLite版本不包含JSON1扩展，你可能需要重新编译SQLite以包含这个扩展，或者使用其他方法来处理JSON数据。


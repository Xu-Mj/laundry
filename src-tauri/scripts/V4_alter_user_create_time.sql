-- 1. 将原表重命名为临时表
ALTER TABLE users RENAME TO users_old;

-- 2. 创建新表结构（修改字段类型为INTEGER）
CREATE TABLE IF NOT EXISTS users
(
    user_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id    INTEGER NOT NULL,
    open_id     TEXT,
    dept_id     INTEGER DEFAULT NULL,
    user_name   TEXT NOT NULL,
    nick_name   TEXT NOT NULL,
    address     TEXT,
    user_type   TEXT DEFAULT '00',
    email       TEXT DEFAULT '',
    phonenumber TEXT DEFAULT '',
    sex         TEXT DEFAULT '0',
    avatar      TEXT DEFAULT '',
    password    TEXT DEFAULT '',
    status      TEXT DEFAULT '0',
    del_flag    TEXT DEFAULT '0',
    integral    INTEGER DEFAULT 0,
    identify    TEXT DEFAULT '00',
    login_ip    TEXT DEFAULT '',
    login_date  TIMESTAMP DEFAULT NULL,
    create_by   TEXT DEFAULT '',
    create_time INTEGER,  -- 修改为INTEGER
    update_by   TEXT DEFAULT '',
    update_time INTEGER,  -- 修改为INTEGER
    remark      TEXT DEFAULT NULL
);

-- 3. 从旧表复制数据到新表
INSERT INTO users SELECT 
    user_id, store_id, open_id, dept_id, user_name, nick_name, address,
    user_type, email, phonenumber, sex, avatar, password, status,
    del_flag, integral, identify, login_ip, login_date, create_by,
    -- 转换TIMESTAMP为INTEGER（UNIX时间戳）
    CASE WHEN create_time IS NULL THEN NULL 
         ELSE CAST(strftime('%s', create_time) AS INTEGER) END,
    update_by,
    CASE WHEN update_time IS NULL THEN NULL 
         ELSE CAST(strftime('%s', update_time) AS INTEGER) END,
    remark
FROM users_old;

-- 4. 删除临时表（可选）
DROP TABLE users_old;
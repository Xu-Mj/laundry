-- 创建user_tours表，用于存储用户引导记录
CREATE TABLE IF NOT EXISTS user_tours (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    page_key TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES local_users (id)
);

-- 添加索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_user_tours_user_id ON user_tours (user_id);
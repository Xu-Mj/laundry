CREATE TABLE IF NOT EXISTS subscriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id INTEGER NOT NULL,
    plan_id INTEGER NOT NULL,
    start_date INTEGER NOT NULL,
    expiry_date INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    remark TEXT,
    FOREIGN KEY (store_id) REFERENCES local_users(id)
);

-- 添加索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_subscriptions_store_id ON subscriptions(store_id);
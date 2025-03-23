CREATE TABLE IF NOT EXISTS alipay_configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id INTEGER NOT NULL,
    app_id TEXT NOT NULL,
    private_key TEXT NOT NULL,
    alipay_public_key TEXT,
    seller_id TEXT,
    is_active INTEGER NOT NULL DEFAULT 0,
    is_sandbox INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_alipay_configs_store_id ON alipay_configs (store_id);

CREATE INDEX idx_alipay_configs_is_active ON alipay_configs (is_active);

CREATE TABLE wechat_configs (
    id INTEGER PRIMARY KEY,
    store_id INTEGER NOT NULL,
    sp_appid TEXT,
    sp_mchid TEXT,
    app_id TEXT NOT NULL,
    mchid TEXT NOT NULL,
    mch_key TEXT NOT NULL,
    apiclient_key TEXT NOT NULL,
    apiclient_cert TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT false,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_wechat_configs_store_id ON wechat_configs (store_id);

CREATE INDEX idx_wechat_configs_is_active ON wechat_configs (is_active);
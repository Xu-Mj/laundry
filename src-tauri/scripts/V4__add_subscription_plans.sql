-- 创建subscription_plans表，用于存储订阅套餐信息
CREATE TABLE IF NOT EXISTS subscription_plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    plan_type TEXT NOT NULL,
    period TEXT NOT NULL,
    price TEXT NOT NULL,
    description TEXT,
    features TEXT,  -- JSON格式存储
    is_recommended BOOLEAN NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    remark TEXT
);

-- 添加索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_subscription_plans_type ON subscription_plans(plan_type);
CREATE INDEX IF NOT EXISTS idx_subscription_plans_active ON subscription_plans(is_active);

CREATE TABLE sms_plans (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    plan_type TEXT NOT NULL,
    period TEXT NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    sms_count INTEGER NOT NULL,
    description TEXT,
    features JSON,
    is_recommended BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    sort_order INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    remark TEXT
);

-- 添加索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_sms_plans_type ON sms_plans(plan_type);
CREATE INDEX IF NOT EXISTS idx_sms_plans_active ON sms_plans(is_active);
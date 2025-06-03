CREATE TABLE IF NOT EXISTS payments (
    pay_id TEXT PRIMARY KEY,
    pay_number TEXT,
    uc_order_id INTEGER,
    order_type TEXT,
    total_amount REAL,
    payment_status TEXT,
    payment_method TEXT,
    create_time INTEGER,
    update_time INTEGER,
    store_id INTEGER,
    refund_reason TEXT
);

CREATE TABLE IF NOT EXISTS payment_method_details (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    payment_id TEXT,
    store_id INTEGER,
    payment_status TEXT,
    transaction_id INTEGER,
    method TEXT,
    amount REAL,
    create_time INTEGER,
    FOREIGN KEY (payment_id) REFERENCES payments (pay_id)
);

CREATE TABLE IF NOT EXISTS coupon_usages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    payment_id TEXT,
    coupon_id INTEGER,
    coupon_type TEXT,
    applied_amount REAL,
    is_refunded BOOLEAN DEFAULT 0,
    FOREIGN KEY (payment_id) REFERENCES payments (pay_id)
);
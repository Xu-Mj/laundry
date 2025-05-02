CREATE TABLE IF NOT EXISTS migrations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    version INTEGER NOT NULL,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS printers
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    system_name TEXT NOT NULL,
    driver_name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS local_users
(
    id  INTEGER PRIMARY KEY AUTOINCREMENT,
    nickname      TEXT    NOT NULL,
    owner_name      TEXT    NOT NULL,
    avatar          TEXT    NOT NULL,
    owner_phone     TEXT    NOT NULL,
    password        TEXT    NOT NULL,
    role            TEXT    NOT NULL,
    is_guest        INTEGER,
    store_name      TEXT NOT NULL,
    store_location  TEXT NOT NULL,
    owner_gender    TEXT,
    store_status    TEXT NOT NULL,
    province        TEXT,
    city            TEXT,
    district        TEXT,
    address_detail  TEXT,
    created_at      INTEGER,
    updated_at      INTEGER,
    remark          TEXT,
    email           TEXT,
    deleted         TEXT,
    user_id         INTEGER
);

CREATE TABLE IF NOT EXISTS membership_level (
    level_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, -- 会员等级ID
    level_code TEXT NOT NULL,                            -- 等级编码
    level_name TEXT NOT NULL,                            -- 等级名称
    level_sort INTEGER NOT NULL,                         -- 显示顺序
    status TEXT NOT NULL,                                -- 状态（0: 正常, 1: 停用）
    create_time DATETIME DEFAULT NULL,                   -- 创建时间
    update_time DATETIME DEFAULT NULL,                   -- 更新时间
    remark TEXT DEFAULT NULL                             -- 备注
);

CREATE TABLE IF NOT EXISTS user_membership_level (
    user_id INTEGER NOT NULL,      -- 用户ID
    level_id INTEGER NOT NULL,     -- 会员等级ID
    PRIMARY KEY (user_id, level_id) -- 复合主键: 用户ID和会员等级ID
);

CREATE TABLE IF NOT EXISTS configs
(
    config_id    INTEGER PRIMARY KEY AUTOINCREMENT,                           -- 参数主键
    config_name  TEXT                                   DEFAULT ''  NOT NULL, -- 参数名称
    config_key   TEXT                                   DEFAULT ''  NOT NULL, -- 参数键名
    config_value TEXT                                   DEFAULT ''  NOT NULL, -- 参数键值
    config_type  TEXT CHECK (config_type IN ('Y', 'N')) DEFAULT 'N' NOT NULL, -- 系统内置（Y是 N否）
    create_by    TEXT                                   DEFAULT '',           -- 创建者
    create_time  TIMESTAMP                              DEFAULT NULL,         -- 创建时间 (用 ISO8601 格式存储)
    update_by    TEXT                                   DEFAULT '',           -- 更新者
    update_time  TIMESTAMP                              DEFAULT NULL,         -- 更新时间 (用 ISO8601 格式存储)
    remark       TEXT                                   DEFAULT NULL          -- 备注
);

CREATE TABLE IF NOT EXISTS dict_type
(
    dict_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    dict_name   TEXT DEFAULT '',
    dict_type   TEXT DEFAULT '',
    status      TEXT DEFAULT '0',
    create_time TIMESTAMP,
    update_time TIMESTAMP,
    remark      TEXT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS dict_data
(
    dict_code   INTEGER PRIMARY KEY AUTOINCREMENT,
    dict_sort   INTEGER DEFAULT 0,
    dict_label  TEXT    DEFAULT '',
    dict_value  TEXT    DEFAULT '',
    dict_type   TEXT    DEFAULT '',
    css_class   TEXT    DEFAULT NULL,
    list_class  TEXT    DEFAULT NULL,
    is_default  TEXT    DEFAULT 'N',
    status      TEXT    DEFAULT '0',
    create_time TIMESTAMP,
    update_time TIMESTAMP,
    remark      TEXT    DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS users
(
    user_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id    INTEGER NOT NULL,
    open_id     TEXT ,
    dept_id     INTEGER  DEFAULT NULL,
    user_name   TEXT NOT NULL,
    nick_name   TEXT NOT NULL,
    address     TEXT,
    user_type   TEXT          DEFAULT '00',
    email       TEXT          DEFAULT '',
    phonenumber TEXT          DEFAULT '',
    sex         TEXT          DEFAULT '0',
    avatar      TEXT          DEFAULT '',
    password    TEXT          DEFAULT '',
    status      TEXT          DEFAULT '0',
    del_flag    TEXT          DEFAULT '0',
    integral    INTEGER       DEFAULT 0,
    identify    TEXT          DEFAULT '00',
    login_ip    TEXT          DEFAULT '',
    login_date  TIMESTAMP     DEFAULT NULL,
    create_by   TEXT          DEFAULT '',
    create_time TIMESTAMP,
    update_by   TEXT          DEFAULT '',
    update_time TIMESTAMP,
    remark      TEXT          DEFAULT NULL
);
CREATE INDEX idx_users_store_id ON users (store_id);

-- 会员画像表
CREATE TABLE user_tags
(
    user_id INTEGER PRIMARY KEY,
    tags    TEXT    NOT NULL,
    remark  TEXT
);

-- 积分使用记录表
CREATE TABLE user_integral_record
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id     INTEGER NOT NULL,
    coupon_id   INTEGER NOT NULL,
    identify    INTEGER NOT NULL,
    create_time TIMESTAMP
);

-- 创建索引，提高根据用户id查询效率
CREATE INDEX idx_user_integral_record_user_id ON user_integral_record (user_id);

-- 标签管理表
CREATE TABLE tags
(
    tag_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id   INTEGER NOT NULL,
    tag_number TEXT NOT NULL,
    tag_order  TEXT,
    tag_name   TEXT NOT NULL,
    ref_num    INTEGER DEFAULT 0,
    order_num  INTEGER DEFAULT 0,
    status     TEXT    DEFAULT '0',
    del_flag   TEXT    DEFAULT '0',
    remark     TEXT
);
CREATE INDEX idx_tags_store_id ON tags (store_id);

-- 衣物品类表
CREATE TABLE clothing_categories
(
    category_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id      INTEGER NOT NULL,
    category_code TEXT    NOT NULL,
    category_name TEXT    NOT NULL,
    order_num     INTEGER DEFAULT 0,
    remark        TEXT,
    del_flag      TEXT    DEFAULT '0',
    created_at    INTEGER NOT NULL,
    updated_at    INTEGER NOT NULL
);

-- 创建衣物品类表索引
CREATE INDEX idx_clothing_categories_store_id ON clothing_categories(store_id);
CREATE INDEX idx_clothing_categories_code ON clothing_categories(category_code);
CREATE INDEX idx_clothing_categories_name ON clothing_categories(category_name);
CREATE INDEX idx_clothing_categories_del_flag ON clothing_categories(del_flag);

-- 衣物分类表
CREATE TABLE clothing_styles
(
    style_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id     INTEGER NOT NULL,
    category_id  INTEGER NOT NULL,
    style_code   TEXT    NOT NULL,
    style_name   TEXT    NOT NULL,
    order_num    INTEGER DEFAULT 0,
    remark       TEXT,
    del_flag     TEXT    DEFAULT '0',
    created_at   INTEGER NOT NULL,
    updated_at   INTEGER NOT NULL,
    FOREIGN KEY(category_id) REFERENCES clothing_categories(category_id)
);

-- 创建衣物分类表索引
CREATE INDEX idx_clothing_styles_store_id ON clothing_styles(store_id);
CREATE INDEX idx_clothing_styles_category_id ON clothing_styles(category_id);
CREATE INDEX idx_clothing_styles_code ON clothing_styles(style_code);
CREATE INDEX idx_clothing_styles_name ON clothing_styles(style_name);
CREATE INDEX idx_clothing_styles_del_flag ON clothing_styles(del_flag);

-- 衣物管理表
CREATE TABLE clothing
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id           INTEGER NOT NULL,
    clothing_number    TEXT,
    category_id        INTEGER,
    style_id           INTEGER,
    title              TEXT   NOT NULL,
    etitle             TEXT,
    primary_image      TEXT,
    images             TEXT,
    description_images TEXT,
    is_put_on_sale     BOOLEAN DEFAULT FALSE,
    is_available       BOOLEAN DEFAULT TRUE,
    is_sold_out        BOOLEAN DEFAULT FALSE,
    is_default         BOOLEAN DEFAULT FALSE,
    clothing_base_price DOUBLE NOT NULL,
    sale_price         DOUBLE,
    clothing_min_price DOUBLE,
    stock_quantity     INTEGER DEFAULT 0,
    sold_num           INTEGER DEFAULT 0,
    sku_list           TEXT,
    spec_list          TEXT,
    tag_list           TEXT,
    hang_type          TEXT   DEFAULT '1',
    order_num          INTEGER DEFAULT 0,
    clothing_degree    INTEGER DEFAULT 0,
    del_flag           TEXT   DEFAULT '0',
    create_time        INTEGER NOT NULL,
    update_time        INTEGER NOT NULL,
    FOREIGN KEY(category_id) REFERENCES clothing_categories(category_id),
    FOREIGN KEY(style_id) REFERENCES clothing_styles(style_id)
);

-- 创建索引，提高根据衣物类别和衣物名称查询效率
CREATE INDEX idx_clothing_category ON clothing (clothing_category);
CREATE INDEX idx_clothing_title ON clothing (title);
CREATE INDEX idx_clothing_store_id ON clothing (store_id);
CREATE INDEX idx_clothing_category_id ON clothing(category_id);
CREATE INDEX idx_clothing_style_id ON clothing(style_id);

-- 卡券管理表
CREATE TABLE coupons
(
    coupon_id           INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id            INTEGER NOT NULL,
    coupon_number       TEXT UNIQUE NOT NULL,
    coupon_type         TEXT        NOT NULL DEFAULT '000',
    coupon_title        TEXT        NOT NULL,
    desc                TEXT,
    coupon_value        DOUBLE      NOT NULL,
    min_spend           DOUBLE               DEFAULT 0,
    customer_invalid    TEXT                 DEFAULT '0',
    customer_sale_total INTEGER              DEFAULT 0,
    customer_sale_count INTEGER              DEFAULT 0,
    valid_from          TIMESTAMP   NOT NULL,
    valid_to            TIMESTAMP   NOT NULL,
    auto_delay          TEXT                 DEFAULT '0',
    usage_value         DOUBLE               DEFAULT 0,
    usage_limit         DOUBLE               DEFAULT 0,
    del_flag            TEXT                 DEFAULT '0',
    applicable_category TEXT                 DEFAULT NULL,
    applicable_style    TEXT                 DEFAULT NULL,
    applicable_cloths   TEXT                 DEFAULT NULL,
    status              TEXT                 DEFAULT '0',
    remark              TEXT                 DEFAULT NULL
);

-- 卡券表索引
CREATE INDEX idx_coupon_name ON coupons (coupon_title);

CREATE INDEX idx_coupon_type ON coupons (coupon_type);

CREATE INDEX idx_coupon_status ON coupons (status);

CREATE INDEX idx_coupon_del_flag ON coupons (del_flag);
CREATE INDEX idx_coupons_store_id ON coupons (store_id);

-- 用户卡券关联表
CREATE TABLE user_coupons
(
    uc_id           INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id        INTEGER NOT NULL,
    user_id         INTEGER NOT NULL,
    coupon_id       INTEGER NOT NULL,
    create_time     TIMESTAMP,
    obtain_at       TIMESTAMP DEFAULT NULL,
    available_value DOUBLE    DEFAULT 0,
    uc_count        INTEGER   DEFAULT 1,
    pay_id          TEXT   DEFAULT NULL,
    uc_type         TEXT      DEFAULT '01',
    status          TEXT      DEFAULT '00',
    remark          TEXT      DEFAULT NULL
);

-- 用户卡券索引
CREATE INDEX idx_user_coupons_user_id_user_coupons ON user_coupons (user_id);

CREATE INDEX idx_coupon_id ON user_coupons (coupon_id);

CREATE INDEX idx_status ON user_coupons (status);

CREATE INDEX idx_user_status ON user_coupons (user_id, status);
CREATE INDEX idx_user_coupons_store_id ON user_coupons (store_id);

-- 卡券订单表
CREATE TABLE coupon_orders
(
    order_id    INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id    INTEGER NOT NULL,
    uc_id       TEXT NOT NULL,
    create_time TIMESTAMP
);

-- 卡券订单索引
CREATE INDEX idx_uc_id ON coupon_orders (uc_id);

CREATE INDEX idx_coupon_orders_create_time ON coupon_orders (create_time);

CREATE INDEX idx_create_uc ON coupon_orders (create_time, uc_id);
CREATE INDEX idx_coupon_orders_store_id ON coupon_orders (store_id);

-- 支付记录表
CREATE TABLE payments
(
    pay_id             TEXT PRIMARY KEY,
    store_id           INTEGER NOT NULL,
    pay_number         TEXT   NOT NULL,
    order_type         TEXT   NOT NULL,
    total_amount       DOUBLE NOT NULL,
    payment_amount     DOUBLE NOT NULL,
    payment_amount_vip DOUBLE DEFAULT 0,
    payment_amount_mv  DOUBLE DEFAULT 0,
    payment_status     TEXT   NOT NULL,
    payment_method     TEXT   NOT NULL,
    transaction_id     INTEGER,
    uc_order_id        INTEGER,
    uc_id              TEXT,
    create_time        INTEGER,
    update_time        INTEGER,
    order_status       TEXT   NOT NULL
);

CREATE INDEX idx_payments_order_type ON payments (order_type); -- 支付记录索引
CREATE INDEX idx_payments_payment_status ON payments (payment_status);

CREATE INDEX idx_pay_number ON payments (pay_number);

CREATE INDEX idx_payments_create_time ON payments (create_time);

CREATE INDEX idx_order_status ON payments (order_type, payment_status);

CREATE INDEX idx_pay_number_order ON payments (pay_number, order_type);
CREATE INDEX idx_payments_store_id ON payments (store_id);

-- 通知模板管理表
CREATE TABLE notice_temp
(
    temp_id       INTEGER PRIMARY KEY AUTOINCREMENT,
    temp_name     TEXT NOT NULL,
    notice_method TEXT NOT NULL,
    content       TEXT NOT NULL,
    create_time   INTEGER,
    temp_type     TEXT NOT NULL,
    remark        TEXT
);

-- 通知记录管理表
-- todo 订单id需要后续进行关联
CREATE TABLE notice_record
(
    notice_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id      INTEGER NOT NULL,
    user_id       INTEGER NOT NULL,
    order_number  TEXT    NULL,
    notice_method TEXT    NOT NULL,
    notice_type   TEXT    NOT NULL,
    notice_time   TIMESTAMP,
    title         TEXT    NOT NULL,
    content       TEXT    NOT NULL,
    result        TEXT,
    remark        TEXT
);
CREATE INDEX idx_notice_record_store_id ON notice_record (store_id);

-- 创建索引
CREATE INDEX idx_notice_record_user_id ON notice_record (user_id);

-- 衣物价格表
CREATE TABLE cloth_price
(
    price_id       INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id       INTEGER NOT NULL,
    price_number   TEXT    NOT NULL,
    order_type     TEXT    NOT NULL,
    price_name     TEXT    NOT NULL,
    price_value    DOUBLE,
    price_discount DOUBLE,
    order_num      INTEGER NOT NULL,
    ref_num        INTEGER DEFAULT 0,
    status         TEXT    DEFAULT '0',
    del_flag       TEXT    DEFAULT '0',
    remark         TEXT,
    create_time    TIMESTAMP,
    update_time    TIMESTAMP
);

CREATE INDEX idx_cloth_price_order_type ON cloth_price (order_type);
CREATE INDEX idx_cloth_price_store_id ON cloth_price (store_id);

-- 订单表
CREATE TABLE orders
(
    order_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id             INTEGER NOT NULL,
    order_number         TEXT      NOT NULL,
    business_type        TEXT      NOT NULL,
    user_id              INTEGER   NOT NULL,
    price_id             INTEGER,
    desire_complete_time TIMESTAMP NOT NULL,
    cost_time_alarm      TEXT,
    pickup_code          TEXT,
    complete_time        TIMESTAMP,
    delivery_mode        TEXT,
    source               TEXT      NOT NULL,
    status               TEXT      NOT NULL,
    payment_status       TEXT      NOT NULL,
    remark               TEXT,
    order_type           TEXT      NOT NULL,
    create_time          TIMESTAMP,
    update_time          TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_order_number ON orders (order_number);

CREATE INDEX idx_orders_user_id ON orders (user_id);

CREATE INDEX idx_pickup_code ON orders (pickup_code);

CREATE INDEX idx_cost_time_alarm ON orders (cost_time_alarm);

CREATE INDEX idx_orders_payment_status ON orders (payment_status);
CREATE INDEX idx_orders_store_id ON orders (store_id);

-- 衣物清单表
CREATE TABLE order_clothes
(
    cloth_id            INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id            INTEGER NOT NULL,
    order_id            INTEGER NULL,
    clothing_id         INTEGER NOT NULL,
    clothing_category   TEXT,
    category_id         INTEGER    NOT NULL,
    clothing_style      TEXT,
    style_id            INTEGER    NOT NULL,
    clothing_color      INTEGER,
    clothing_flaw       TEXT,
    estimate            TEXT,
    clothing_brand      INTEGER,
    service_type        TEXT    NOT NULL,
    service_requirement TEXT,
    before_pics         TEXT,
    after_pics          TEXT,
    notes               TEXT,
    process_markup      FLOAT,
    price_value         FLOAT   NOT NULL,
    hang_type           TEXT    NOT NULL DEFAULT '1',
    hang_location_code  INTEGER,
    hanger_number       INTEGER,
    hang_cloth_code     TEXT UNIQUE,
    hang_remark         TEXT,
    create_time         TIMESTAMP,
    pickup_time         TIMESTAMP,
    pickup_method       TEXT,
    clothing_status     TEXT,
    remark              TEXT,
    
    FOREIGN KEY(store_id) REFERENCES local_users(id),
    FOREIGN KEY(category_id) REFERENCES clothing_categories(category_id),
    FOREIGN KEY(style_id) REFERENCES clothing_styles(style_id)
);

-- 创建索引
CREATE INDEX idx_order_clothing_id ON order_clothes (order_id);
-- CREATE INDEX idx_order_clothes_store_id ON order_clothes (store_id);

-- 订单衣物调价记录表 ，记录订单中每个衣物或订单总价的价格调整
CREATE TABLE order_clothes_adjust
(
    adjust_id        INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id         INTEGER UNIQUE,
    adjust_value_add FLOAT,
    adjust_value_sub FLOAT,
    adjust_total     FLOAT,
    remark           TEXT
);

-- 订单涉及的图片索引表，记录订单中相关的图片信息
CREATE TABLE order_pictures
(
    picture_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    picture_path TEXT NOT NULL,
    create_by    TEXT,
    create_time  TIMESTAMP
);

-- 订单派送信息
CREATE TABLE IF NOT EXISTS deliveries (
    delivery_id INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id INTEGER,
    user_id INTEGER,
    order_id TEXT,
    cloth_id TEXT,
    address TEXT,
    dispatch_time DATETIME,
    complete_time DATETIME,
    remark TEXT,
    delivery_status TEXT DEFAULT '00',
    create_time DATETIME,
    update_time DATETIME,
    FOREIGN KEY (store_id) REFERENCES local_users(id),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_deliveries_user_id ON deliveries(user_id);
CREATE INDEX IF NOT EXISTS idx_deliveries_store_id ON deliveries(store_id);
CREATE INDEX IF NOT EXISTS idx_deliveries_status ON deliveries(delivery_status);
-- 支出记录表
CREATE TABLE expenditure
(
    exp_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id           INTEGER NOT NULL,
    order_id           TEXT,
    cloth_ids          TEXT,
    exp_title          TEXT    NOT NULL,
    recv_account       INTEGER,
    recv_account_title TEXT,
    exp_type           TEXT    NOT NULL,
    exp_amount         INTEGER NOT NULL,
    create_time        INTEGER,
    remark             TEXT
);
CREATE INDEX idx_expenditure_store_id ON expenditure (store_id);

-- 晾衣架表
CREATE TABLE drying_rack
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    store_id           INTEGER NOT NULL,
    name               TEXT    NOT NULL,
    rack_type          TEXT             DEFAULT '1',
    capacity           INTEGER NOT NULL,
    remaining_capacity INTEGER NOT NULL,
    position           INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX idx_drying_rack_store_id ON drying_rack (store_id);
-- 写一条游客使用的数据
INSERT INTO drying_rack (store_id, name, rack_type, capacity, remaining_capacity, position) VALUES (0, '游客使用晾衣架', '1', 100, 100, 0);


-- 用来存储衣物的编码最大值
CREATE TABLE cloth_sequence
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    date            DATE    NOT NULL,
    sequence_number INTEGER NOT NULL
);

-- 复洗订单关联表
CREATE TABLE order_repair
(
    repair_id    INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id     INTEGER,
    old_order_id INTEGER,
    create_time  TIMESTAMP,
    remark       TEXT
);

-- 推广模板
CREATE TABLE promote_template
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    content         TEXT NOT NULL,
    promote_type    TEXT NOT NULL,
    promote_method  TEXT NOT NULL,
    promote_count   INTEGER,
    promote_objects TEXT,
    create_time     TIMESTAMP,
    promote_picture TEXT,
    is_pin          TEXT DEFAULT '0',
    remark          TEXT
);

-- 推广记录表
-- 因为存在仅本次有效的情况，所以还是需要一个推广对象的字段的
CREATE TABLE promote_record
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    temp_id         INTEGER,
    promote_objects TEXT NOT NULL,
    promote_time    TIMESTAMP,
    status          TEXT DEFAULT '0'
);

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

CREATE TABLE sms_subscriptions (
    id BIGSERIAL PRIMARY KEY,
    store_id BIGINT NOT NULL REFERENCES local_users(id),
    plan_id BIGINT NOT NULL REFERENCES sms_plans(id),
    status VARCHAR(20) NOT NULL,
    start_date BIGINT NOT NULL,
    expiry_date BIGINT NOT NULL,
    auto_renew BOOLEAN NOT NULL DEFAULT true,
    last_payment_id TEXT REFERENCES payments(pay_id),
    next_billing_date BIGINT,
    price_paid DECIMAL(10,2) NOT NULL,
    total_sms_count INTEGER NOT NULL,
    used_sms_count INTEGER NOT NULL DEFAULT 0,
    remaining_sms_count INTEGER GENERATED ALWAYS AS (total_sms_count - used_sms_count) STORED,
    promo_code VARCHAR(20),
    is_first_free BOOLEAN NOT NULL DEFAULT false,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    cancellation_reason TEXT,
    remark TEXT
);

-- 支付宝配置表
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

-- 微信支付配置表
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

-- 二维码支付信息表
CREATE TABLE qrcode_payments
(
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    pay_id            TEXT NOT NULL,                -- 关联的支付记录ID
    store_id          INTEGER NOT NULL,                -- 商店ID
    payment_type      TEXT NOT NULL,                   -- 支付类型：alipay/wechat
    auth_code         TEXT,                            -- 授权码（用户付款码）
    qr_code           TEXT,                            -- 二维码内容（商家收款码）
    out_trade_no      TEXT,                            -- 商户订单号
    trade_no          TEXT,                            -- 支付宝/微信交易号
    total_amount      DOUBLE NOT NULL,                 -- 订单金额
    subject           TEXT,                            -- 交易主题
    trade_status      TEXT,                            -- 交易状态
    buyer_id          TEXT,                            -- 买家ID
    buyer_logon_id    TEXT,                            -- 买家登录账号
    receipt_amount    DOUBLE,                          -- 实收金额
    point_amount      DOUBLE,                          -- 积分金额
    invoice_amount    DOUBLE,                          -- 开票金额
    fund_bill_list    TEXT,                            -- 支付资金明细
    voucher_detail_list TEXT,                          -- 优惠券信息
    gmt_payment       TIMESTAMP,                       -- 交易支付时间
    raw_response      TEXT,                            -- 原始响应数据
    create_time       TIMESTAMP NOT NULL,              -- 创建时间
    update_time       TIMESTAMP                        -- 更新时间
);

-- 创建索引
CREATE INDEX idx_qrcode_payments_pay_id ON qrcode_payments (pay_id);
CREATE INDEX idx_qrcode_payments_payment_type ON qrcode_payments (payment_type);
CREATE INDEX idx_qrcode_payments_trade_no ON qrcode_payments (trade_no);
CREATE INDEX idx_qrcode_payments_out_trade_no ON qrcode_payments (out_trade_no);

-- 添加WebSocket消息表
CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sender_id INTEGER NOT NULL,
    receiver_id INTEGER NOT NULL,
    message_type VARCHAR(20) NOT NULL,
    content TEXT NOT NULL,
    read BOOLEAN NOT NULL DEFAULT FALSE,
    sent BOOLEAN NOT NULL DEFAULT FALSE,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 创建索引
CREATE INDEX idx_messages_sender_id ON messages(sender_id);
CREATE INDEX idx_messages_receiver_id ON messages(receiver_id);
CREATE INDEX idx_messages_read ON messages(read);

INSERT INTO local_users (id, nickname, owner_name, avatar, owner_phone, password, role, is_guest, store_name, store_location, store_status, created_at, deleted) VALUES (0, 'Guest', 'Guest', 'images/avatars/avatar1.png', '1234567890', '123', 'Guest', 1, 'guest', 'guest fake location', '0', CURRENT_TIMESTAMP, '0');

INSERT INTO configs (config_id, config_name, config_key, config_value, config_type, create_by, create_time, update_by, update_time, remark) VALUES (1, '账号自助-验证码开关', 'sys.account.captchaEnabled', 'false', 'Y', 'admin', null, '', null, '是否开启验证码功能（true开启，false关闭）');
INSERT INTO configs (config_id, config_name, config_key, config_value, config_type, create_by, create_time, update_by, update_time, remark) VALUES (2, '预计取衣时间', 'desire_complete_time', '17', 'Y', null, '2025-02-15T10:22:13.032273400+08:00', null, '2025-02-15T10:23:31.618923900+08:00', '默认七天后取衣');
INSERT INTO configs (config_id, config_name, config_key, config_value, config_type, create_by, create_time, update_by, update_time, remark) VALUES (3, '页面无操作注销时间', 'logout_timeout', '600', 'Y', null, '2025-02-15T11:40:01.890123500+08:00', null, null, '单位：秒');

INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (1, '用户性别', 'sys_user_sex', '0', '2024-08-16 06:41:56', null, '用户性别列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (2, '菜单状态', 'sys_show_hide', '0', '2024-08-16 06:41:56', '', '菜单状态列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (3, '系统开关', 'sys_normal_disable', '0', '2024-08-16 06:41:56', '', '系统开关列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (4, '任务状态', 'sys_job_status', '0', '2024-08-16 06:41:56', '', '任务状态列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (5, '任务分组', 'sys_job_group', '0', '2024-08-16 06:41:56', '', '任务分组列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (6, '系统是否', 'sys_yes_no', '0', '2024-08-16 06:41:56', '', '系统是否列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (7, '通知类型', 'sys_notice_type', '0', '2024-08-16 06:41:56', '', '通知类型列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (8, '通知状态', 'sys_notice_status', '0', '2024-08-16 06:41:56', '', '通知状态列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (9, '操作类型', 'sys_oper_type', '0', '2024-08-16 06:41:56', '', '操作类型列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (10, '系统状态', 'sys_common_status', '0', '2024-08-16 06:41:56', '', '登录状态列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (100, '用户类型', 'sys_user_type', '0', '2024-08-22 15:56:17', '', '用户类型，00 系统用户，01 会员客户');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (101, '衣物类别', 'sys_cloth_cate', '0', '2024-08-29 12:31:19', '2024-08-29 12:49:05', '000服装洗护 001家居洗护 002皮具养护 003鞋靴洗护 004奢侈品洗护 005轻奢洗护');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (102, '衣物种类', 'sys_cloth_style', '0', '2024-08-29 12:48:56', '', '000上衣 001鞋 002裤子等');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (103, '会员画像', 'sys_user_tags', '0', '2024-09-01 02:46:06', '', '00 优质客户
01 不友好客户
02 事多客户');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (104, '黑灰名单', 'sys_user_identify', '0', '2024-09-01 13:49:57', '', '用户黑灰名单');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (105, '标签类别', 'sys_tag_order', '0', '2024-09-02 06:52:15', '', '001 洗前瑕疵 002 洗后预估 003 衣物颜色……');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (106, '删除状态', 'sys_del_status', '0', '2024-09-03 06:50:19', '', '删除状态列表：0 正常 2 删除');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (107, '卡券状态', 'sys_coupon_status', '0', '2024-09-03 06:56:54', '', '卡券状态列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (108, '卡券类别', 'sys_coupon_type', '0', '2024-09-03 06:59:02', '', '卡券类别列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (109, '客户可见', 'sys_coupon_customer_invalid', '0', '2024-09-03 07:32:00', '', '客户可见列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (110, '自动延期', 'sys_coupon_auto_delay', '0', '2024-09-03 07:32:21', '', '自动延期列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (111, '通知方式', 'sys_notice_method', '0', '2024-09-04 14:33:43', '', '通知方式列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (112, '通知模板类型', 'sys_temp_type', '0', '2024-09-04 14:35:50', '', '通知模板类型列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (113, '通知结果', 'sys_notice_result', '0', '2024-09-04 14:36:34', '', '通知结果列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (114, '支付方式', 'sys_payment_method', '0', '2024-09-06 00:37:19', '', '支付方式列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (115, '用户卡券状态', 'sys_uc_status', '0', '2024-09-06 00:58:54', '', '用户卡券状态列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (116, '订单类型', 'sys_price_order_type', '0', '2024-09-06 13:47:14', '', '订单类型列表，用于价格管理模块');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (117, '支付状态', 'sys_payment_status', '0', '2024-09-07 01:48:58', '', '支付状态列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (118, '时效预警', 'sys_cost_time_alarm', '0', '2024-09-07 01:50:09', '', '时效预警列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (119, '衣物颜色', 'sys_color_list', '0', '2024-09-07 11:57:38', '', '衣物颜色列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (120, '服务类型', 'sys_service_type', '0', '2024-09-08 01:12:01', '', '服务类型列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (121, '服务要求', 'sys_service_requirement', '0', '2024-09-08 01:12:26', '', '服务要求列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (122, '衣物状态', 'sys_clothing_status', '0', '2024-09-08 01:13:11', '', '衣物状态列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (123, '业务类型', 'sys_business_type', '0', '2024-09-08 13:21:42', '', '业务类型列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (124, '取回方式', 'sys_delivery_mode', '0', '2024-09-08 13:23:07', '', '取回方式列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (125, '订单状态', 'sys_order_status', '0', '2024-09-09 01:46:56', '', '订单状态列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (126, '订单类型', 'sys_order_type', '0', '2024-09-09 01:49:56', '', '订单类型列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (127, '支出类型', 'sys_exp_type', '0', '2024-09-11 01:18:58', '', '支出类型列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (128, '卡券支付方式', 'sys_coupon_payment_method', '0', '2024-09-23 12:03:24', '', null);
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (129, '服装洗护分类', 'sys_cloth_style000', '0', '2024-09-26 02:56:57', '', '服装洗护分类列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (130, '推广方式', 'sys_promote_method', '0', '2024-10-14 01:49:57', '', '推广方式列表');
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (131, '推广类型', 'sys_promote_type', '0', '2024-10-14 02:15:43', '', null);
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (132, '支付方式-展示', 'sys_payment_method_show', '0', '2024-10-16 04:50:20', '', null);
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (133, '推广状态', 'sys_promote_result', '0', '2024-10-20 05:53:02', '2024-10-20 05:53:22', null);
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (134, '家具洗护分类', 'sys_cloth_style001', '0', '2024-12-11T18:16:55.937813900+08:00', null, null);
INSERT INTO dict_type (dict_id, dict_name, dict_type, status, create_time, update_time, remark) VALUES (135, '皮具养护', 'sys_cloth_style002', '0', '2025-02-17T19:50:47.943800200+08:00', null, null);

INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (1, 4, '男', '0', 'sys_user_sex', '', '', 'Y', '0', '2024-08-16 06:41:56', null, '性别男');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (2, 2, '女', '1', 'sys_user_sex', '', '', 'N', '0', '2024-08-16 06:41:56', '', '性别女');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (3, 3, '未知', '2', 'sys_user_sex', '', '', 'N', '0', '2024-08-16 06:41:56', '', '性别未知');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (4, 1, '显示', '0', 'sys_show_hide', '', 'primary', 'Y', '0', '2024-08-16 06:41:56', '', '显示菜单');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (5, 2, '隐藏', '1', 'sys_show_hide', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '', '隐藏菜单');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (6, 1, '正常', '0', 'sys_normal_disable', '', 'primary', 'Y', '0', '2024-08-16 06:41:56', '', '正常状态');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (7, 2, '停用', '1', 'sys_normal_disable', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '', '停用状态');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (8, 1, '正常', '0', 'sys_job_status', '', 'primary', 'Y', '0', '2024-08-16 06:41:56', '', '正常状态');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (9, 2, '暂停', '1', 'sys_job_status', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '', '停用状态');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (10, 1, '默认', 'DEFAULT', 'sys_job_group', '', '', 'Y', '0', '2024-08-16 06:41:56', '', '默认分组');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (11, 2, '系统', 'SYSTEM', 'sys_job_group', '', '', 'N', '0', '2024-08-16 06:41:56', '', '系统分组');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (12, 1, '是', 'Y', 'sys_yes_no', '', 'primary', 'Y', '0', '2024-08-16 06:41:56', '', '系统默认是');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (13, 2, '否', 'N', 'sys_yes_no', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '', '系统默认否');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (14, 1, '通知', '1', 'sys_notice_type', '', 'warning', 'Y', '0', '2024-08-16 06:41:56', '', '通知');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (15, 2, '公告', '2', 'sys_notice_type', '', 'success', 'N', '0', '2024-08-16 06:41:56', '', '公告');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (16, 1, '正常', '0', 'sys_notice_status', '', 'primary', 'Y', '0', '2024-08-16 06:41:56', '', '正常状态');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (17, 2, '关闭', '1', 'sys_notice_status', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '', '关闭状态');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (18, 99, '其他', '0', 'sys_oper_type', '', 'info', 'N', '0', '2024-08-16 06:41:56', '', '其他操作');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (19, 1, '新增', '1', 'sys_oper_type', '', 'info', 'N', '0', '2024-08-16 06:41:56', '', '新增操作');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (20, 2, '修改', '2', 'sys_oper_type', '', 'info', 'N', '0', '2024-08-16 06:41:56', '', '修改操作');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (21, 3, '删除', '3', 'sys_oper_type', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '', '删除操作');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (22, 4, '授权', '4', 'sys_oper_type', '', 'primary', 'N', '0', '2024-08-16 06:41:56', '', '授权操作');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (23, 5, '导出', '5', 'sys_oper_type', '', 'warning', 'N', '0', '2024-08-16 06:41:56', '', '导出操作');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (24, 6, '导入', '6', 'sys_oper_type', '', 'warning', 'N', '0', '2024-08-16 06:41:56', '', '导入操作');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (25, 7, '强退', '7', 'sys_oper_type', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '', '强退操作');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (26, 8, '生成代码', '8', 'sys_oper_type', '', 'warning', 'N', '0', '2024-08-16 06:41:56', '', '生成操作');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (27, 9, '清空数据', '9', 'sys_oper_type', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '', '清空操作');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (28, 1, '成功', '0', 'sys_common_status', '', 'primary', 'N', '0', '2024-08-16 06:41:56', '', '正常状态');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (29, 2, '失败', '1', 'sys_common_status', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '', '停用状态');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (100, 1, '系统用户', '00', 'sys_user_type', null, 'primary', 'N', '0', '2024-08-22 15:57:50', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (101, 2, '会员客户', '01', 'sys_user_type', null, 'success', 'N', '0', '2024-08-22 15:58:09', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (102, 0, '服装洗护', '000', 'sys_cloth_cate', null, 'success', 'N', '0', '2024-08-29 12:33:39', '2024-08-29 12:46:07', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (103, 1, '家居洗护', '001', 'sys_cloth_cate', null, 'default', 'N', '0', '2024-08-29 12:34:04', '2024-08-29 12:34:19', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (104, 2, '皮具养护', '002', 'sys_cloth_cate', null, 'default', 'N', '0', '2024-08-29 12:34:46', '2024-08-29 12:34:53', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (105, 3, '鞋靴洗护', '003', 'sys_cloth_cate', null, 'default', 'N', '0', '2024-08-29 12:35:10', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (106, 4, '奢侈品洗护', '004', 'sys_cloth_cate', null, 'default', 'N', '0', '2024-08-29 12:35:23', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (107, 5, '轻奢洗护', '005', 'sys_cloth_cate', null, 'default', 'N', '0', '2024-08-29 12:35:38', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (108, 0, '上衣', '000', 'sys_cloth_style', null, 'default', 'N', '0', '2024-08-29 12:49:24', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (109, 1, '鞋', '001', 'sys_cloth_style', null, 'default', 'N', '0', '2024-08-29 12:49:39', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (110, 2, '裤子', '002', 'sys_cloth_style', null, 'default', 'N', '0', '2024-08-29 12:49:52', '2024-09-03 01:40:15', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (111, 0, '优质客户', '00', 'sys_user_tags', null, 'success', 'N', '0', '2024-09-01 02:46:49', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (112, 1, '不友好客户', '01', 'sys_user_tags', null, 'danger', 'N', '0', '2024-09-01 02:47:16', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (113, 2, '事多客户', '02', 'sys_user_tags', null, 'warning', 'N', '0', '2024-09-01 02:47:37', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (114, 3, '其他', '03', 'sys_user_tags', null, 'primary', 'N', '0', '2024-09-01 02:47:53', '2024-09-01 03:50:09', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (115, 0, '正常', '00', 'sys_user_identify', null, 'success', 'N', '0', '2024-09-01 13:50:28', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (116, 1, '黑名单', '01', 'sys_user_identify', null, 'danger', 'N', '0', '2024-09-01 13:51:05', '', '黑名单，不允许消费');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (117, 2, '灰名单', '02', 'sys_user_identify', null, 'warning', 'N', '0', '2024-09-01 13:51:46', '', '灰名单只能到店消费，不能小程序自助下单');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (118, 0, '洗前瑕疵', '001', 'sys_tag_order', null, 'warning', 'N', '0', '2024-09-02 06:52:48', '', '001 洗前瑕疵');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (119, 1, '洗后预估', '002', 'sys_tag_order', null, 'primary', 'N', '0', '2024-09-02 06:53:09', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (120, 2, '衣物颜色', '003', 'sys_tag_order', null, 'success', 'N', '0', '2024-09-02 06:53:50', '2024-09-07 13:37:06', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (121, 0, '正常', '0', 'sys_del_status', null, 'success', 'N', '0', '2024-09-03 06:50:36', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (122, 1, '已删除', '2', 'sys_del_status', null, 'danger', 'N', '0', '2024-09-03 06:50:52', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (123, 0, '正常销售', '0', 'sys_coupon_status', null, 'success', 'N', '0', '2024-09-03 06:57:33', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (124, 1, '不可销售', '1', 'sys_coupon_status', null, 'danger', 'N', '0', '2024-09-03 06:58:01', '2024-09-03 06:58:32', '代表由于编辑生成新的卡券后，此卡券变为不可销售，且不可编辑修改');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (125, 2, '暂不销售', '2', 'sys_coupon_status', null, 'warning', 'N', '0', '2024-09-03 06:58:24', '', '代表店家提前创建好了卡券，但是暂时不对外售卖');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (126, 0, '储值卡', '000', 'sys_coupon_type', null, 'primary', 'N', '0', '2024-09-03 06:59:26', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (127, 1, '福利卡', '001', 'sys_coupon_type', null, 'primary', 'N', '1', '2024-09-03 06:59:49', '2024-09-24 03:12:48', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (128, 2, '次卡', '002', 'sys_coupon_type', null, 'primary', 'N', '0', '2024-09-03 07:00:06', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (129, 3, '折扣券', '003', 'sys_coupon_type', null, 'primary', 'N', '0', '2024-09-03 07:00:23', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (130, 4, '满减券', '004', 'sys_coupon_type', null, 'primary', 'N', '0', '2024-09-03 07:00:37', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (131, 0, '是', '0', 'sys_coupon_auto_delay', null, 'success', 'N', '0', '2024-09-03 07:33:22', '', '自动延期');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (132, 1, '否', '2', 'sys_coupon_auto_delay', null, 'danger', 'N', '0', '2024-09-03 07:33:38', '', '不自动延期');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (133, 0, '可见', '0', 'sys_coupon_customer_invalid', null, 'success', 'N', '0', '2024-09-03 07:33:58', '', '客户可见');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (134, 1, '不可见', '2', 'sys_coupon_customer_invalid', null, 'danger', 'N', '0', '2024-09-03 07:34:12', '', '客户不可见');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (135, 0, '短信', '0', 'sys_notice_method', null, 'primary', 'N', '0', '2024-09-04 14:37:13', '', '短信通知');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (136, 1, '小程序', '1', 'sys_notice_method', null, 'success', 'N', '0', '2024-09-04 14:37:31', '', '小程序通知');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (137, 0, '取衣通知', '0', 'sys_temp_type', null, 'success', 'N', '0', '2024-09-04 14:37:56', '2024-09-28 06:42:58', '取衣通知');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (138, 1, '推广', '1', 'sys_temp_type', null, 'primary', 'N', '0', '2024-09-04 14:38:13', '2024-09-28 06:43:14', '推广');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (139, 0, '发送成功', '0', 'sys_notice_result', null, 'success', 'N', '0', '2024-09-04 14:38:31', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (140, 1, '发送失败', '1', 'sys_notice_result', null, 'danger', 'N', '0', '2024-09-04 14:38:44', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (141, 0, '支付宝', '01', 'sys_payment_method', null, 'success', 'N', '0', '2024-09-06 00:37:41', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (142, 1, '微信', '02', 'sys_payment_method', null, 'success', 'N', '0', '2024-09-06 00:37:55', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (143, 2, '美团转结', '03', 'sys_payment_method', null, 'success', 'N', '0', '2024-09-06 00:38:17', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (144, 3, '抖音结转', '04', 'sys_payment_method', null, 'success', 'N', '0', '2024-09-06 00:38:36', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (145, 4, '现金支付', '05', 'sys_payment_method', null, 'success', 'N', '0', '2024-09-06 00:39:22', '2024-10-05 02:00:00', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (146, 5, '储值卡', '06', 'sys_payment_method', null, 'primary', 'N', '0', '2024-09-06 00:39:39', '2024-10-05 02:00:14', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (147, 0, '正常', '00', 'sys_uc_status', null, 'primary', 'N', '0', '2024-09-06 00:59:43', '', '卡券未退款');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (148, 1, '已退款', '01', 'sys_uc_status', null, 'danger', 'N', '0', '2024-09-06 01:00:02', '', '已经退款');
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (149, 0, '小程序', '00', 'sys_price_order_type', null, 'success', 'N', '0', '2024-09-06 13:47:44', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (150, 1, '美团', '01', 'sys_price_order_type', null, 'primary', 'N', '0', '2024-09-06 13:48:03', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (151, 2, '抖音', '02', 'sys_price_order_type', null, 'info', 'N', '0', '2024-09-06 13:48:19', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (152, 3, '到店', '03', 'sys_price_order_type', null, 'success', 'N', '0', '2024-09-06 13:48:35', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (153, 4, '其他', '04', 'sys_price_order_type', null, 'warning', 'N', '0', '2024-09-06 13:48:52', '2024-09-06 13:49:02', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (154, 0, '已支付', '00', 'sys_payment_status', null, 'success', 'N', '0', '2024-09-07 01:49:15', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (155, 1, '未支付', '01', 'sys_payment_status', null, 'danger', 'N', '0', '2024-09-07 01:49:26', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (156, 0, '正常', '00', 'sys_cost_time_alarm', null, 'success', 'N', '0', '2024-09-07 01:50:24', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (157, 1, '即将超时', '01', 'sys_cost_time_alarm', null, 'warning', 'N', '0', '2024-09-07 01:50:43', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (158, 2, '已超时', '02', 'sys_cost_time_alarm', null, 'danger', 'N', '0', '2024-09-07 01:50:58', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (159, 0, '红色', '001', 'sys_color_list', null, 'default', 'N', '0', '2024-09-07 11:58:08', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (160, 1, '黄色', '002', 'sys_color_list', null, 'default', 'N', '0', '2024-09-07 11:58:21', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (161, 2, '蓝色', '003', 'sys_color_list', null, 'default', 'N', '0', '2024-09-07 11:58:34', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (162, 3, '绿色', '004', 'sys_color_list', null, 'default', 'N', '0', '2024-09-07 11:58:51', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (163, 0, '其他', '000', 'sys_color_list', null, 'default', 'N', '0', '2024-09-07 11:59:06', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (164, 3, '品牌', '004', 'sys_tag_order', null, 'info', 'N', '0', '2024-09-07 13:36:59', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (165, 0, '已取走', '00', 'sys_clothing_status', null, 'success', 'N', '0', '2024-09-08 01:13:33', '2024-09-13 13:20:25', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (166, 1, '洗护中', '01', 'sys_clothing_status', null, 'primary', 'N', '0', '2024-09-08 01:13:46', '2024-09-13 13:20:20', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (167, 2, '已上挂', '02', 'sys_clothing_status', null, 'success', 'N', '0', '2024-09-08 01:14:02', '2024-09-13 13:20:15', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (168, 0, '常规', '000', 'sys_service_requirement', null, 'success', 'N', '0', '2024-09-08 01:14:23', '2024-09-08 01:15:15', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (169, 1, '加急', '001', 'sys_service_requirement', null, 'danger', 'N', '0', '2024-09-08 01:14:46', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (170, 2, '单洗', '002', 'sys_service_requirement', null, 'primary', 'N', '0', '2024-09-08 01:15:08', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (171, 3, '其他', '003', 'sys_service_requirement', null, 'info', 'N', '0', '2024-09-08 01:15:40', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (172, 0, '洗护', '000', 'sys_service_type', null, 'primary', 'N', '0', '2024-09-08 01:16:03', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (173, 1, '熨烫', '001', 'sys_service_type', null, 'primary', 'N', '0', '2024-09-08 01:16:20', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (174, 2, '扦裤脚', '002', 'sys_service_type', null, 'primary', 'N', '0', '2024-09-08 01:16:37', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (175, 3, '其他', '003', 'sys_service_type', null, 'info', 'N', '0', '2024-09-08 01:16:55', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (176, 0, '主营业务', '00', 'sys_business_type', null, 'success', 'N', '0', '2024-09-08 13:22:02', '2024-09-09 01:29:12', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (177, 1, '增值业务', '01', 'sys_business_type', null, 'primary', 'N', '0', '2024-09-08 13:22:17', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (178, 0, '客户自取', '00', 'sys_delivery_mode', null, 'primary', 'N', '0', '2024-09-08 13:23:23', '2024-09-08 13:24:04', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (179, 1, '上门派送', '01', 'sys_delivery_mode', null, 'warning', 'N', '0', '2024-09-08 13:23:37', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (180, 2, '快递邮寄', '02', 'sys_delivery_mode', null, 'danger', 'N', '0', '2024-09-08 13:23:55', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (181, 2, '其他', '02', 'sys_business_type', null, 'info', 'N', '0', '2024-09-09 01:29:26', '2024-09-09 01:29:34', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (182, 0, '正在洗护', '01', 'sys_order_status', null, 'primary', 'N', '0', '2024-09-09 01:47:20', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (183, 1, '上挂待取', '02', 'sys_order_status', null, 'success', 'N', '0', '2024-09-09 01:47:35', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (184, 2, '上挂待送', '03', 'sys_order_status', null, 'primary', 'N', '0', '2024-09-09 01:48:02', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (185, 3, '已完成', '04', 'sys_order_status', null, 'success', 'N', '0', '2024-09-09 01:48:17', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (186, 4, '已退单', '05', 'sys_order_status', null, 'danger', 'N', '0', '2024-09-09 01:48:29', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (187, 5, '退单退款', '06', 'sys_order_status', null, 'danger', 'N', '0', '2024-09-09 01:49:04', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (188, 0, '正常订单', '00', 'sys_order_type', null, 'primary', 'N', '0', '2024-09-09 01:50:10', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (189, 1, '售后质保', '02', 'sys_order_type', null, 'warning', 'N', '0', '2024-09-09 01:50:31', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (190, 0, '订单退款', '00', 'sys_exp_type', null, 'default', 'N', '0', '2024-09-11 01:19:48', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (191, 1, '事故赔偿', '01', 'sys_exp_type', null, 'default', 'N', '0', '2024-09-11 01:19:58', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (192, 2, '经营支出', '02', 'sys_exp_type', null, 'default', 'N', '0', '2024-09-11 01:20:16', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (193, 3, '卡券退款', '03', 'sys_exp_type', null, 'default', 'N', '0', '2024-09-11 01:20:39', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (194, 4, '水电支出', '04', 'sys_exp_type', null, 'default', 'N', '0', '2024-09-11 01:20:58', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (195, 3, '已退单', '03', 'sys_clothing_status', null, 'danger', 'N', '0', '2024-09-13 13:20:00', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (196, 0, '支付宝', '01', 'sys_coupon_payment_method', null, 'success', 'N', '0', '2024-09-23 12:04:55', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (197, 1, '微信', '02', 'sys_coupon_payment_method', null, 'success', 'N', '0', '2024-09-23 12:05:08', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (198, 2, '现金', '05', 'sys_coupon_payment_method', null, 'success', 'N', '0', '2024-09-23 12:05:27', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (199, 3, '其他', '06', 'sys_coupon_payment_method', null, 'success', 'N', '0', '2024-09-23 12:05:41', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (200, 0, '上衣', '0', 'sys_cloth_style000', null, 'default', 'N', '0', '2024-09-26 02:57:37', '2025-02-17T19:46:16.271405600+08:00', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (201, 1, '裤子', '1', 'sys_cloth_style000', null, 'default', 'N', '0', '2024-09-26 02:57:45', '2025-02-17T19:46:24.781881500+08:00', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (202, 2, '卫衣', '2', 'sys_cloth_style000', null, 'default', 'N', '0', '2024-09-26 02:57:57', '2025-02-17T19:46:31.678427400+08:00', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (203, 2, '其他', '2', 'sys_temp_type', null, 'warning', 'N', '0', '2024-09-28 06:43:36', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (204, 2, '发送中...', '2', 'sys_notice_result', null, 'primary', 'N', '0', '2024-09-28 07:00:45', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (205, 7, '其他', '09', 'sys_payment_method', null, 'info', 'N', '0', '2024-10-05 02:00:57', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (206, 6, '次卡', '07', 'sys_payment_method', null, 'success', 'N', '0', '2024-10-05 02:05:20', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (207, 0, '小程序', '00', 'sys_promote_method', null, 'primary', 'N', '0', '2024-10-14 01:51:20', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (208, 1, '短信', '01', 'sys_promote_method', null, 'success', 'N', '0', '2024-10-14 01:51:32', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (209, 2, '其他', '02', 'sys_promote_method', null, 'default', 'N', '0', '2024-10-14 01:51:44', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (210, 0, '赠券', '00', 'sys_promote_type', null, 'primary', 'N', '0', '2024-10-14 02:16:06', '2024-10-16 13:28:56', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (211, 0, '支付宝', '01', 'sys_payment_method_show', null, 'primary', 'N', '0', '2024-10-16 04:50:36', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (212, 1, '微信', '02', 'sys_payment_method_show', null, 'success', 'N', '0', '2024-10-16 04:50:48', '2024-10-16 04:58:15', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (213, 2, '美团结转', '03', 'sys_payment_method_show', null, 'info', 'N', '0', '2024-10-16 04:51:05', '2024-10-16 04:58:10', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (214, 3, '抖音结转', '04', 'sys_payment_method_show', null, 'info', 'N', '0', '2024-10-16 04:51:17', '2024-10-16 04:57:57', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (215, 4, '现金支付', '05', 'sys_payment_method_show', null, 'warning', 'N', '0', '2024-10-16 04:51:33', '2024-10-16 04:57:50', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (216, 5, '储值卡', '06', 'sys_payment_method_show', null, 'danger', 'N', '0', '2024-10-16 04:51:48', '2024-10-16 04:57:20', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (217, 6, '次卡', '07', 'sys_payment_method_show', null, 'danger', 'N', '0', '2024-10-16 04:52:00', '2024-10-16 04:57:32', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (218, 7, '其他', '09', 'sys_payment_method_show', null, 'info', 'N', '0', '2024-10-16 04:52:14', '2024-10-16 04:56:55', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (219, 8, '支付宝+储值卡', '16', 'sys_payment_method_show', null, 'primary', 'N', '0', '2024-10-16 04:52:48', '2024-10-16 04:53:06', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (220, 9, '支付宝+次卡', '17', 'sys_payment_method_show', null, 'primary', 'N', '0', '2024-10-16 04:53:29', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (221, 10, '支付宝+折扣券', '18', 'sys_payment_method_show', null, 'primary', 'N', '0', '2024-10-16 04:53:47', '2024-10-16 06:37:26', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (222, 12, '微信+储值卡', '26', 'sys_payment_method_show', null, 'success', 'N', '0', '2024-10-16 04:54:36', '2024-10-16 06:42:38', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (223, 13, '微信+次卡', '27', 'sys_payment_method_show', null, 'success', 'N', '0', '2024-10-16 04:54:54', '2024-10-16 06:42:46', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (224, 14, '微信+折扣券', '28', 'sys_payment_method_show', null, 'success', 'N', '0', '2024-10-16 04:55:11', '2024-10-16 06:43:39', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (225, 16, '现金+储值卡', '56', 'sys_payment_method_show', null, 'warning', 'N', '0', '2024-10-16 04:55:39', '2024-10-16 06:43:45', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (226, 17, '现金+次卡', '57', 'sys_payment_method_show', null, 'warning', 'N', '0', '2024-10-16 04:55:59', '2024-10-16 06:43:49', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (227, 18, '现金+折扣券', '58', 'sys_payment_method_show', null, 'warning', 'N', '0', '2024-10-16 04:56:20', '2024-10-16 06:44:24', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (228, 11, '支付宝+满减券', '19', 'sys_payment_method_show', null, 'primary', 'N', '0', '2024-10-16 06:42:17', '2024-10-16 06:42:31', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (229, 15, '微信+满减券', '29', 'sys_payment_method_show', null, 'success', 'N', '0', '2024-10-16 06:43:28', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (230, 19, '现金+满减券', '59', 'sys_payment_method_show', null, 'warning', 'N', '0', '2024-10-16 06:44:16', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (231, 0, '成功', '0', 'sys_promote_result', null, 'success', 'N', '0', '2024-10-20 05:53:34', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (232, 1, '失败', '1', 'sys_promote_result', null, 'danger', 'N', '0', '2024-10-20 05:53:44', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (233, 2, '部分成功', '2', 'sys_promote_result', null, 'warning', 'N', '0', '2024-10-20 05:53:56', '', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (234, 0, '喇叭库', '004', 'sys_cloth_style', null, 'default', null, '0', '2024-12-09T14:33:34.460359186+08:00', null, null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (235, 0, '沙发垫', '0', 'sys_cloth_style001', null, 'primary', null, '0', '2024-12-11T18:17:18.591259100+08:00', '2025-02-17T19:46:05.221694800+08:00', null);
INSERT INTO dict_data (dict_code, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark) VALUES (236, 2, '已退单', '05', 'sys_payment_status', null, 'warning', null, '0', '2024-12-15T10:57:59.500259700+08:00', null, null);

INSERT INTO membership_level (level_id, level_code, level_name, level_sort, status, create_time, update_time, remark)
VALUES (1, 'dpyy', '店铺经营', 1, '0', '2024-08-16 06:41:56', '2024-08-22 15:24:56', '实际最终店铺的辅助经营人员');
INSERT INTO membership_level (level_id, level_code, level_name, level_sort, status, create_time, update_time, remark)
VALUES (2, 'dpdz', '店铺店长', 2, '0', '2024-08-16 06:41:56', '2024-08-22 15:24:23', '店铺的实际店长，即店铺负责人');
INSERT INTO membership_level (level_id, level_code, level_name, level_sort, status, create_time, update_time, remark)
VALUES (3, 'vip-pt', '普通会员', 3, '0', '2024-08-16 06:41:56', '2024-08-22 15:25:33', '客户会员等级 - 普通会员');
INSERT INTO membership_level (level_id, level_code, level_name, level_sort, status, create_time, update_time, remark)
VALUES (4, 'vip-by', '白银会员', 4, '0', '2024-08-16 06:41:56', '2024-08-22 15:25:57', '客户会员等级 - 白银会员');
INSERT INTO membership_level (level_id, level_code, level_name, level_sort, status, create_time, update_time, remark)
VALUES (5, 'vip-hj', '黄金会员', 5, '0', '2024-08-22 15:26:30', null, '客户会员等级 - 黄金会员');
INSERT INTO membership_level (level_id, level_code, level_name, level_sort, status, create_time, update_time, remark)
VALUES (6, 'vip-zs', '钻石会员', 6, '0', '2024-08-22 15:26:53', null, '客户会员等级 - 钻石会员');
INSERT INTO membership_level (level_id, level_code, level_name, level_sort, status, create_time, update_time, remark)
VALUES (7, 'gsyyzy', '公司运营专员', 7, '0', '2024-08-22 15:29:21', '2024-08-22 15:30:52',
        '公司运营专员，无实体店铺，主要管理系统中各分店铺的账号与数据');
INSERT INTO membership_level (level_id, level_code, level_name, level_sort, status, create_time, update_time, remark)
VALUES (8, 'boss', 'BOSS', 8, '0', '2024-08-22 15:30:43', '2024-08-22 15:30:58', '最高权限的管理，用于公司高级领导');

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
    owner_name      TEXT    NOT NULL,
    avatar          TEXT    NOT NULL,
    owner_phone     TEXT    NOT NULL,
    password        TEXT    NOT NULL,
    role            TEXT    NOT NULL,
    first_login     INTEGER,
    store_name      TEXT NOT NULL,
    store_location  TEXT NOT NULL,
    owner_gender    TEXT,
    store_status    TEXT NOT NULL,
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

CREATE TABLE IF NOT EXISTS menu
(
    menu_id     INTEGER PRIMARY KEY AUTOINCREMENT, -- 菜单ID
    menu_name   TEXT NOT NULL,                     -- 菜单名称
    parent_id   INTEGER  DEFAULT 0,                -- 父菜单ID
    order_num   INTEGER  DEFAULT 0,                -- 显示顺序
    path        TEXT     DEFAULT '',               -- 路由地址
    component   TEXT     DEFAULT NULL,             -- 组件路径
    query       TEXT     DEFAULT NULL,             -- 路由参数
    route_name  TEXT     DEFAULT '',               -- 路由名称
    is_frame    INTEGER  DEFAULT 1,                -- 是否为外链（0是 1否）
    is_cache    INTEGER  DEFAULT 0,                -- 是否缓存（0缓存 1不缓存）
    menu_type   TEXT     DEFAULT '',               -- 菜单类型（M目录 C菜单 F按钮）
    visible     TEXT     DEFAULT '0',              -- 菜单状态（0显示 1隐藏）
    status      TEXT     DEFAULT '0',              -- 菜单状态（0正常 1停用）
    perms       TEXT     DEFAULT NULL,             -- 权限标识
    icon        TEXT     DEFAULT '#',              -- 菜单图标
    create_by   TEXT     DEFAULT '',               -- 创建者
    create_time DATETIME DEFAULT NULL,             -- 创建时间
    update_by   TEXT     DEFAULT '',               -- 更新者
    update_time DATETIME DEFAULT NULL,             -- 更新时间
    remark      TEXT     DEFAULT ''                -- 备注
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

-- 会员画像表
CREATE TABLE user_tags
(
    user_id INTEGER NOT NULL,
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
    tag_number TEXT UNIQUE NOT NULL,
    tag_order  TEXT,
    tag_name   TEXT UNIQUE NOT NULL,
    ref_num    INTEGER DEFAULT 0,
    order_num  INTEGER DEFAULT 0,
    status     TEXT    DEFAULT '0',
    del_flag   TEXT    DEFAULT '0',
    remark     TEXT
);

-- 衣物管理表
CREATE TABLE clothing
(
    clothing_id         INTEGER PRIMARY KEY AUTOINCREMENT,
    clothing_category   TEXT   NOT NULL,
    clothing_number     TEXT   NOT NULL,
    clothing_style      TEXT   NOT NULL,
    clothing_name       TEXT   NOT NULL,
    clothing_base_price DOUBLE NOT NULL,
    clothing_min_price  DOUBLE NOT NULL,
    order_num           INTEGER         DEFAULT 0,
    clothing_degree     INTEGER         DEFAULT 0,
    hang_type           TEXT   NOT NULL DEFAULT '1',
    del_flag            TEXT            DEFAULT '0',
    remark              TEXT
);

-- 创建索引，提高根据衣物类别和衣物名称查询效率
CREATE INDEX idx_clothing_category ON clothing (clothing_category);

CREATE INDEX idx_clothing_name ON clothing (clothing_name);

-- 卡券管理表
CREATE TABLE coupons
(
    coupon_id           INTEGER PRIMARY KEY AUTOINCREMENT,
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

-- 用户卡券关联表
CREATE TABLE user_coupons
(
    uc_id           INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER NOT NULL,
    coupon_id       INTEGER NOT NULL,
    create_time     TIMESTAMP,
    obtain_at       TIMESTAMP DEFAULT NULL,
    available_value DOUBLE    DEFAULT 0,
    uc_count        INTEGER   DEFAULT 1,
    pay_id          INTEGER   DEFAULT NULL,
    uc_type         TEXT      DEFAULT '01',
    status          TEXT      DEFAULT '00',
    remark          TEXT      DEFAULT NULL
);

-- 用户卡券索引
CREATE INDEX idx_user_coupons_user_id_user_coupons ON user_coupons (user_id);

CREATE INDEX idx_coupon_id ON user_coupons (coupon_id);

CREATE INDEX idx_status ON user_coupons (status);

CREATE INDEX idx_user_status ON user_coupons (user_id, status);

-- 卡券订单表
CREATE TABLE coupon_orders
(
    order_id    INTEGER PRIMARY KEY AUTOINCREMENT,
    uc_id       TEXT NOT NULL,
    create_time TIMESTAMP
);

-- 卡券订单索引
CREATE INDEX idx_uc_id ON coupon_orders (uc_id);

CREATE INDEX idx_coupon_orders_create_time ON coupon_orders (create_time);

CREATE INDEX idx_create_uc ON coupon_orders (create_time, uc_id);

-- 支付记录表
CREATE TABLE payments
(
    pay_id             INTEGER PRIMARY KEY AUTOINCREMENT,
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
    create_time        TIMESTAMP,
    update_time        TIMESTAMP,
    order_status       TEXT   NOT NULL
);

CREATE INDEX idx_payments_order_type ON payments (order_type); -- 支付记录索引
CREATE INDEX idx_payments_payment_status ON payments (payment_status);

CREATE INDEX idx_pay_number ON payments (pay_number);

CREATE INDEX idx_payments_create_time ON payments (create_time);

CREATE INDEX idx_order_status ON payments (order_type, payment_status);

CREATE INDEX idx_pay_number_order ON payments (pay_number, order_type);

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

-- 创建索引
CREATE INDEX idx_notice_record_user_id ON notice_record (user_id);

-- 衣物价格表
CREATE TABLE cloth_price
(
    price_id       INTEGER PRIMARY KEY AUTOINCREMENT,
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

-- 订单表
CREATE TABLE orders
(
    order_id             INTEGER PRIMARY KEY AUTOINCREMENT,
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

-- 衣物清单表
CREATE TABLE order_clothes
(
    cloth_id            INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id            INTEGER NULL,
    clothing_id         INTEGER NOT NULL,
    clothing_category   TEXT    NOT NULL,
    clothing_style      TEXT    NOT NULL,
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
    remark              TEXT
);

-- 创建索引
CREATE INDEX idx_order_clothing_id ON order_clothes (order_id);

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

-- 支付记录表
CREATE TABLE order_pays
(
    pay_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id    INTEGER NOT NULL,
    create_by   TEXT,
    create_time TIMESTAMP,
    update_by   TEXT,
    update_time TIMESTAMP,
    remark      TEXT,
    pay_amount  FLOAT   NOT NULL,
    pay_method  TEXT    NOT NULL
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
CREATE TABLE order_dispatch
(
    dispatch_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id      TEXT NOT NULL,
    cloth_id      TEXT NOT NULL,
    delivery_comp TEXT,
    delivery_num  TEXT,
    dispatch_time TIMESTAMP,
    remark        TEXT,
    create_time   TIMESTAMP
);

-- 支出记录表
CREATE TABLE expenditure
(
    exp_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id           TEXT,
    cloth_ids          TEXT,
    exp_title          TEXT    NOT NULL,
    recv_account       INTEGER,
    recv_account_title TEXT,
    exp_type           TEXT    NOT NULL,
    exp_amount         INTEGER NOT NULL,
    create_time        TIMESTAMP,
    remark             TEXT
);

-- 晾衣架表
CREATE TABLE drying_rack
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    name               TEXT    NOT NULL,
    rack_type          TEXT             DEFAULT '1',
    capacity           INTEGER NOT NULL,
    remaining_capacity INTEGER NOT NULL,
    position           INTEGER NOT NULL DEFAULT 0
);

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

-- INSERT INTO local_users (id, username, avatar, account, password, role, remark, first_login) VALUES (1, 'admin', 'images/avatars/avatar1.png', 'admin', '$argon2id$v=19$m=19456,t=2,p=1$xmj1234xmj1234xmj1234xmj1234xmj1234$xcjB3Mxf+IbNnfZMiHNE+XrGRv+bWdmyiM6uTbZ1Mxs', 'admin', null, 1);

INSERT INTO configs (config_id, config_name, config_key, config_value, config_type, create_by, create_time, update_by, update_time, remark) VALUES (1, '账号自助-验证码开关', 'sys.account.captchaEnabled', 'false', 'Y', 'admin', null, '', null, '是否开启验证码功能（true开启，false关闭）');
INSERT INTO configs (config_id, config_name, config_key, config_value, config_type, create_by, create_time, update_by, update_time, remark) VALUES (2, '预计取衣事件', 'desire_complete_time', '17', 'Y', null, '2025-02-15T10:22:13.032273400+08:00', null, '2025-02-15T10:23:31.618923900+08:00', '默认七天后取衣');
INSERT INTO configs (config_id, config_name, config_key, config_value, config_type, create_by, create_time, update_by, update_time, remark) VALUES (3, '页面无操作注销时间', 'logout_timeout', '600', 'Y', null, '2025-02-15T11:40:01.890123500+08:00', null, null, '单位：秒');


INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1, '系统管理', 0, 1, '/system', null, '', '', '1', '0', 'M', '0', '0', '', '/system', 'admin', '2024-08-16 06:41:56', '', null, '系统管理目录');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2, '系统监控', 0, 9, 'monitor', null, '', '', '1', '0', 'M', '1', '1', '', 'monitor', 'admin', '2024-08-16 06:41:56', 'admin', '2024-12-07T11:33:16.780350800+08:00', '系统监控目录');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (3, '系统工具', 0, 12, 'tool', null, '', '', '1', '0', 'M', '1', '1', '', 'tool', 'admin', '2024-08-16 06:41:56', 'admin', '2024-12-07T11:33:28.219474100+08:00', '系统工具目录');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (100, '会员管理', 0, 5, 'user', 'system/user/index', '', '', '1', '1', 'C', '0', '0', '', 'user', 'admin', '2024-08-16 06:41:56', 'admin', '2024-12-06T11:20:38.294438400+08:00', '用户管理菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (101, '角色管理', 1, 2, 'role', 'system/role/index', '', '', '1', '0', 'C', '1', '1', 'system:role:list', 'peoples', 'admin', '2024-08-16 06:41:56', '', '2024-12-07T11:31:00.561549500+08:00', '角色管理菜单');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (102, '菜单管理', 1, 3, 'menu', 'system/menu/index', '', '', '1', '0', 'C', '0', '0', 'system:menu:list', 'tree-table', 'admin', '2024-08-16 06:41:56', '', null, '菜单管理菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (103, '组织管理', 1, 4, 'dept', 'system/dept/index', '', '', '1', '0', 'C', '1', '1', 'system:dept:list', 'tree', 'admin', '2024-08-16 06:41:56', 'admin', '2024-12-07T11:31:14.087201+08:00', '部门管理菜单');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (104, '等级管理', 1, 5, 'post', 'system/post/index', '', '', '1', '0', 'C', '0', '0', 'system:post:list', 'post', 'admin', '2024-08-16 06:41:56', 'admin', '2024-08-22 15:31:54', '岗位管理菜单');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (105, '字典管理', 1, 6, 'dict', 'system/dict/index', '', '', '1', '0', 'C', '0', '0', 'system:dict:list', 'dict', 'admin', '2024-08-16 06:41:56', '', null, '字典管理菜单');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (106, '参数设置', 1, 7, 'config', 'system/config/index', '', '', '1', '0', 'C', '0', '0', 'system:config:list', 'edit', 'admin', '2024-08-16 06:41:56', '', null, '参数设置菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (107, '通知公告', 1, 8, 'notice', 'system/notice/index', '', '', '1', '0', 'C', '1', '1', 'system:notice:list', 'message', 'admin', '2024-08-16 06:41:56', '', '2024-12-07T11:32:47.141147+08:00', '通知公告菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1012, '菜单查询', 102, 1, '', '', '', '', '1', '0', 'F', '0', '0', 'system:menu:query', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1013, '菜单新增', 102, 2, '', '', '', '', '1', '0', 'F', '0', '0', 'system:menu:add', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1014, '菜单修改', 102, 3, '', '', '', '', '1', '0', 'F', '0', '0', 'system:menu:edit', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1015, '菜单删除', 102, 4, '', '', '', '', '1', '0', 'F', '0', '0', 'system:menu:remove', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1016, '部门查询', 103, 1, '', '', '', '', '1', '0', 'F', '0', '0', 'system:dept:query', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1017, '部门新增', 103, 2, '', '', '', '', '1', '0', 'F', '0', '0', 'system:dept:add', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1018, '部门修改', 103, 3, '', '', '', '', '1', '0', 'F', '0', '0', 'system:dept:edit', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1019, '部门删除', 103, 4, '', '', '', '', '1', '0', 'F', '0', '0', 'system:dept:remove', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1020, '岗位查询', 104, 1, '', '', '', '', '1', '0', 'F', '0', '0', 'system:post:query', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1021, '岗位新增', 104, 2, '', '', '', '', '1', '0', 'F', '0', '0', 'system:post:add', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1022, '岗位修改', 104, 3, '', '', '', '', '1', '0', 'F', '0', '0', 'system:post:edit', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1023, '岗位删除', 104, 4, '', '', '', '', '1', '0', 'F', '0', '0', 'system:post:remove', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1024, '岗位导出', 104, 5, '', '', '', '', '1', '0', 'F', '0', '0', 'system:post:export', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1025, '字典查询', 105, 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:dict:query', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1026, '字典新增', 105, 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:dict:add', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1027, '字典修改', 105, 3, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:dict:edit', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1028, '字典删除', 105, 4, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:dict:remove', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1029, '字典导出', 105, 5, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:dict:export', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1030, '参数查询', 106, 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:config:query', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1031, '参数新增', 106, 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:config:add', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1032, '参数修改', 106, 3, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:config:edit', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1033, '参数删除', 106, 4, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:config:remove', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1034, '参数导出', 106, 5, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:config:export', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1035, '公告查询', 107, 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:notice:query', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1036, '公告新增', 107, 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:notice:add', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1037, '公告修改', 107, 3, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:notice:edit', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1038, '公告删除', 107, 4, '#', '', '', '', '1', '0', 'F', '0', '0', 'system:notice:remove', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1039, '操作查询', 500, 1, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:operlog:query', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (1040, '操作删除', 500, 2, '#', '', '', '', '1', '0', 'F', '0', '0', 'monitor:operlog:remove', '#', 'admin', '2024-08-16 06:41:56', '', null, '');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2000, '用户地址', 3, 1, 'address', 'system/address/index', null, '', '1', '0', 'C', '0', '0', 'system:address:list', '#', 'admin', '2024-08-29 00:50:02', '', null, '用户地址菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2001, '用户地址查询', 2000, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:address:query', '#', 'admin', '2024-08-29 00:50:02', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2002, '用户地址新增', 2000, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:address:add', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2003, '用户地址修改', 2000, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:address:edit', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2004, '用户地址删除', 2000, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:address:remove', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2005, '用户地址导出', 2000, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:address:export', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2006, '衣物管理', 1, 1, 'clothing', 'system/clothing/index', null, '', '1', '0', 'C', '0', '0', 'system:clothing:list', 'theme', 'admin', '2024-08-29 00:50:03', 'admin', '2025-02-15T22:19:20.840106400+08:00', '衣物信息菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2007, '衣物信息查询', 2006, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:clothing:query', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2008, '衣物信息新增', 2006, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:clothing:add', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2009, '衣物信息修改', 2006, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:clothing:edit', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2010, '衣物信息删除', 2006, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:clothing:remove', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2011, '衣物信息导出', 2006, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:clothing:export', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2012, '预约订单', 3, 1, 'reservation', 'system/reservation/index', null, '', '1', '0', 'C', '0', '0', 'system:reservation:list', '#', 'admin', '2024-08-29 00:50:03', '', null, '预约订单菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2013, '预约订单查询', 2012, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:reservation:query', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2014, '预约订单新增', 2012, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:reservation:add', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2015, '预约订单修改', 2012, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:reservation:edit', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2016, '预约订单删除', 2012, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:reservation:remove', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2017, '预约订单导出', 2012, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:reservation:export', '#', 'admin', '2024-08-29 00:50:03', '', null, '');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2030, '标签管理', 1, 1, 'tags', 'system/tags/index', null, '', '1', '0', 'C', '0', '0', 'system:tags:list', 'checkbox', 'admin', '2024-09-02 06:32:39', 'admin', '2025-02-15T22:19:48.939528900+08:00', '用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2031, '搜索', 2030, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:tags:query', '#', 'admin', '2024-09-02 06:32:39', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2032, '新增', 2030, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:tags:add', '#', 'admin', '2024-09-02 06:32:39', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2033, '修改', 2030, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:tags:edit', '#', 'admin', '2024-09-02 06:32:39', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2034, '删除', 2030, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:tags:remove', '#', 'admin', '2024-09-02 06:32:39', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2035, '导出', 2030, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:tags:export', '#', 'admin', '2024-09-02 06:32:39', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2037, '卡券查询', 2036, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:coupon:query', '#', 'admin', '2024-09-03 06:44:02', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2038, '卡券新增', 2036, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:coupon:add', '#', 'admin', '2024-09-03 06:44:02', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2039, '卡券修改', 2036, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:coupon:edit', '#', 'admin', '2024-09-03 06:44:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2040, '卡券删除', 2036, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:coupon:remove', '#', 'admin', '2024-09-03 06:44:03', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2041, '卡券导出', 2036, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:coupon:export', '#', 'admin', '2024-09-03 06:44:03', '', null, '');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2042, '通知模板', 1, 1, 'template', 'system/template/index', null, '', '1', '0', 'C', '0', '0', 'system:template:list', 'email', 'admin', '2024-09-04 14:26:01', 'admin', '2025-02-15T22:19:57.149302500+08:00', '通知模板管理菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2043, '通知模板管理查询', 2042, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:template:query', '#', 'admin', '2024-09-04 14:26:01', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2044, '通知模板管理新增', 2042, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:template:add', '#', 'admin', '2024-09-04 14:26:01', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2045, '通知模板管理修改', 2042, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:template:edit', '#', 'admin', '2024-09-04 14:26:01', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2046, '通知模板管理删除', 2042, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:template:remove', '#', 'admin', '2024-09-04 14:26:01', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2047, '通知模板管理导出', 2042, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:template:export', '#', 'admin', '2024-09-04 14:26:01', '', null, '');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2048, '通知记录', 1, 1, 'notice_record', 'system/notice_record/index', null, '', '1', '0', 'C', '0', '0', 'system:noticeRecord:list', 'documentation', 'admin', '2024-09-05 02:42:29', 'admin', '2025-02-15T22:20:02.971941200+08:00', '通知记录管理菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2049, '通知记录管理查询', 2048, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:record:query', '#', 'admin', '2024-09-05 02:42:29', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2050, '通知记录管理新增', 2048, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:record:add', '#', 'admin', '2024-09-05 02:42:29', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2051, '通知记录管理修改', 2048, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:record:edit', '#', 'admin', '2024-09-05 02:42:29', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2052, '通知记录管理删除', 2048, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:record:remove', '#', 'admin', '2024-09-05 02:42:29', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2053, '通知记录管理导出', 2048, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:record:export', '#', 'admin', '2024-09-05 02:42:29', '', null, '');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2054, '价格管理', 1, 1, 'price', 'system/price/index', null, '', '1', '0', 'C', '0', '0', 'system:price:list', 'money', 'admin', '2024-09-06 13:39:17', 'admin', '2025-02-15T22:20:19.476233+08:00', '价格管理菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2055, '价格管理查询', 2054, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:price:query', '#', 'admin', '2024-09-06 13:39:17', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2056, '价格管理新增', 2054, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:price:add', '#', 'admin', '2024-09-06 13:39:17', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2057, '价格管理修改', 2054, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:price:edit', '#', 'admin', '2024-09-06 13:39:17', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2058, '价格管理删除', 2054, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:price:remove', '#', 'admin', '2024-09-06 13:39:17', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2059, '价格管理导出', 2054, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:price:export', '#', 'admin', '2024-09-06 13:39:17', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2060, '订单管理', 0, 4, 'orders', 'system/orders/index', null, '', '1', '1', 'C', '0', '0', '', 'clipboard', 'admin', '2024-09-07 01:41:40', 'admin', '2024-12-07T11:33:55.991381800+08:00', '洗护服务订单菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2061, '洗护服务订单查询', 2060, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:orders:query', '#', 'admin', '2024-09-07 01:41:40', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2062, '洗护服务订单新增', 2060, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:orders:add', '#', 'admin', '2024-09-07 01:41:40', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2063, '洗护服务订单修改', 2060, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:orders:edit', '#', 'admin', '2024-09-07 01:41:40', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2064, '洗护服务订单删除', 2060, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:orders:remove', '#', 'admin', '2024-09-07 01:41:40', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2065, '洗护服务订单导出', 2060, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:orders:export', '#', 'admin', '2024-09-07 01:41:40', '', null, '');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2066, '支出管理', 2078, 1, 'expenditure', 'system/expenditure/index', null, '', '1', '0', 'C', '0', '0', 'system:expenditure:list', 'edit', 'admin', '2024-09-11 01:26:21', 'admin', '2025-02-15T22:20:47.895946300+08:00', '支出菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2067, '支出查询', 2066, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:expenditure:query', '#', 'admin', '2024-09-11 01:26:21', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2068, '支出新增', 2066, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:expenditure:add', '#', 'admin', '2024-09-11 01:26:21', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2069, '支出修改', 2066, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:expenditure:edit', '#', 'admin', '2024-09-11 01:26:21', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2070, '支出删除', 2066, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:expenditure:remove', '#', 'admin', '2024-09-11 01:26:21', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2071, '支出导出', 2066, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:expenditure:export', '#', 'admin', '2024-09-11 01:26:21', '', null, '');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2072, '衣挂管理', 1, 1, 'rack', 'system/rack/index', null, '', '1', '0', 'C', '0', '0', 'system:rack:list', 'list', 'admin', '2024-09-14 06:13:30', 'admin', '2025-02-15T22:20:29.505985200+08:00', '晾衣架菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2073, '晾衣架查询', 2072, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:rack:query', '#', 'admin', '2024-09-14 06:13:30', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2074, '晾衣架新增', 2072, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:rack:add', '#', 'admin', '2024-09-14 06:13:30', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2075, '晾衣架修改', 2072, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:rack:edit', '#', 'admin', '2024-09-14 06:13:30', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2076, '晾衣架删除', 2072, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:rack:remove', '#', 'admin', '2024-09-14 06:13:30', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2077, '晾衣架导出', 2072, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:rack:export', '#', 'admin', '2024-09-14 06:13:30', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2078, '综合管理', 0, 3, 'integrated', null, null, '', '1', '0', 'M', '0', '0', '', 'log', 'admin', '2024-09-21 03:42:57', 'admin', '2024-12-07T11:34:35.608330500+08:00', '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2079, '统计报表', 0, 2, 'analysis', null, null, '', '1', '0', 'M', '0', '0', null, 'education', 'admin', '2024-09-21 03:47:00', '', '2024-12-07T11:34:06.599426100+08:00', '');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2080, '推广模板', 1, 1, 'promote-template', 'system/promote_template/index', null, '', '1', '0', 'C', '1', '1', 'system:promote-template:list', '#', 'admin', '2024-10-13 11:25:55', 'admin', '2024-12-07T11:33:04.524330900+08:00', '推广模板菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2081, '推广模板查询', 2080, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:template:query', '#', 'admin', '2024-10-13 11:25:55', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2082, '推广模板新增', 2080, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:template:add', '#', 'admin', '2024-10-13 11:25:55', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2083, '推广模板修改', 2080, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:template:edit', '#', 'admin', '2024-10-13 11:25:55', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2084, '推广模板删除', 2080, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:template:remove', '#', 'admin', '2024-10-13 11:25:55', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2085, '推广模板导出', 2080, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:template:export', '#', 'admin', '2024-10-13 11:25:55', '', null, '');
INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2086, '推广记录', 1, 1, 'promote-record', 'system/promote_record/index', null, '', '1', '0', 'C', '1', '1', 'system:promote-record:list', '#', 'admin', '2024-10-13 11:29:08', 'admin', '2024-12-07T11:33:09.946013+08:00', '推广记录菜单');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2087, '推广记录查询', 2086, 1, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:record:query', '#', 'admin', '2024-10-13 11:29:08', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2088, '推广记录新增', 2086, 2, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:record:add', '#', 'admin', '2024-10-13 11:29:08', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2089, '推广记录修改', 2086, 3, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:record:edit', '#', 'admin', '2024-10-13 11:29:08', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2090, '推广记录删除', 2086, 4, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:record:remove', '#', 'admin', '2024-10-13 11:29:08', '', null, '');
-- INSERT INTO menu (menu_id, menu_name, parent_id, order_num, path, component, query, route_name, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark) VALUES (2091, '推广记录导出', 2086, 5, '#', '', null, '', '1', '0', 'F', '0', '0', 'system:record:export', '#', 'admin', '2024-10-13 11:29:08', '', null, '');

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

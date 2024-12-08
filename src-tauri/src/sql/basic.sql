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
    open_id     TEXT NOT NULL DEFAULT '',
    dept_id     INTEGER       DEFAULT NULL,
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

CREATE TABLE IF NOT EXISTS post
(
    post_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    post_code   TEXT    NOT NULL,
    post_name   TEXT    NOT NULL,
    post_sort   INTEGER NOT NULL,
    status      TEXT    NOT NULL,
    create_time DATETIME DEFAULT NULL,
    update_time DATETIME DEFAULT NULL,
    remark      TEXT     DEFAULT NULL
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
    id          INTEGER AUTOINCREMENT PRIMARY KEY,
    user_id     INTEGER NOT NULL,
    coupon_id   INTEGER NOT NULL,
    identify    INTEGER NOT NULL,
    create_time TIMESTAMP
);

-- 创建索引，提高根据用户id查询效率
CREATE INDEX idx_user_id ON user_integral_record (user_id);

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
    coupon_id           INTEGER AUTOINCREMENT PRIMARY KEY,
    coupon_number       TEXT UNIQUE NOT NULL,
    coupon_type         TEXT        NOT NULL DEFAULT '000',
    coupon_title        TEXT        NOT NULL,
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
CREATE TABLE user_coupon
(
    uc_id           INTEGER AUTOINCREMENT PRIMARY KEY,
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
CREATE INDEX idx_user_id_user_coupon ON user_coupon (user_id);

CREATE INDEX idx_coupon_id ON user_coupon (coupon_id);

CREATE INDEX idx_status ON user_coupon (status);

CREATE INDEX idx_user_status ON user_coupon (user_id, status);

-- 卡券订单表
CREATE TABLE coupon_orders
(
    order_id    INTEGER AUTOINCREMENT PRIMARY KEY,
    uc_id       TEXT NOT NULL,
    create_time TIMESTAMP
);

-- 卡券订单索引
CREATE INDEX idx_uc_id ON coupon_orders (uc_id);

CREATE INDEX idx_create_time ON coupon_orders (create_time);

CREATE INDEX idx_create_uc ON coupon_orders (create_time, uc_id);

-- 支付记录表
CREATE INDEX idx_order_type ON payments (order_type);

-- 支付记录索引
CREATE TABLE payments
(
    pay_id             INTEGER AUTOINCREMENT PRIMARY KEY,
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

CREATE INDEX idx_payment_status ON payments (payment_status);

CREATE INDEX idx_pay_number ON payments (pay_number);

CREATE INDEX idx_create_time ON payments (create_time);

CREATE INDEX idx_order_status ON payments (order_type, payment_status);

CREATE INDEX idx_pay_number_order ON payments (pay_number, order_type);

-- 通知模板管理表
CREATE TABLE notice_template
(
    temp_id       INTEGER PRIMARY KEY AUTOINCREMENT,
    temp_name     TEXT NOT NULL,
    notice_method TEXT NOT NULL,
    content       TEXT NOT NULL,
    create_time   TIMESTAMP,
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
CREATE INDEX idx_user_id ON notice_record (user_id);

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

CREATE INDEX idx_order_type ON cloth_price (order_type);

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

CREATE INDEX idx_user_id ON orders (user_id);

CREATE INDEX idx_pickup_code ON orders (pickup_code);

CREATE INDEX idx_cost_time_alarm ON orders (cost_time_alarm);

CREATE INDEX idx_payment_status ON orders (payment_status);

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

INSERT INTO `menu` (`menu_id`, `menu_name`, `parent_id`, `order_num`, `path`, `component`, `query`, `route_name`,
                    `is_frame`, `is_cache`, `menu_type`, `visible`, `status`, `perms`, `icon`, `create_by`,
                    `create_time`, `update_by`, `update_time`, `remark`)
VALUES (1, '系统管理', 0, 1, 'system', NULL, '', '', 1, 0, 'M', '0', '0', '', 'system', 'admin', '2024-08-16 06:41:56',
        '', NULL, '系统管理目录'),
       (2, '系统监控', 0, 2, 'monitor', NULL, '', '', 1, 0, 'M', '0', '0', '', 'monitor', 'admin',
        '2024-08-16 06:41:56', '', NULL, '系统监控目录'),
       (3, '系统工具', 0, 3, 'tool', NULL, '', '', 1, 0, 'M', '0', '0', '', 'tool', 'admin', '2024-08-16 06:41:56', '',
        NULL, '系统工具目录'),
       (4, '若依官网', 0, 4, 'http://ruoyi.vip', NULL, '', '', 0, 0, 'M', '0', '0', '', 'guide', 'admin',
        '2024-08-16 06:41:56', '', NULL, '若依官网地址'),
       (100, '用户管理', 1, 1, 'user', 'system/user/index', '', '', 1, 0, 'C', '0', '0', 'system:user:list', 'user',
        'admin', '2024-08-16 06:41:56', '', NULL, '用户管理菜单'),
       (101, '角色管理', 1, 2, 'role', 'system/role/index', '', '', 1, 0, 'C', '0', '0', 'system:role:list', 'peoples',
        'admin', '2024-08-16 06:41:56', '', NULL, '角色管理菜单'),
       (102, '菜单管理', 1, 3, 'menu', 'system/menu/index', '', '', 1, 0, 'C', '0', '0', 'system:menu:list',
        'tree-table', 'admin', '2024-08-16 06:41:56', '', NULL, '菜单管理菜单'),
       (103, '组织管理', 1, 4, 'dept', 'system/dept/index', '', '', 1, 0, 'C', '0', '0', 'system:dept:list', 'tree',
        'admin', '2024-08-16 06:41:56', 'admin', '2024-08-22 17:51:15', '部门管理菜单'),
       (104, '等级管理', 1, 5, 'post', 'system/post/index', '', '', 1, 0, 'C', '0', '0', 'system:post:list', 'post',
        'admin', '2024-08-16 06:41:56', 'admin', '2024-08-22 15:31:54', '岗位管理菜单'),
       (105, '字典管理', 1, 6, 'dict', 'system/dict/index', '', '', 1, 0, 'C', '0', '0', 'system:dict:list', 'dict',
        'admin', '2024-08-16 06:41:56', '', NULL, '字典管理菜单'),
       (106, '参数设置', 1, 7, 'config', 'system/config/index', '', '', 1, 0, 'C', '0', '0', 'system:config:list',
        'edit', 'admin', '2024-08-16 06:41:56', '', NULL, '参数设置菜单'),
       (107, '通知公告', 1, 8, 'notice', 'system/notice/index', '', '', 1, 0, 'C', '0', '0', 'system:notice:list',
        'message', 'admin', '2024-08-16 06:41:56', '', NULL, '通知公告菜单'),
       (108, '日志管理', 1, 9, 'log', '', '', '', 1, 0, 'M', '0', '0', '', 'log', 'admin', '2024-08-16 06:41:56', '',
        NULL, '日志管理菜单'),
       (109, '在线用户', 2, 1, 'online', 'monitor/online/index', '', '', 1, 0, 'C', '0', '0', 'monitor:online:list',
        'online', 'admin', '2024-08-16 06:41:56', '', NULL, '在线用户菜单'),
       (110, '定时任务', 2, 2, 'job', 'monitor/job/index', '', '', 1, 0, 'C', '0', '0', 'monitor:job:list', 'job',
        'admin', '2024-08-16 06:41:56', '', NULL, '定时任务菜单'),
       (111, '数据监控', 2, 3, 'druid', 'monitor/druid/index', '', '', 1, 0, 'C', '0', '0', 'monitor:druid:list',
        'druid', 'admin', '2024-08-16 06:41:56', '', NULL, '数据监控菜单'),
       (112, '服务监控', 2, 4, 'server', 'monitor/server/index', '', '', 1, 0, 'C', '0', '0', 'monitor:server:list',
        'server', 'admin', '2024-08-16 06:41:56', '', NULL, '服务监控菜单'),
       (113, '缓存监控', 2, 5, 'cache', 'monitor/cache/index', '', '', 1, 0, 'C', '0', '0', 'monitor:cache:list',
        'redis', 'admin', '2024-08-16 06:41:56', '', NULL, '缓存监控菜单'),
       (114, '缓存列表', 2, 6, 'cacheList', 'monitor/cache/list', '', '', 1, 0, 'C', '0', '0', 'monitor:cache:list',
        'redis-list', 'admin', '2024-08-16 06:41:56', '', NULL, '缓存列表菜单'),
       (115, '表单构建', 3, 1, 'build', 'tool/build/index', '', '', 1, 0, 'C', '0', '0', 'tool:build:list', 'build',
        'admin', '2024-08-16 06:41:56', '', NULL, '表单构建菜单'),
       (116, '代码生成', 3, 2, 'gen', 'tool/gen/index', '', '', 1, 0, 'C', '0', '0', 'tool:gen:list', 'code', 'admin',
        '2024-08-16 06:41:56', '', NULL, '代码生成菜单'),
       (117, '系统接口', 3, 3, 'swagger', 'tool/swagger/index', '', '', 1, 0, 'C', '0', '0', 'tool:swagger:list',
        'swagger', 'admin', '2024-08-16 06:41:56', '', NULL, '系统接口菜单'),
       (500, '操作日志', 108, 1, 'operlog', 'monitor/operlog/index', '', '', 1, 0, 'C', '0', '0',
        'monitor:operlog:list', 'form', 'admin', '2024-08-16 06:41:56', '', NULL, '操作日志菜单'),
       (501, '登录日志', 108, 2, 'logininfor', 'monitor/logininfor/index', '', '', 1, 0, 'C', '0', '0',
        'monitor:logininfor:list', 'logininfor', 'admin', '2024-08-16 06:41:56', '', NULL, '登录日志菜单'),
       (1000, '用户查询', 100, 1, '', '', '', '', 1, 0, 'F', '0', '0', 'system:user:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1001, '用户新增', 100, 2, '', '', '', '', 1, 0, 'F', '0', '0', 'system:user:add', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1002, '用户修改', 100, 3, '', '', '', '', 1, 0, 'F', '0', '0', 'system:user:edit', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1003, '用户删除', 100, 4, '', '', '', '', 1, 0, 'F', '0', '0', 'system:user:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1004, '用户导出', 100, 5, '', '', '', '', 1, 0, 'F', '0', '0', 'system:user:export', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1005, '用户导入', 100, 6, '', '', '', '', 1, 0, 'F', '0', '0', 'system:user:import', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1006, '重置密码', 100, 7, '', '', '', '', 1, 0, 'F', '0', '0', 'system:user:resetPwd', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1007, '角色查询', 101, 1, '', '', '', '', 1, 0, 'F', '0', '0', 'system:role:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1008, '角色新增', 101, 2, '', '', '', '', 1, 0, 'F', '0', '0', 'system:role:add', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1009, '角色修改', 101, 3, '', '', '', '', 1, 0, 'F', '0', '0', 'system:role:edit', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1010, '角色删除', 101, 4, '', '', '', '', 1, 0, 'F', '0', '0', 'system:role:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1011, '角色导出', 101, 5, '', '', '', '', 1, 0, 'F', '0', '0', 'system:role:export', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1012, '菜单查询', 102, 1, '', '', '', '', 1, 0, 'F', '0', '0', 'system:menu:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1013, '菜单新增', 102, 2, '', '', '', '', 1, 0, 'F', '0', '0', 'system:menu:add', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1014, '菜单修改', 102, 3, '', '', '', '', 1, 0, 'F', '0', '0', 'system:menu:edit', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1015, '菜单删除', 102, 4, '', '', '', '', 1, 0, 'F', '0', '0', 'system:menu:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1016, '部门查询', 103, 1, '', '', '', '', 1, 0, 'F', '0', '0', 'system:dept:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1017, '部门新增', 103, 2, '', '', '', '', 1, 0, 'F', '0', '0', 'system:dept:add', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1018, '部门修改', 103, 3, '', '', '', '', 1, 0, 'F', '0', '0', 'system:dept:edit', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1019, '部门删除', 103, 4, '', '', '', '', 1, 0, 'F', '0', '0', 'system:dept:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1020, '岗位查询', 104, 1, '', '', '', '', 1, 0, 'F', '0', '0', 'system:post:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1021, '岗位新增', 104, 2, '', '', '', '', 1, 0, 'F', '0', '0', 'system:post:add', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1022, '岗位修改', 104, 3, '', '', '', '', 1, 0, 'F', '0', '0', 'system:post:edit', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1023, '岗位删除', 104, 4, '', '', '', '', 1, 0, 'F', '0', '0', 'system:post:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1024, '岗位导出', 104, 5, '', '', '', '', 1, 0, 'F', '0', '0', 'system:post:export', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1025, '字典查询', 105, 1, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:dict:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1026, '字典新增', 105, 2, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:dict:add', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1027, '字典修改', 105, 3, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:dict:edit', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1028, '字典删除', 105, 4, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:dict:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1029, '字典导出', 105, 5, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:dict:export', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1030, '参数查询', 106, 1, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:config:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1031, '参数新增', 106, 2, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:config:add', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1032, '参数修改', 106, 3, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:config:edit', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1033, '参数删除', 106, 4, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:config:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1034, '参数导出', 106, 5, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:config:export', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1035, '公告查询', 107, 1, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:notice:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1036, '公告新增', 107, 2, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:notice:add', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1037, '公告修改', 107, 3, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:notice:edit', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1038, '公告删除', 107, 4, '#', '', '', '', 1, 0, 'F', '0', '0', 'system:notice:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1039, '操作查询', 500, 1, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:operlog:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1040, '操作删除', 500, 2, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:operlog:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1041, '日志导出', 500, 3, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:operlog:export', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1042, '登录查询', 501, 1, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:logininfor:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1043, '登录删除', 501, 2, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:logininfor:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1044, '日志导出', 501, 3, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:logininfor:export', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1045, '账户解锁', 501, 4, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:logininfor:unlock', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1046, '在线查询', 109, 1, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:online:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1047, '批量强退', 109, 2, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:online:batchLogout', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1048, '单条强退', 109, 3, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:online:forceLogout', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1049, '任务查询', 110, 1, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:job:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1050, '任务新增', 110, 2, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:job:add', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1051, '任务修改', 110, 3, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:job:edit', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1052, '任务删除', 110, 4, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:job:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1053, '状态修改', 110, 5, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:job:changeStatus', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1054, '任务导出', 110, 6, '#', '', '', '', 1, 0, 'F', '0', '0', 'monitor:job:export', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1055, '生成查询', 116, 1, '#', '', '', '', 1, 0, 'F', '0', '0', 'tool:gen:query', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1056, '生成修改', 116, 2, '#', '', '', '', 1, 0, 'F', '0', '0', 'tool:gen:edit', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1057, '生成删除', 116, 3, '#', '', '', '', 1, 0, 'F', '0', '0', 'tool:gen:remove', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1058, '导入代码', 116, 4, '#', '', '', '', 1, 0, 'F', '0', '0', 'tool:gen:import', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1059, '预览代码', 116, 5, '#', '', '', '', 1, 0, 'F', '0', '0', 'tool:gen:preview', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, ''),
       (1060, '生成代码', 116, 6, '#', '', '', '', 1, 0, 'F', '0', '0', 'tool:gen:code', '#', 'admin',
        '2024-08-16 06:41:56', '', NULL, '');

INSERT INTO `dict_type` (`dict_id`, `dict_name`, `dict_type`, `status`, `create_time`,
                         `update_time`, `remark`)
VALUES (1, '用户性别', 'sys_user_sex', '0', '2024-08-16 06:41:56', '', '用户性别列表'),
       (2, '菜单状态', 'sys_show_hide', '0', '2024-08-16 06:41:56', '', '菜单状态列表'),
       (3, '系统开关', 'sys_normal_disable', '0', '2024-08-16 06:41:56', '', '系统开关列表'),
       (4, '任务状态', 'sys_job_status', '0', '2024-08-16 06:41:56', '', '任务状态列表'),
       (5, '任务分组', 'sys_job_group', '0', '2024-08-16 06:41:56', '', '任务分组列表'),
       (6, '系统是否', 'sys_yes_no', '0', '2024-08-16 06:41:56', '', '系统是否列表'),
       (7, '通知类型', 'sys_notice_type', '0', '2024-08-16 06:41:56', '', '通知类型列表'),
       (8, '通知状态', 'sys_notice_status', '0', '2024-08-16 06:41:56', '', '通知状态列表'),
       (9, '操作类型', 'sys_oper_type', '0', '2024-08-16 06:41:56', '', '操作类型列表'),
       (10, '系统状态', 'sys_common_status', '0', '2024-08-16 06:41:56', '', '登录状态列表'),
       (100, '用户类型', 'sys_user_type', '0', '2024-08-22 15:56:17', '',
        '用户类型，00 系统用户，01 会员客户');

INSERT INTO `dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type`, `css_class`,
                         `list_class`, `is_default`, `status`, `create_time`,
                         `update_time`, `remark`)
VALUES (1, 1, '男', '0', 'sys_user_sex', '', '', 'Y', '0', '2024-08-16 06:41:56', '', '性别男'),
       (2, 2, '女', '1', 'sys_user_sex', '', '', 'N', '0', '2024-08-16 06:41:56', '', '性别女'),
       (3, 3, '未知', '2', 'sys_user_sex', '', '', 'N', '0', '2024-08-16 06:41:56', '', '性别未知'),
       (4, 1, '显示', '0', 'sys_show_hide', '', 'primary', 'Y', '0', '2024-08-16 06:41:56', '',
        '显示菜单'),
       (5, 2, '隐藏', '1', 'sys_show_hide', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '',
        '隐藏菜单'),
       (6, 1, '正常', '0', 'sys_normal_disable', '', 'primary', 'Y', '0', '2024-08-16 06:41:56', '',
        '正常状态'),
       (7, 2, '停用', '1', 'sys_normal_disable', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '',
        '停用状态'),
       (8, 1, '正常', '0', 'sys_job_status', '', 'primary', 'Y', '0', '2024-08-16 06:41:56', '',
        '正常状态'),
       (9, 2, '暂停', '1', 'sys_job_status', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '',
        '停用状态'),
       (10, 1, '默认', 'DEFAULT', 'sys_job_group', '', '', 'Y', '0', '2024-08-16 06:41:56', '',
        '默认分组'),
       (11, 2, '系统', 'SYSTEM', 'sys_job_group', '', '', 'N', '0', '2024-08-16 06:41:56', '',
        '系统分组'),
       (12, 1, '是', 'Y', 'sys_yes_no', '', 'primary', 'Y', '0', '2024-08-16 06:41:56', '',
        '系统默认是'),
       (13, 2, '否', 'N', 'sys_yes_no', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '', '系统默认否'),
       (14, 1, '通知', '1', 'sys_notice_type', '', 'warning', 'Y', '0', '2024-08-16 06:41:56', '',
        '通知'),
       (15, 2, '公告', '2', 'sys_notice_type', '', 'success', 'N', '0', '2024-08-16 06:41:56', '',
        '公告'),
       (16, 1, '正常', '0', 'sys_notice_status', '', 'primary', 'Y', '0', '2024-08-16 06:41:56', '',
        '正常状态'),
       (17, 2, '关闭', '1', 'sys_notice_status', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '',
        '关闭状态'),
       (18, 99, '其他', '0', 'sys_oper_type', '', 'info', 'N', '0', '2024-08-16 06:41:56', '',
        '其他操作'),
       (19, 1, '新增', '1', 'sys_oper_type', '', 'info', 'N', '0', '2024-08-16 06:41:56', '',
        '新增操作'),
       (20, 2, '修改', '2', 'sys_oper_type', '', 'info', 'N', '0', '2024-08-16 06:41:56', '',
        '修改操作'),
       (21, 3, '删除', '3', 'sys_oper_type', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '',
        '删除操作'),
       (22, 4, '授权', '4', 'sys_oper_type', '', 'primary', 'N', '0', '2024-08-16 06:41:56', '',
        '授权操作'),
       (23, 5, '导出', '5', 'sys_oper_type', '', 'warning', 'N', '0', '2024-08-16 06:41:56', '',
        '导出操作'),
       (24, 6, '导入', '6', 'sys_oper_type', '', 'warning', 'N', '0', '2024-08-16 06:41:56', '',
        '导入操作'),
       (25, 7, '强退', '7', 'sys_oper_type', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '',
        '强退操作'),
       (26, 8, '生成代码', '8', 'sys_oper_type', '', 'warning', 'N', '0', '2024-08-16 06:41:56', '',
        '生成操作'),
       (27, 9, '清空数据', '9', 'sys_oper_type', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '',
        '清空操作'),
       (28, 1, '成功', '0', 'sys_common_status', '', 'primary', 'N', '0', '2024-08-16 06:41:56', '',
        '正常状态'),
       (29, 2, '失败', '1', 'sys_common_status', '', 'danger', 'N', '0', '2024-08-16 06:41:56', '',
        '停用状态'),
       (100, 1, '系统用户', '00', 'sys_user_type', NULL, 'primary', 'N', '0', '2024-08-22 15:57:50', '',
        NULL),
       (101, 2, '会员客户', '01', 'sys_user_type', NULL, 'success', 'N', '0', '2024-08-22 15:58:09', '',
        NULL);

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

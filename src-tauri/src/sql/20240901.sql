CREATE TABLE IF NOT EXISTS sys_dict_type
(
    dict_id     INTEGER NOT NULL AUTOINCREMENT PRIMARY KEY,
    dict_name   varchar(100) DEFAULT '',
    dict_type   varchar(100) DEFAULT '',
    status      char(1)      DEFAULT '0',
    create_by   varchar(64)  DEFAULT '',
    create_time datetime     DEFAULT NULL,
    update_by   varchar(64)  DEFAULT '',
    update_time datetime     DEFAULT NULL,
    remark      varchar(500) DEFAULT NULL
);
;
CREATE TABLE IF NOT EXISTS sys_dict_data
(
    dict_code   INTEGER NOT NULL AUTOINCREMENT PRIMARY KEY,
    dict_sort   INTEGER      DEFAULT 0,
    dict_label  varchar(100) DEFAULT '',
    dict_value  varchar(100) DEFAULT '',
    dict_type   varchar(100) DEFAULT '',
    css_class   varchar(100) DEFAULT NULL,
    list_class  varchar(100) DEFAULT NULL,
    is_default  char(1)      DEFAULT 'N',
    status      char(1)      DEFAULT '0',
    create_by   varchar(64)  DEFAULT '',
    create_time datetime     DEFAULT NULL,
    update_by   varchar(64)  DEFAULT '',
    update_time datetime     DEFAULT NULL,
    remark      varchar(500) DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS sys_user
(
    user_id     INTEGER      NOT NULL AUTOINCREMENT PRIMARY KEY,
    open_id     varchar(50)  NOT NULL DEFAULT '',
    dept_id     INTEGER               DEFAULT NULL,
    user_name   varchar(30)  NOT NULL,
    nick_name   varchar(30)  NOT NULL,
    address     varchar(128) NOT NULL,
    user_type   varchar(2)            DEFAULT '00',
    email       varchar(50)           DEFAULT '',
    phonenumber varchar(11)           DEFAULT '',
    sex         char(1)               DEFAULT '0',
    avatar      varchar(100)          DEFAULT '',
    password    varchar(100)          DEFAULT '',
    status      char(1)               DEFAULT '0',
    del_flag    char(1)               DEFAULT '0',
    integral    INTEGER               DEFAULT 0,
    identify    varchar(2)            DEFAULT '00',
    login_ip    varchar(128)          DEFAULT '',
    login_date  datetime              DEFAULT NULL,
    create_by   varchar(64)           DEFAULT '',
    create_time datetime              DEFAULT NULL,
    update_by   varchar(64)           DEFAULT '',
    update_time datetime              DEFAULT NULL,
    remark      varchar(500)          DEFAULT NULL
);

-- 会员画像表
CREATE TABLE sys_user_tags
(
    user_id  INTEGER      NOT NULL,
    user_tag VARCHAR(128) NOT NULL,
    remark   VARCHAR(256)
);

-- 积分使用记录表
CREATE TABLE sys_user_integral_record
(
    id          INTEGER AUTOINCREMENT PRIMARY KEY,
    user_id     INTEGER  NOT NULL,
    coupon_id   INTEGER  NOT NULL,
    identify    INTEGER  NOT NULL,
    create_time DATETIME NOT NULL
);

-- 创建索引，提高根据用户id查询效率
CREATE INDEX idx_user_id ON sys_user_integral_record (user_id);

-- 标签管理表
CREATE TABLE sys_tags
(
    tag_id     INTEGER AUTOINCREMENT PRIMARY KEY,
    tag_number VARCHAR(50) UNIQUE NOT NULL,
    tag_order  VARCHAR(3),
    tag_name   VARCHAR(50) UNIQUE NOT NULL,
    ref_num    INTEGER DEFAULT 0,
    order_num  INTEGER DEFAULT 0,
    status     CHAR(1) DEFAULT '0',
    del_flag   CHAR(1) DEFAULT '0',
    remark     VARCHAR(500)
);

-- 衣物管理表
CREATE TABLE clothing
(
    clothing_id         INTEGER PRIMARY KEY AUTOINCREMENT,
    clothing_category   VARCHAR(3)  NOT NULL,
    clothing_number     VARCHAR(30) NOT NULL,
    clothing_style      VARCHAR(3)  NOT NULL,
    clothing_name       VARCHAR(50) NOT NULL,
    clothing_base_price DOUBLE      NOT NULL,
    clothing_min_price  DOUBLE      NOT NULL,
    order_num           INTEGER              DEFAULT 0,
    clothing_degree     INTEGER              DEFAULT 0,
    hang_type           CHAR(1)     NOT NULL DEFAULT '1',
    del_flag            CHAR(1)              DEFAULT '0',
    remark              VARCHAR(500)
);

-- 创建索引，提高根据衣物类别和衣物名称查询效率
CREATE INDEX idx_clothing_category ON clothing (clothing_category);

CREATE INDEX idx_clothing_name ON clothing (clothing_name);

-- 卡券管理表
CREATE TABLE sys_coupon
(
    coupon_id           INTEGER AUTOINCREMENT PRIMARY KEY,
    coupon_number       VARCHAR(30) UNIQUE NOT NULL,
    coupon_type         VARCHAR(3)         NOT NULL DEFAULT '000',
    coupon_title        VARCHAR(50)        NOT NULL,
    coupon_value        DOUBLE             NOT NULL,
    min_spend           DOUBLE                      DEFAULT 0,
    customer_invalid    CHAR(1)                     DEFAULT '0',
    customer_sale_total INTEGER                     DEFAULT 0,
    customer_sale_count INTEGER                     DEFAULT 0,
    valid_from          DATETIME           NOT NULL,
    valid_to            DATETIME           NOT NULL,
    auto_delay          CHAR(1)                     DEFAULT '0',
    usage_value         DOUBLE                      DEFAULT 0,
    usage_limit         DOUBLE                      DEFAULT 0,
    del_flag            CHAR(1)                     DEFAULT '0',
    applicable_category VARCHAR(500)                DEFAULT NULL,
    applicable_style    VARCHAR(500)                DEFAULT NULL,
    applicable_cloths   VARCHAR(500)                DEFAULT NULL,
    status              CHAR(1)                     DEFAULT '0',
    remark              VARCHAR(500)                DEFAULT NULL
);

-- 卡券表索引
CREATE INDEX idx_coupon_name ON sys_coupon (coupon_title);

CREATE INDEX idx_coupon_type ON sys_coupon (coupon_type);

CREATE INDEX idx_coupon_status ON sys_coupon (status);

CREATE INDEX idx_coupon_del_flag ON sys_coupon (del_flag);

-- 用户卡券关联表
CREATE TABLE sys_user_coupon
(
    uc_id           INTEGER AUTOINCREMENT PRIMARY KEY,
    user_id         INTEGER  NOT NULL,
    coupon_id       INTEGER  NOT NULL,
    create_time     DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    obtain_at       DATETIME          DEFAULT NULL,
    available_value DOUBLE            DEFAULT 0,
    uc_count        INTEGER           DEFAULT 1,
    pay_id          INTEGER           DEFAULT NULL,
    uc_type         VARCHAR(2)        DEFAULT '01',
    status          VARCHAR(2)        DEFAULT '00',
    remark          VARCHAR(256)      DEFAULT NULL
);

-- 用户卡券索引
CREATE INDEX idx_user_id_user_coupon ON sys_user_coupon (user_id);

CREATE INDEX idx_coupon_id ON sys_user_coupon (coupon_id);

CREATE INDEX idx_status ON sys_user_coupon (status);

CREATE INDEX idx_user_status ON sys_user_coupon (user_id, status);

-- 卡券订单表
CREATE TABLE sys_coupon_order
(
    order_id    INTEGER AUTOINCREMENT PRIMARY KEY,
    uc_id       VARCHAR(64) NOT NULL,
    create_time DATETIME    NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 卡券订单索引
CREATE INDEX idx_uc_id ON sys_coupon_order (uc_id);

CREATE INDEX idx_create_time ON sys_coupon_order (create_time);

CREATE INDEX idx_create_uc ON sys_coupon_order (create_time, uc_id);

-- 支付记录表
CREATE INDEX idx_order_type ON sys_payment (order_type);

-- 支付记录索引
CREATE TABLE sys_payment
(
    pay_id             INTEGER AUTOINCREMENT PRIMARY KEY,
    pay_number         VARCHAR(30) NOT NULL,
    order_type         CHAR(1)     NOT NULL,
    total_amount       DOUBLE      NOT NULL,
    payment_amount     DOUBLE      NOT NULL,
    payment_amount_vip DOUBLE               DEFAULT 0,
    payment_amount_mv  DOUBLE               DEFAULT 0,
    payment_status     VARCHAR(2)  NOT NULL,
    payment_method     VARCHAR(2)  NOT NULL,
    transaction_id     INTEGER,
    uc_order_id        INTEGER,
    uc_id              VARCHAR(256),
    create_time        DATETIME    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_time        DATETIME             DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    order_status       VARCHAR(2)  NOT NULL
);

CREATE INDEX idx_payment_status ON sys_payment (payment_status);

CREATE INDEX idx_pay_number ON sys_payment (pay_number);

CREATE INDEX idx_create_time ON sys_payment (create_time);

CREATE INDEX idx_order_status ON sys_payment (order_type, payment_status);

CREATE INDEX idx_pay_number_order ON sys_payment (pay_number, order_type);

-- 通知模板管理表
CREATE TABLE sys_notice_template
(
    temp_id       INTEGER AUTOINCREMENT PRIMARY KEY,
    temp_name     VARCHAR(50)  NOT NULL,
    notice_method char(1)      NOT NULL,
    content       VARCHAR(500) NOT NULL,
    create_time   DATETIME DEFAULT CURRENT_TIMESTAMP,
    temp_type     char(1)      NOT NULL,
    remark        VARCHAR(500)
);

-- 通知记录管理表
-- todo 订单id需要后续进行关联
CREATE TABLE sys_notice_record
(
    notice_id     INTEGER AUTOINCREMENT PRIMARY KEY,
    user_id       INTEGER      NOT NULL,
    order_number  varchar(64)  NULL,
    notice_method char(1)      NOT NULL,
    notice_type   char(1)      NOT NULL,
    notice_time   DATETIME DEFAULT NOW(),
    title         VARCHAR(50)  NOT NULL,
    content       VARCHAR(500) NOT NULL,
    result        char(1),
    remark        VARCHAR(500)
);

-- 创建索引
CREATE INDEX idx_user_id ON sys_notice_record (user_id);

-- 衣物价格表
CREATE TABLE sys_cloth_price
(
    price_id          INTEGER PRIMARY KEY AUTOINCREMENT,
    price_number      VARCHAR(30) NOT NULL,
    order_type        VARCHAR(2)  NOT NULL,
    price_name        VARCHAR(50) NOT NULL,
    price_value       DOUBLE,
    price_discount    DOUBLE,
    applicable_cloths VARCHAR(500),
    order_num         INTEGER     NOT NULL,
    clothing_degree   INTEGER   DEFAULT 0,
    status            CHAR(1)   DEFAULT '0',
    del_flag          CHAR(1)   DEFAULT '0',
    remark            VARCHAR(500),
    created_at        TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at        TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE INDEX idx_order_type ON sys_cloth_price (order_type);

CREATE INDEX idx_applicable_cloths ON sys_cloth_price (applicable_cloths);

-- 订单表
CREATE TABLE sys_orders
(
    order_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    order_number         VARCHAR(30) NOT NULL,
    business_type        VARCHAR(2)  NOT NULL,
    user_id              INTEGER     NOT NULL,
    price_id             INTEGER,
    desire_complete_time DATETIME    NOT NULL,
    cost_time_alarm      VARCHAR(2),
    pickup_code          VARCHAR(10),
    complete_time        DATETIME,
    delivery_mode        VARCHAR(2),
    source               VARCHAR(2)  NOT NULL,
    status               VARCHAR(2)  NOT NULL,
    payment_status       VARCHAR(2)  NOT NULL,
    remark               VARCHAR(500),
    order_type           VARCHAR(2)  NOT NULL,
    create_time          TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time          TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_order_number ON sys_orders (order_number);

CREATE INDEX idx_user_id ON sys_orders (user_id);

CREATE INDEX idx_pickup_code ON sys_orders (pickup_code);

CREATE INDEX idx_cost_time_alarm ON sys_orders (cost_time_alarm);

CREATE INDEX idx_payment_status ON sys_orders (payment_status);

-- 衣物清单表
CREATE TABLE sys_orders_cloths
(
    cloth_id            INTEGER PRIMARY KEY AUTOINCREMENT,
    order_cloth_id      INTEGER    NULL,
    clothing_id         INTEGER    NOT NULL,
    clothing_category   VARCHAR(3) NOT NULL,
    clothing_style      VARCHAR(3) NOT NULL,
    clothing_color      INTEGER,
    clothing_flaw       VARCHAR(256),
    estimate            VARCHAR(256),
    clothing_brand      INTEGER,
    service_type        VARCHAR(3) NOT NULL,
    service_requirement VARCHAR(3),
    before_pics         VARCHAR(1024),
    after_pics          VARCHAR(1024),
    notes               VARCHAR(1024),
    process_markup      FLOAT,
    price_value         FLOAT      NOT NULL,
    hang_type           CHAR(1)    NOT NULL DEFAULT '1',
    hang_location_code  INTEGER,
    hanger_number       INTEGER,
    hang_cloth_code     VARCHAR(32) UNIQUE,
    hang_remark         VARCHAR(256),
    create_time         DATETIME   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    pickup_time         DATETIME,
    pickup_method       varchar(2),
    clothing_status     varchar(2),
    remark              VARCHAR(256)
);

-- 创建索引
CREATE INDEX idx_order_clothing_id ON sys_orders_cloths (order_cloth_id);

-- 订单衣物调价记录表 ，记录订单中每个衣物或订单总价的价格调整
CREATE TABLE sys_order_cloths_adjust
(
    adjust_id        INTEGER NOT NULL AUTOINCREMENT,
    order_id         INTEGER UNIQUE,
    adjust_value_add FLOAT,
    adjust_value_sub FLOAT,
    adjust_total     FLOAT,
    remark           VARCHAR(256)
);

-- 支付记录表
CREATE TABLE sys_orders_pays
(
    pay_id      INTEGER    NOT NULL AUTOINCREMENT,
    order_id    INTEGER    NOT NULL,
    create_by   VARCHAR(64),
    create_time DATETIME   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_by   VARCHAR(64),
    update_time DATETIME            DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    remark      VARCHAR(500),
    pay_amount  FLOAT      NOT NULL,
    pay_method  VARCHAR(2) NOT NULL
);

-- 订单涉及的图片索引表，记录订单中相关的图片信息
CREATE TABLE sys_orders_pictures
(
    picture_id   INTEGER  PRIMARY KEY AUTOINCREMENT,
    picture_path VARCHAR(256) NOT NULL,
    create_by    VARCHAR(64),
    create_time  DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 订单派送信息
CREATE TABLE sys_orders_dispatch
(
    dispatch_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id      VARCHAR(128) NOT NULL,
    cloth_id      VARCHAR(512) NOT NULL,
    delivery_comp VARCHAR(64),
    delivery_num  VARCHAR(64),
    dispatch_time DATETIME,
    remark        VARCHAR(256),
    create_time   DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 支出记录表
CREATE TABLE sys_expenditure
(
    exp_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id           VARCHAR(64),
    cloth_ids          VARCHAR(512),
    exp_title          VARCHAR(128) NOT NULL,
    recv_account       INTEGER,
    recv_account_title VARCHAR(64),
    exp_type           VARCHAR(10)  NOT NULL,
    exp_amount         INTEGER      NOT NULL,
    create_time        DATETIME DEFAULT CURRENT_TIMESTAMP,
    remark             VARCHAR(100)
);

-- 晾衣架表
CREATE TABLE drying_rack
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    name               VARCHAR(50) NOT NULL,
    rack_type          char(1) DEFAULT '1',
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
CREATE TABLE sys_orders_repair
(
    repair_id    INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id     INTEGER,
    old_order_id INTEGER,
    create_time  DATETIME DEFAULT CURRENT_TIMESTAMP,
    remark       VARCHAR(256)
);

-- 推广模板
CREATE TABLE sys_promote_template
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    content         VARCHAR(64) NOT NULL,
    promote_type    VARCHAR(2)  NOT NULL,
    promote_method  VARCHAR(2)  NOT NULL,
    promote_count   INTEGER,
    promote_objects VARCHAR(1024),
    create_time     DATETIME DEFAULT CURRENT_TIMESTAMP,
    promote_picture VARCHAR(256),
    is_pin          char(1)  DEFAULT '0',
    remark          VARCHAR(256)
);

-- 推广记录表
-- 因为存在仅本次有效的情况，所以还是需要一个推广对象的字段的
CREATE TABLE sys_promote_record
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    temp_id         INTEGER,
    promote_objects VARCHAR(1024) NOT NULL,
    promote_time    DATETIME DEFAULT CURRENT_TIMESTAMP,
    status          CHAR(1)  DEFAULT '0'
);

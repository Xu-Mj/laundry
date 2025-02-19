// const USER_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS users
// (
//     user_id     INTEGER PRIMARY KEY AUTOINCREMENT,
//     open_id     TEXT NOT NULL DEFAULT '',
//     dept_id     INTEGER       DEFAULT NULL,
//     user_name   TEXT NOT NULL,
//     nick_name   TEXT NOT NULL,
//     address     TEXT,
//     user_type   TEXT          DEFAULT '00',
//     email       TEXT          DEFAULT '',
//     phonenumber TEXT          DEFAULT '',
//     sex         TEXT          DEFAULT '0',
//     avatar      TEXT          DEFAULT '',
//     password    TEXT          DEFAULT '',
//     status      TEXT          DEFAULT '0',
//     del_flag    TEXT          DEFAULT '0',
//     integral    INTEGER       DEFAULT 0,
//     identify    TEXT          DEFAULT '00',
//     login_ip    TEXT          DEFAULT '',
//     login_date  TIMESTAMP     DEFAULT NULL,
//     create_by   TEXT          DEFAULT '',
//     create_time TIMESTAMP,
//     update_by   TEXT          DEFAULT '',
//     update_time TIMESTAMP,
//     remark      TEXT          DEFAULT NULL
// );";

// const TAG_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS tags
// (
//     tag_id     INTEGER PRIMARY KEY AUTOINCREMENT,
//     tag_number TEXT UNIQUE NOT NULL,
//     tag_order  TEXT,
//     tag_name   TEXT UNIQUE NOT NULL,
//     ref_num    INTEGER DEFAULT 0,
//     order_num  INTEGER DEFAULT 0,
//     status     TEXT    DEFAULT '0',
//     del_flag   TEXT    DEFAULT '0',
//     remark     TEXT
// );";

// const PRINTERS_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS printers (
//                                         id INTEGER PRIMARY KEY AUTOINCREMENT,
//                                         name TEXT NOT NULL,
//                                         system_name TEXT NOT NULL,
//                                         driver_name TEXT NOT NULL
//                                     )";

// const CLOTHING_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS clothing
// (
//     clothing_id         INTEGER PRIMARY KEY AUTOINCREMENT,
//     clothing_category   TEXT   NOT NULL,
//     clothing_number     TEXT   NOT NULL,
//     clothing_style      TEXT   NOT NULL,
//     clothing_name       TEXT   NOT NULL,
//     clothing_base_price DOUBLE NOT NULL,
//     clothing_min_price  DOUBLE NOT NULL,
//     order_num           INTEGER         DEFAULT 0,
//     clothing_degree     INTEGER         DEFAULT 0,
//     hang_type           TEXT   NOT NULL DEFAULT '1',
//     del_flag            TEXT            DEFAULT '0',
//     remark              TEXT
// );";

// const DRYING_RACK_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS drying_rack
//                                     (
//                                         id                 INTEGER PRIMARY KEY AUTOINCREMENT,
//                                         name               TEXT NOT NULL,
//                                         rack_type          TEXT DEFAULT '1',
//                                         capacity           INTEGER NOT NULL,
//                                         remaining_capacity INTEGER NOT NULL,
//                                         position           INTEGER NOT NULL DEFAULT 0
//                                     )";

// /// 用来存储衣物的编码最大值
// const CLOTH_SEQUENCE_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS cloth_sequence
//                                         (
//                                             id              INTEGER PRIMARY KEY AUTOINCREMENT,
//                                             date            DATE    NOT NULL,
//                                             sequence_number INTEGER NOT NULL
//                                         )";

// const CLOTH_PRICE_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS cloth_price
//                                     (
//                                         price_id       INTEGER PRIMARY KEY AUTOINCREMENT,
//                                         price_number   TEXT     NOT NULL,
//                                         order_type     TEXT     NOT NULL,
//                                         price_name     TEXT     NOT NULL,
//                                         price_value    DOUBLE,
//                                         price_discount DOUBLE,
//                                         order_num      INTEGER   NOT NULL,
//                                         ref_num        INTEGER   DEFAULT 0,
//                                         status         TEXT   DEFAULT '0',
//                                         del_flag       TEXT   DEFAULT '0',
//                                         remark         TEXT,
//                                         create_time    TIMESTAMP,
//                                         update_time    TIMESTAMP
//                                     )";

// const ORDER_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS  orders
//                                 (
//                                     order_id             INTEGER PRIMARY KEY AUTOINCREMENT,
//                                     order_number         TEXT NOT NULL,
//                                     business_type        TEXT  NOT NULL,
//                                     user_id              INTEGER     NOT NULL,
//                                     price_id             INTEGER,
//                                     desire_complete_time TIMESTAMP   NOT NULL,
//                                     cost_time_alarm      TEXT,
//                                     pickup_code          TEXT,
//                                     complete_time        TIMESTAMP,
//                                     delivery_mode        TEXT,
//                                     source               TEXT  NOT NULL,
//                                     status               TEXT  NOT NULL,
//                                     payment_status       TEXT  NOT NULL,
//                                     remark               TEXT,
//                                     order_type           TEXT  NOT NULL,
//                                     create_time          TIMESTAMP,
//                                     update_time          TIMESTAMP
//                                 );";

// const ORDER_ADJUST_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS order_clothes_adjust
//                                     (
//                                         adjust_id        INTEGER PRIMARY KEY AUTOINCREMENT,
//                                         order_id         INTEGER UNIQUE,
//                                         adjust_value_add FLOAT,
//                                         adjust_value_sub FLOAT,
//                                         adjust_total     FLOAT,
//                                         remark           TEXT
//                                     );";

// const ORDER_CLOTHES_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS order_clothes
//                                         (
//                                             cloth_id            INTEGER PRIMARY KEY AUTOINCREMENT,
//                                             order_id            INTEGER    NULL,
//                                             clothing_id         INTEGER    NOT NULL,
//                                             clothing_category   TEXT NOT NULL,
//                                             clothing_style      TEXT NOT NULL,
//                                             clothing_color      INTEGER,
//                                             clothing_flaw       TEXT,
//                                             estimate            TEXT,
//                                             clothing_brand      INTEGER,
//                                             service_type        TEXT NOT NULL,
//                                             service_requirement TEXT,
//                                             before_pics         TEXT,
//                                             after_pics          TEXT,
//                                             notes               TEXT,
//                                             process_markup      FLOAT,
//                                             price_value         FLOAT      NOT NULL,
//                                             hang_type           TEXT    NOT NULL DEFAULT '1',
//                                             hang_location_code  INTEGER,
//                                             hanger_number       INTEGER,
//                                             hang_cloth_code     TEXT UNIQUE,
//                                             hang_remark         TEXT,
//                                             create_time         TIMESTAMP,
//                                             pickup_time         TIMESTAMP,
//                                             pickup_method       TEXT,
//                                             clothing_status     TEXT,
//                                             remark              TEXT
//                                         );";

// const ORDER_PICTURES_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS order_pictures
//                                         (
//                                             picture_id   INTEGER PRIMARY KEY AUTOINCREMENT,
//                                             picture_path TEXT NOT NULL,
//                                             create_by    TEXT,
//                                             create_time  TIMESTAMP
//                                         );";

// const PAYMENT_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS payments
//                                 (
//                                     pay_id             INTEGER PRIMARY KEY AUTOINCREMENT,
//                                     pay_number         TEXT NOT NULL,
//                                     order_type         TEXT     NOT NULL,
//                                     total_amount       DOUBLE      NOT NULL,
//                                     payment_amount     DOUBLE      NOT NULL,
//                                     payment_amount_vip DOUBLE DEFAULT 0,
//                                     payment_amount_mv  DOUBLE DEFAULT 0,
//                                     payment_status     TEXT  NOT NULL,
//                                     payment_method     TEXT  NOT NULL,
//                                     transaction_id     INTEGER,
//                                     uc_order_id        INTEGER,
//                                     uc_id              TEXT,
//                                     create_time        TIMESTAMP,
//                                     update_time        TIMESTAMP,
//                                     order_status       TEXT  NOT NULL
//                                 );";

// const COUPON_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS coupons
//                                 (
//                                     coupon_id           INTEGER PRIMARY KEY AUTOINCREMENT,
//                                     coupon_number       TEXT UNIQUE NOT NULL,
//                                     coupon_type         TEXT         NOT NULL DEFAULT '000',
//                                     coupon_title        TEXT        NOT NULL,
//                                     desc                TEXT        NOT NULL,
//                                     coupon_value        DOUBLE             NOT NULL,
//                                     min_spend           DOUBLE                      DEFAULT 0,
//                                     customer_invalid    TEXT                     DEFAULT '0',
//                                     customer_sale_total INTEGER                     DEFAULT 0,
//                                     customer_sale_count INTEGER                     DEFAULT 0,
//                                     valid_from          TIMESTAMP          NOT NULL,
//                                     valid_to            TIMESTAMP          NOT NULL,
//                                     auto_delay          TEXT                     DEFAULT '0',
//                                     usage_value         DOUBLE                      DEFAULT 0,
//                                     usage_limit         DOUBLE                      DEFAULT 0,
//                                     del_flag            TEXT                     DEFAULT '0',
//                                     applicable_category TEXT                DEFAULT NULL,
//                                     applicable_style    TEXT                DEFAULT NULL,
//                                     applicable_cloths   TEXT                DEFAULT NULL,
//                                     status              TEXT                     DEFAULT '0',
//                                     remark              TEXT                DEFAULT NULL
//                                 );";

// const USER_COUPON_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS user_coupons
//                                     (
//                                         uc_id           INTEGER PRIMARY KEY AUTOINCREMENT,
//                                         user_id         INTEGER NOT NULL,
//                                         coupon_id       INTEGER NOT NULL,
//                                         create_time     TIMESTAMP,
//                                         obtain_at       TIMESTAMP    DEFAULT NULL,
//                                         available_value DOUBLE       DEFAULT 0,
//                                         uc_count        INTEGER      DEFAULT 1,
//                                         pay_id          INTEGER      DEFAULT NULL,
//                                         uc_type         TEXT   DEFAULT '01',
//                                         status          TEXT   DEFAULT '00',
//                                         remark          TEXT DEFAULT NULL
//                                     );";

// const COUPON_ORDER_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS coupon_orders
//                                     (
//                                         order_id    INTEGER PRIMARY KEY AUTOINCREMENT,
//                                         uc_ids       TEXT NOT NULL,
//                                         create_time TIMESTAMP
//                                     );";

// const CONFIG_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS configs
// (
//     config_id    INTEGER PRIMARY KEY AUTOINCREMENT,                           -- 参数主键
//     config_name  TEXT                                   DEFAULT ''  NOT NULL, -- 参数名称
//     config_key   TEXT                                   DEFAULT ''  NOT NULL, -- 参数键名
//     config_value TEXT                                   DEFAULT ''  NOT NULL, -- 参数键值
//     config_type  TEXT CHECK (config_type IN ('Y', 'N')) DEFAULT 'N' NOT NULL, -- 系统内置（Y是 N否）
//     create_by    TEXT                                   DEFAULT '',           -- 创建者
//     create_time  TIMESTAMP                              DEFAULT NULL,         -- 创建时间 (用 ISO8601 格式存储)
//     update_by    TEXT                                   DEFAULT '',           -- 更新者
//     update_time  TIMESTAMP                              DEFAULT NULL,         -- 更新时间 (用 ISO8601 格式存储)
//     remark       TEXT                                   DEFAULT NULL          -- 备注
// );";

// const USER_TAGS_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS user_tags
//                                     (
//                                         user_id  INTEGER UNIQUE NOT NULL,
//                                         tags     TEXT    NOT NULL,
//                                         remark   TEXT
//                                     );";

// const LOCAL_USER_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS local_users
//                                     (
//                                         id  INTEGER PRIMARY KEY AUTOINCREMENT,
//                                         username    TEXT    NOT NULL,
//                                         avatar      TEXT    NOT NULL,
//                                         account     TEXT    NOT NULL,
//                                         password    TEXT    NOT NULL,
//                                         role        TEXT    NOT NULL,
//                                         remark   TEXT,
//                                         is_first_login   INTEGER
//                                     );";

// const DICT_TYPE_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS dict_type
//                                     (
//                                         dict_id     INTEGER PRIMARY KEY AUTOINCREMENT,
//                                         dict_name   TEXT DEFAULT '',
//                                         dict_type   TEXT DEFAULT '',
//                                         status      TEXT      DEFAULT '0',
//                                         create_time TIMESTAMP,
//                                         update_time TIMESTAMP,
//                                         remark      TEXT DEFAULT NULL
//                                     );";

// const DICT_DATA_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS dict_data
//                                     (
//                                         dict_code   INTEGER PRIMARY KEY AUTOINCREMENT,
//                                         dict_sort   INTEGER      DEFAULT 0,
//                                         dict_label  TEXT DEFAULT '',
//                                         dict_value  TEXT DEFAULT '',
//                                         dict_type   TEXT DEFAULT '',
//                                         css_class   TEXT DEFAULT NULL,
//                                         list_class  TEXT DEFAULT NULL,
//                                         is_default  TEXT      DEFAULT 'N',
//                                         status      TEXT      DEFAULT '0',
//                                         create_time TIMESTAMP,
//                                         update_time TIMESTAMP,
//                                         remark      TEXT DEFAULT NULL
//                                     );";

// const MENU_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS menu
//                                     (
//                                         menu_id     INTEGER PRIMARY KEY AUTOINCREMENT, -- 菜单ID
//                                         menu_name   TEXT NOT NULL,                     -- 菜单名称
//                                         parent_id   INTEGER  DEFAULT 0,                -- 父菜单ID
//                                         order_num   INTEGER  DEFAULT 0,                -- 显示顺序
//                                         path        TEXT     DEFAULT '',               -- 路由地址
//                                         component   TEXT     DEFAULT NULL,             -- 组件路径
//                                         query       TEXT     DEFAULT NULL,             -- 路由参数
//                                         route_name  TEXT     DEFAULT '',               -- 路由名称
//                                         is_frame    INTEGER  DEFAULT 1,                -- 是否为外链（0是 1否）
//                                         is_cache    INTEGER  DEFAULT 0,                -- 是否缓存（0缓存 1不缓存）
//                                         menu_type   TEXT     DEFAULT '',               -- 菜单类型（M目录 C菜单 F按钮）
//                                         visible     TEXT     DEFAULT '0',              -- 菜单状态（0显示 1隐藏）
//                                         status      TEXT     DEFAULT '0',              -- 菜单状态（0正常 1停用）
//                                         perms       TEXT     DEFAULT NULL,             -- 权限标识
//                                         icon        TEXT     DEFAULT '#',              -- 菜单图标
//                                         create_by   TEXT     DEFAULT '',               -- 创建者
//                                         create_time DATETIME DEFAULT NULL,             -- 创建时间
//                                         update_by   TEXT     DEFAULT '',               -- 更新者
//                                         update_time DATETIME DEFAULT NULL,             -- 更新时间
//                                         remark      TEXT     DEFAULT ''                -- 备注
//                                     );";

// const NOTICE_TEMP_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS notice_temp
// (
//     temp_id       INTEGER PRIMARY KEY AUTOINCREMENT,
//     temp_name     TEXT NOT NULL,
//     notice_method TEXT NOT NULL,
//     content       TEXT NOT NULL,
//     create_time   TIMESTAMP,
//     temp_type     TEXT NOT NULL,
//     remark        TEXT
// );";

// const NOTICE_RECORD_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS notice_record
// (
//     notice_id     INTEGER PRIMARY KEY AUTOINCREMENT,
//     user_id       INTEGER NOT NULL,
//     order_number  TEXT    NULL,
//     notice_method TEXT    NOT NULL,
//     notice_type   TEXT    NOT NULL,
//     notice_time   TIMESTAMP,
//     title         TEXT    NOT NULL,
//     content       TEXT    NOT NULL,
//     result        TEXT,
//     remark        TEXT
// );";

// const EXPENDITURE_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS expenditure
// (
//     exp_id             INTEGER PRIMARY KEY AUTOINCREMENT,
//     order_id           TEXT,
//     cloth_ids          TEXT,
//     exp_title          TEXT    NOT NULL,
//     recv_account       INTEGER,
//     recv_account_title TEXT,
//     exp_type           TEXT    NOT NULL,
//     exp_amount         INTEGER NOT NULL,
//     create_time        TIMESTAMP,
//     remark             TEXT
// );";

// const MEMBERSHIP_LEVEL_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS membership_level (
//     level_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, -- 会员等级ID
//     level_code TEXT NOT NULL,                            -- 等级编码
//     level_name TEXT NOT NULL,                            -- 等级名称
//     level_sort INTEGER NOT NULL,                         -- 显示顺序
//     status TEXT NOT NULL,                                -- 状态（0: 正常, 1: 停用）
//     create_time DATETIME DEFAULT NULL,                   -- 创建时间
//     update_time DATETIME DEFAULT NULL,                   -- 更新时间
//     remark TEXT DEFAULT NULL                             -- 备注
// );";

// const USER_MEMBERSHIP_LEVEL_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS user_membership_level (
//     user_id INTEGER NOT NULL,      -- 用户ID
//     level_id INTEGER NOT NULL,     -- 会员等级ID
//     PRIMARY KEY (user_id, level_id) -- 复合主键: 用户ID和会员等级ID
// );";

// pub(crate) const DDL: &[&str] = &[
//     USER_TABLE_DDL,
//     TAG_TABLE_DDL,
//     PRINTERS_TABLE_DDL,
//     CLOTHING_TABLE_DDL,
//     DRYING_RACK_TABLE_DDL,
//     CLOTH_SEQUENCE_TABLE_DDL,
//     CLOTH_PRICE_TABLE_DDL,
//     ORDER_TABLE_DDL,
//     ORDER_ADJUST_TABLE_DDL,
//     ORDER_CLOTHES_TABLE_DDL,
//     ORDER_PICTURES_TABLE_DDL,
//     PAYMENT_TABLE_DDL,
//     COUPON_TABLE_DDL,
//     USER_COUPON_TABLE_DDL,
//     COUPON_ORDER_TABLE_DDL,
//     CONFIG_TABLE_DDL,
//     USER_TAGS_TABLE_DDL,
//     LOCAL_USER_TABLE_DDL,
//     DICT_DATA_TABLE_DDL,
//     DICT_TYPE_TABLE_DDL,
//     MENU_TABLE_DDL,
//     NOTICE_TEMP_TABLE_DDL,
//     NOTICE_RECORD_TABLE_DDL,
//     EXPENDITURE_TABLE_DDL,
//     MEMBERSHIP_LEVEL_TABLE_DDL,
//     USER_MEMBERSHIP_LEVEL_TABLE_DDL,
// ];

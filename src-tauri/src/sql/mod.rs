const USER_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS users (
                                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                                    name TEXT NOT NULL
                                )";

const TAG_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS tags (
                                tag_id     INTEGER PRIMARY KEY AUTOINCREMENT,
                                tag_number VARCHAR(50) UNIQUE NOT NULL,
                                tag_order  VARCHAR(3),
                                tag_name   VARCHAR(50) UNIQUE NOT NULL,
                                ref_num    INTEGER DEFAULT 0,
                                order_num  INTEGER DEFAULT 0,
                                status     CHAR(1) DEFAULT '0',
                                del_flag   CHAR(1) DEFAULT '0',
                                remark     VARCHAR(500)
                            )";

const PRINTERS_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS printers (
                                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                                        name TEXT NOT NULL,
                                        system_name TEXT NOT NULL,
                                        driver_name TEXT NOT NULL
                                    )";

const CLOTHING_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS clothing
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
                                    )";

const DRYING_RACK_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS drying_rack
                                    (
                                        id                 INTEGER PRIMARY KEY AUTOINCREMENT,
                                        name               VARCHAR(50) NOT NULL,
                                        rack_type          char(1) DEFAULT '1',
                                        capacity           INTEGER NOT NULL,
                                        remaining_capacity INTEGER NOT NULL,
                                        position           INTEGER NOT NULL DEFAULT 0
                                    )";

/// 用来存储衣物的编码最大值
const CLOTH_SEQUENCE_TABLE_DDL: &str = "CREATE TABLE IF NOT EXISTS cloth_sequence
                                        (
                                            id              INTEGER PRIMARY KEY AUTOINCREMENT,
                                            date            DATE    NOT NULL,
                                            sequence_number INTEGER NOT NULL
                                        )";

pub(crate) const DDL: &[&str] = &[USER_TABLE_DDL, TAG_TABLE_DDL, PRINTERS_TABLE_DDL, CLOTHING_TABLE_DDL, DRYING_RACK_TABLE_DDL];

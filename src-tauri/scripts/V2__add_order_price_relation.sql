-- Author: 张三
-- Date: 2023-10-01
-- Description: 初始化数据库结构

-- 修改oder表字段
ALTER TABLE orders DROP COLUMN price_id;
-- 创建订单价格关系表
CREATE TABLE IF NOT EXISTS order_price_relations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id INTEGER NOT NULL,
    price_id INTEGER NOT NULL,
    create_time DATETIME,
    FOREIGN KEY (order_id) REFERENCES orders(order_id),
    FOREIGN KEY (price_id) REFERENCES cloth_price(price_id)
);
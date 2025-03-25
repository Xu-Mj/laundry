-- 二维码支付信息表
CREATE TABLE qrcode_payments
(
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    pay_id            INTEGER NOT NULL,                -- 关联的支付记录ID
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
// 订单来源
export const OrderSource = [
  { label: "小程序", value: "MiniProgram", type: "primary", icon: "Iphone" },
  { label: "抖音", value: "Douyin", type: "success", icon: "Goods" },
  { label: "美团", value: "Meituan", type: "warning", icon: "Food" },
  { label: "到店", value: "Store", type: "success", icon: "School" },
  { label: "其他", value: "Other", type: "danger", icon: "More" }
];

export const OrderSourceMap = Object.fromEntries(
  OrderSource.map(item => [item.value, item])
);

// 订单状态
export const OrderStatus = [
  { label: "正在洗护", value: "Processing" },
  { label: "上挂待取", value: "ReadyForPickup" },
  { label: "已完成", value: "Completed" },
  { label: "已退单", value: "Cancelled" },
  { label: "退单退款", value: "Refunded" }
];

export const OrderStatusMap = Object.fromEntries(
  OrderStatus.map(item => [item.value, item])
);

// 衣物状态
export const ClothStatus = [
  { label: "已取走", value: "PickedUp", type: "success" },
  { label: "洗护中", value: "Processing", type: "warning" },
  { label: "上挂待取", value: "ReadyForPickup", type: "success" },
  { label: "已退款", value: "Refunded", type: "danger" },
  { label: "配送中", value: "Delivering", type: "warning" },
  { label: "已送达", value: "Delivered", type: "success" },
  { label: "配送完成", value: "DeliveryCompleted", type: "success" },
];

export const ClothStatusMap = Object.fromEntries(
  ClothStatus.map(item => [item.value, item])
);

// 卡券类型
export const CouponType = [
  { label: "储值卡", value: "StoredValueCard" },
  { label: "折扣卡", value: "DiscountCard" },
  { label: "满减券", value: "SpendAndSaveCard" },
  { label: "折扣券", value: "DiscountCoupon" },
  { label: "次卡", value: "SessionCard" }
];

export const CouponTypeMap = Object.fromEntries(
  CouponType.map(item => [item.value, item])
);

// 支付方式
export const PaymentMethod = [
  { label: "其他", value: "Other", type: "info" },
  { label: "储值卡", value: "StoredValueCard", type: "success" },
  { label: "折扣卡", value: "DiscountCard", type: "success" },
  { label: "次卡", value: "SessionCard", type: "warning" },
  { label: "现金", value: "Cash", type: "warning" },
  { label: "支付宝", value: "Alipay", type: "primary" },
  { label: "微信支付", value: "WechatPay", type: "primary" },
  { label: "美团结转", value: "Meituan", type: "warning" },
  { label: "抖音结转", value: "Douyin", type: "success" },
  { label: "现金+储值卡", value: "CashAndStoredValueCard", type: "warning" },
  { label: "支付宝+储值卡", value: "AlipayAndStoredValueCard", type: "primary" },
  { label: "微信支付+储值卡", value: "WechatPayAndStoredValueCard", type: "primary" },
  { label: "现金+折扣卡", value: "CashAndDiscountCard", type: "warning" },
  { label: "支付宝+折扣卡", value: "AlipayAndDiscountCard", type: "primary" },
  { label: "微信支付+折扣卡", value: "WechatPayAndDiscountCard", type: "primary" },
  { label: "现金+满减券", value: "CashAndSpendAndSaveCard", type: "warning" },
  { label: "支付宝+满减券", value: "AlipayAndSpendAndSaveCard", type: "primary" },
  { label: "微信支付+满减券", value: "WechatPayAndSpendAndSaveCard", type: "primary" },
  { label: "现金+折扣券", value: "CashAndDiscountCoupon", type: "warning" },
  { label: "支付宝+折扣券", value: "AlipayAndDiscountCoupon", type: "primary" },
  { label: "微信支付+折扣券", value: "WechatPayAndDiscountCoupon", type: "primary" },
  { label: "现金+次卡", value: "CashAndSessionCard", type: "warning" },
  { label: "支付宝+次卡", value: "AlipayAndSessionCard", type: "primary" },
  { label: "微信支付+次卡", value: "WechatPayAndSessionCard", type: "primary" }
];

export const PaymentMethodMap = Object.fromEntries(
  PaymentMethod.map(item => [item.value, item])
);

export const CouponPaymentMethod = [
  { label: "支付宝", value: "Alipay", icon: "Money"  },
  { label: "微信", value: "WechatPay", icon: "ChatDotRound" },
  { label: "现金", value: "Cash", icon: "CreditCard"  },
  { label: "其他", value: "Other", icon: "More" },
]

// 支付状态
export const PaymentStatus = [
  { label: "未支付", value: "Unpaid" },
  { label: "已支付", value: "Paid" },
  { label: "已退款", value: "Refunded" }
];

export const PaymentStatusMap = Object.fromEntries(
  PaymentStatus.map(item => [item.value, item])
);

// 提醒类型
export const AlarmType = [
  { label: "正常", value: "Normal" },
  { label: "即将超时", value: "Warning" },
  { label: "已超时", value: "Overdue" }
];

export const AlarmTypeMap = Object.fromEntries(
  AlarmType.map(item => [item.value, item])
);

// 服务需求类型
export const ServiceRequirmentType = [
  { label: "正常", value: "Normal", icon: "CircleCheck", type: "success" },
  { label: "加急", value: "Emergency", icon: "AlarmClock", type: "danger" },
  { label: "单洗", value: "SingleWash", icon: "Box", type: "warning" },
  { label: "其他", value: "Other", icon: "More", type: "info" }
];

export const ServiceRequirmentMap = Object.fromEntries(
  ServiceRequirmentType.map(item => [item.value, item])
);

export const ServiceType = [
  { label: "洗护", value: "Processing", icon: "TakeawayBox" },
  { label: "熨烫", value: "Ironing", icon: "HotWater" },
  { label: "扦裤脚", value: "Hemming", icon: "Discount" },
  { label: "其他", value: "Other", icon: "More" }
];

export const ServiceTypeMap = Object.fromEntries(
  ServiceType.map(item => [item.value, item])
);

// 标签类型
export const TagType = [
  { label: "品牌", value: "Brand" },
  { label: "颜色", value: "Color" },
  { label: "洗前瑕疵", value: "PreCleaningFlaws" },
  { label: "洗后预估", value: "PostCleaningProjection" }
];
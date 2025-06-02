use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum OrderStatus {
    #[default]
    Processing,
    ReadyForPickup,
    Completed,
    Cancelled,
    Refunded,
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Processing => write!(f, "Processing"),
            OrderStatus::ReadyForPickup => write!(f, "Ready for pickup"),
            OrderStatus::Completed => write!(f, "Completed"),
            OrderStatus::Cancelled => write!(f, "Cancelled"),
            OrderStatus::Refunded => write!(f, "Refunded"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentOrderType {
    #[default]
    Laundry,
    Coupon,
}

impl Display for PaymentOrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentOrderType::Laundry => write!(f, "Laundry"),
            PaymentOrderType::Coupon => write!(f, "Coupon"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ClothStatus {
    PickedUp,
    #[default]
    Processing,
    ReadyForPickup,
    Refunded,
    Delivering,
    Delivered,
    DeliveryCompleted,
    // unnecessary this type, if canceld, the status will be ReadyForPickup
    // DeliveryCanceled,
}
impl Display for ClothStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClothStatus::PickedUp => write!(f, "Picked up"),
            ClothStatus::Processing => write!(f, "Processing"),
            ClothStatus::ReadyForPickup => write!(f, "Ready for pickup"),
            ClothStatus::Refunded => write!(f, "Refunded"),
            ClothStatus::Delivering => write!(f, "Delivering"),
            ClothStatus::Delivered => write!(f, "Delivered"),
            ClothStatus::DeliveryCompleted => write!(f, "DeliveryCompleted"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CouponType {
    #[default]
    StoredValueCard,
    DiscountCard,
    SpendAndSaveCard,
    DiscountCoupon,
    SessionCard,
}

impl Display for CouponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CouponType::StoredValueCard => write!(f, "StoredValueCard"),
            CouponType::DiscountCard => write!(f, "DiscountCard"),
            CouponType::SpendAndSaveCard => write!(f, "SpendAndSaveCard"),
            CouponType::DiscountCoupon => write!(f, "DiscountCoupon"),
            CouponType::SessionCard => write!(f, "SessionCard"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentMethod {
    Other,
    /// single method
    StoredValueCard,
    DiscountCard,
    SessionCard,
    Cash,
    #[default]
    Alipay,
    WechatPay,
    Meituan,
    Douyin,

    /// combination method
    CashAndStoredValueCard,
    AlipayAndStoredValueCard,
    WechatPayAndStoredValueCard,

    CashAndDiscountCard,
    AlipayAndDiscountCard,
    WechatPayAndDiscountCard,

    CashAndSpendAndSaveCard,
    AlipayAndSpendAndSaveCard,
    WechatPayAndSpendAndSaveCard,

    CashAndDiscountCoupon,
    AlipayAndDiscountCoupon,
    WechatPayAndDiscountCoupon,

    CashAndSessionCard,
    AlipayAndSessionCard,
    WechatPayAndSessionCard,
}

impl Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentMethod::Other => write!(f, "Other"),
            PaymentMethod::StoredValueCard => write!(f, "StoredValueCard"),
            PaymentMethod::DiscountCard => write!(f, "DiscountCard"),
            PaymentMethod::SessionCard => write!(f, "SessionCard"),
            PaymentMethod::Cash => write!(f, "Cash"),
            PaymentMethod::Alipay => write!(f, "Alipay"),
            PaymentMethod::WechatPay => write!(f, "WechatPay"),
            PaymentMethod::Meituan => write!(f, "Meituan"),
            PaymentMethod::Douyin => write!(f, "Douyin"),
            PaymentMethod::CashAndStoredValueCard => write!(f, "CashAndStoredValueCard"),
            PaymentMethod::AlipayAndStoredValueCard => write!(f, "AlipayAndStoredValueCard"),
            PaymentMethod::WechatPayAndStoredValueCard => write!(f, "WechatPayAndStoredValueCard"),
            PaymentMethod::CashAndDiscountCard => write!(f, "CashAndDiscountCard"),
            PaymentMethod::AlipayAndDiscountCard => write!(f, "AlipayAndDiscountCard"),
            PaymentMethod::WechatPayAndDiscountCard => write!(f, "WechatPayAndDiscountCard"),
            PaymentMethod::CashAndSpendAndSaveCard => write!(f, "CashAndSpendAndSaveCard"),
            PaymentMethod::AlipayAndSpendAndSaveCard => write!(f, "AlipayAndSpendAndSaveCard"),
            PaymentMethod::WechatPayAndSpendAndSaveCard => {
                write!(f, "WechatPayAndSpendAndSaveCard")
            }
            PaymentMethod::CashAndDiscountCoupon => write!(f, "CashAndDiscountCoupon"),
            PaymentMethod::AlipayAndDiscountCoupon => write!(f, "AlipayAndDiscountCoupon"),
            PaymentMethod::WechatPayAndDiscountCoupon => write!(f, "WechatPayAndDiscountCoupon"),
            PaymentMethod::CashAndSessionCard => write!(f, "CashAndSessionCard"),
            PaymentMethod::AlipayAndSessionCard => write!(f, "AlipayAndSessionCard"),
            PaymentMethod::WechatPayAndSessionCard => write!(f, "WechatPayAndSessionCard"),
        }
    }
}

impl PaymentMethod {
    pub fn get_other_payment_method(&self, current: &Self) -> Option<Self> {
        match current {
            Self::StoredValueCard => match self {
                Self::AlipayAndStoredValueCard => Some(Self::Alipay),
                Self::WechatPayAndStoredValueCard => Some(Self::WechatPay),
                Self::CashAndStoredValueCard => Some(Self::Cash),
                _ => None,
            },
            Self::SessionCard => match self {
                Self::AlipayAndSessionCard => Some(Self::Alipay),
                Self::WechatPayAndSessionCard => Some(Self::WechatPay),
                Self::CashAndSessionCard => Some(Self::Cash),
                _ => None,
            },
            Self::DiscountCard => match self {
                Self::AlipayAndDiscountCard => Some(Self::Alipay),
                Self::WechatPayAndDiscountCard => Some(Self::WechatPay),
                Self::CashAndDiscountCard => Some(Self::Cash),
                _ => None,
            },
            Self::AlipayAndDiscountCoupon
            | Self::WechatPayAndDiscountCoupon
            | Self::CashAndDiscountCoupon => match self {
                Self::AlipayAndDiscountCoupon => Some(Self::Alipay),
                Self::WechatPayAndDiscountCoupon => Some(Self::WechatPay),
                Self::CashAndDiscountCoupon => Some(Self::Cash),
                _ => None,
            },

            _ => None,
        }
    }

    pub fn get_coupon_another_method(&self) -> Option<Self> {
        match self {
            Self::AlipayAndDiscountCoupon => Some(Self::Alipay),
            Self::WechatPayAndDiscountCoupon => Some(Self::WechatPay),
            Self::CashAndDiscountCoupon => Some(Self::Cash),
            Self::AlipayAndSpendAndSaveCard => Some(Self::Alipay),
            Self::WechatPayAndSpendAndSaveCard => Some(Self::WechatPay),
            Self::CashAndSpendAndSaveCard => Some(Self::Cash),
            Self::CashAndSessionCard => Some(Self::Cash),
            Self::AlipayAndSessionCard => Some(Self::Alipay),
            Self::WechatPayAndSessionCard => Some(Self::WechatPay),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentStatus {
    #[default]
    Unpaid,
    Paid,
    Refunded,
}

impl Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentStatus::Unpaid => write!(f, "Unpaid"),
            PaymentStatus::Paid => write!(f, "Paid"),
            PaymentStatus::Refunded => write!(f, "Refunded"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AlarmType {
    #[default]
    Normal,
    Warning,
    Overdue,
    OverdueNotPickedUp,
}

impl Display for AlarmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlarmType::Normal => write!(f, "Normal"),
            AlarmType::Warning => write!(f, "Warning"),
            AlarmType::Overdue => write!(f, "Overdue"),
            AlarmType::OverdueNotPickedUp => write!(f, "OverdueNotPickedUp"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ServiceType {
    #[default]
    Processing,
    Ironing,
    Hemming,
    Other,
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceType::Processing => write!(f, "Processing"),
            ServiceType::Ironing => write!(f, "Ironing"),
            ServiceType::Hemming => write!(f, "Hemming"),
            ServiceType::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ServiceRequirmentType {
    #[default]
    Normal,
    SingleWash,
    Emergency,
    Other,
}

impl Display for ServiceRequirmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceRequirmentType::Normal => write!(f, "Normal"),
            ServiceRequirmentType::SingleWash => write!(f, "SingleWash"),
            ServiceRequirmentType::Emergency => write!(f, "Emergency"),
            ServiceRequirmentType::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TagType {
    Brand,
    #[default]
    Color,
    PreCleaningFlaws,
    PostCleaningProjection,
}

impl Display for TagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagType::Brand => write!(f, "Brand"),
            TagType::Color => write!(f, "Color"),
            TagType::PreCleaningFlaws => write!(f, "PreCleaningFlaws"),
            TagType::PostCleaningProjection => write!(f, "PostCleaningProjection"),
        }
    }
}

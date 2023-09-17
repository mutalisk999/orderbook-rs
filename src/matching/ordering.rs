use rust_decimal::Decimal;
use std::cmp::Ordering;

pub trait PriceOrderIdKeyOrdering {
    fn new(price: &Decimal, order_id: u64) -> Self;
}

pub struct PriceOrderIdKeyAsc {
    pub price: Decimal,
    pub order_id: u64,
}

impl PriceOrderIdKeyOrdering for PriceOrderIdKeyAsc {
    fn new(price: &Decimal, order_id: u64) -> Self {
        PriceOrderIdKeyAsc {
            price: price.clone(),
            order_id,
        }
    }
}

impl Eq for PriceOrderIdKeyAsc {}

impl PartialEq<Self> for PriceOrderIdKeyAsc {
    fn eq(&self, other: &Self) -> bool {
        self.price.eq(&other.price) && self.order_id.eq(&other.order_id)
    }
}

impl PartialOrd<Self> for PriceOrderIdKeyAsc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriceOrderIdKeyAsc {
    fn cmp(&self, other: &Self) -> Ordering {
        return match self.price.cmp(&other.price) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.order_id.cmp(&other.order_id) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            },
        };
    }
}

pub struct PriceOrderIdKeyDesc {
    pub price: Decimal,
    pub order_id: u64,
}

impl PriceOrderIdKeyOrdering for PriceOrderIdKeyDesc {
    fn new(price: &Decimal, order_id: u64) -> Self {
        PriceOrderIdKeyDesc {
            price: price.clone(),
            order_id,
        }
    }
}

impl Eq for PriceOrderIdKeyDesc {}

impl PartialEq<Self> for PriceOrderIdKeyDesc {
    fn eq(&self, other: &Self) -> bool {
        self.price.eq(&other.price) && self.order_id.eq(&other.order_id)
    }
}

impl PartialOrd<Self> for PriceOrderIdKeyDesc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriceOrderIdKeyDesc {
    fn cmp(&self, other: &Self) -> Ordering {
        return match self.price.cmp(&other.price) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => match self.order_id.cmp(&other.order_id) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            },
        };
    }
}

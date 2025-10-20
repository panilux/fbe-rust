//! Order struct

#[derive(Debug, Clone, Default)]
pub struct Order {
    pub symbol: String,
    pub side: OrderSide,
    pub type: OrderType,
    pub price: f64,
    pub volume: f64,
}

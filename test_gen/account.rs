//! Account struct

#[derive(Debug, Clone, Default)]
pub struct Account {
    pub name: String,
    pub state: State,
    pub wallet: Balance,
    pub asset: Option<Balance>,
    pub orders: Vec<Order>,
}

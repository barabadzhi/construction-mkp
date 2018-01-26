#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    pub profit: u16,
    pub weights: Box<[u16]>,
    pub used: bool,
    pub id: usize,
}

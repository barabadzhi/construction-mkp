use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Item {
    pub id: usize,
    pub profit: u32,
    pub weights: Box<[u32]>,
    pub weighted_profit: f64,
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        self.weighted_profit
            .partial_cmp(&other.weighted_profit)
            .unwrap_or(Ordering::Equal)
    }
}

impl Eq for Item {}

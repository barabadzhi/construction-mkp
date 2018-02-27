use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Item {
    pub id: usize,
    pub profit: u16,
    pub weights: Box<[u16]>,
    pub weighted_profit: f32,
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        self.weighted_profit
            .partial_cmp(&other.weighted_profit)
            .unwrap_or(Ordering::Equal)
    }
}

impl Eq for Item {}

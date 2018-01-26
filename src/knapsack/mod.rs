extern crate colored;

pub mod item;

use std::fmt;
use std::time::Duration;

use self::colored::*;

use self::item::Item;

#[derive(Default, Clone)]
pub struct Knapsack {
    pub n: usize,
    pub m: usize,
    pub items: Vec<Item>,
    pub capacity: Box<[u16]>,
    pub heuristic: String,
    pub statistics: Statistics,
}

#[derive(Debug, Default, Clone)]
pub struct Statistics {
    pub total_profit: u32,
    pub duration: Duration,
}

impl Knapsack {
    pub fn new() -> Knapsack {
        Knapsack {
            ..Default::default()
        }
    }
}

impl fmt::Debug for Knapsack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut items = String::new();

        for item in &self.items {
            items += &format!("\t{:?}\n", item);
        }

        items.pop();

        write!(
            f,
            r#"Knapsack {{
    heuristic: {},
    n: {},
    m: {},
    items:
    {},
    capacity: {:?},
    stistics: {:?}
}}"#,
            self.heuristic, self.n, self.m, items, self.capacity, self.statistics
        )
    }
}

impl fmt::Display for Knapsack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut items = Vec::new();

        for item in &self.items {
            if item.used {
                items.push(item.id);
            }
        }

        items.sort();

        let mut item_list = String::new();

        for item in &items {
            item_list += &format!("{:?}, ", item);
        }

        item_list.pop();
        item_list.pop();

        let mut time = String::new();

        if self.statistics.duration.as_secs() > 0 {
            time += &format!(
                "{} {}",
                self.statistics.duration.as_secs().to_string().green(),
                's'
            );
        }

        if self.statistics.duration.subsec_nanos() > 0 {
            if time.is_empty() {
                time += &format!(
                    "{} {}",
                    self.statistics.duration.subsec_nanos().to_string().green(),
                    "ns"
                )
            } else {
                time += &format!(
                    " {} {}",
                    self.statistics.duration.subsec_nanos().to_string().green(),
                    "ns"
                )
            }
        }

        write!(
            f,
            r#"{}
    -> Total profit: {}
    -> Items ({}): {}
    -> Duration: {}
"#,
            self.heuristic.cyan().bold(),
            self.statistics.total_profit.to_string().green(),
            items.len(),
            item_list.yellow(),
            time
        )
    }
}

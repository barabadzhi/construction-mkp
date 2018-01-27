extern crate colored;

pub mod item;

use std::fmt;
use std::fs::File;
use std::time::Duration;
use std::io::{BufRead, BufReader};

use self::colored::*;

use self::item::Item;

#[derive(Clone, Default)]
pub struct Knapsack {
    pub n: usize,
    pub m: usize,
    pub items: Vec<Item>,
    pub capacity: Box<[u16]>,
    pub heuristic: String,
    pub statistics: Statistics,
}

#[derive(Clone, Default, Debug)]
pub struct Statistics {
    pub total_profit: u32,
    pub duration: Duration,
}

impl Knapsack {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from(file: &str) -> Self {
        let file = File::open(file).expect("Input file is not specified");
        let reader = BufReader::new(file);

        let mut m = 0;
        let mut profits = Vec::new();
        let mut weights = Vec::new();

        let mut knapsack = Knapsack::new();

        for (line_number, contents) in reader.lines().enumerate() {
            let mut contents: Vec<u16> = contents
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u16>().unwrap())
                .collect();

            match line_number {
                0 => {
                    // n m q opt
                    debug_assert_eq!(contents.len(), 4);

                    knapsack.n = contents[0] as usize;
                    knapsack.m = contents[1] as usize;
                    knapsack.items = Vec::with_capacity(knapsack.m);

                    m = knapsack.m + 1;
                }
                1 => {
                    // a line with the n obj. func. coefficients
                    debug_assert_eq!(contents.len(), knapsack.n);
                    profits = contents;
                }
                _ if m >= line_number => {
                    // a line for each m; n coefficients for <= constraints
                    weights.push(contents.into_boxed_slice());
                }
                _ => {
                    // a line with rhs of <= constraints
                    debug_assert_eq!(contents.len(), knapsack.m);
                    knapsack.capacity = contents.into_boxed_slice();
                }
            }
        }

        for (index, profit) in profits.into_iter().enumerate() {
            let mut item_weights = Vec::with_capacity(weights.len());

            for weight in &weights {
                item_weights.push(weight[index]);
            }

            knapsack.items.push(Item {
                profit: profit,
                weights: item_weights.into_boxed_slice(),
                used: false,
                id: index + 1,
            });
        }

        knapsack
    }

    pub fn heuristic(mut self, name: &str) -> Self {
        self.heuristic = name.into();
        self
    }
}

impl fmt::Debug for Knapsack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut items = String::new();

        for item in &self.items {
            items += &format!("\t{:?}\n", item);
        }

        items.trim_right_matches('\n');

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

        let mut list_of_items = String::new();

        for item in &items {
            list_of_items += &format!("{:?}, ", item);
        }

        list_of_items.trim_right_matches(", ");

        let mut run_time = String::new();

        if self.statistics.duration.as_secs() > 0 {
            run_time += &format!(
                "{} {}",
                self.statistics.duration.as_secs().to_string().green(),
                's'
            );
        }

        if self.statistics.duration.subsec_nanos() > 0 {
            if run_time.is_empty() {
                run_time += &format!(
                    "{} {}",
                    self.statistics.duration.subsec_nanos().to_string().green(),
                    "ns"
                )
            } else {
                run_time += &format!(
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
            list_of_items.yellow(),
            run_time
        )
    }
}

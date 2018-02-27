extern crate rand;
extern crate rayon;

pub mod item;
pub mod statistics;

use self::rand::{thread_rng, Rng};
use self::rayon::prelude::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::sync::RwLock;

use self::item::Item;
use self::statistics::Statistics;

#[derive(Default, Debug)]
pub struct Knapsack {
    m: usize,
    n: usize,
    items: Vec<Item>,
    capacity: Box<[u32]>,
}

impl Knapsack {
    fn new() -> Knapsack {
        Knapsack {
            ..Default::default()
        }
    }

    pub fn from(file: &str) -> Knapsack {
        let file = File::open(file).expect("Input file is not specified");
        let reader = BufReader::new(file);

        let mut m = 0;
        let mut profits = Vec::new();
        let mut weights = Vec::new();

        let mut knapsack = Knapsack::new();

        for (line_number, contents) in reader.lines().enumerate() {
            let mut contents: Vec<u32> = contents
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
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

            let weighted_profit = f64::from(profit) / f64::from(item_weights.iter().sum::<u32>());

            knapsack.items.push(Item {
                id: index + 1,
                profit,
                weights: item_weights.into_boxed_slice(),
                weighted_profit,
            });
        }

        knapsack
    }

    pub fn run_greedy(&self) -> Statistics {
        let mut items = self.items.clone();
        let mut capacity_left = self.capacity.clone();
        let mut result = Statistics::new();

        let time = Instant::now();

        items.sort_unstable_by(|a, b| b.cmp(a));

        let mut item_can_be_used = false;

        for item in &items {
            for (index, constraint) in capacity_left.iter().enumerate() {
                if item.weights[index] > *constraint {
                    item_can_be_used = false;
                    break;
                } else {
                    item_can_be_used = true
                };
            }

            if item_can_be_used {
                for (index, constraint) in capacity_left.iter_mut().enumerate() {
                    *constraint -= item.weights[index];
                }

                result.picked_items.push(item.id.to_string());
                result.total_profit += u32::from(item.profit);
            }
        }

        result.duration = time.elapsed();
        result.runs = 1;

        for (left, total) in capacity_left.iter().zip(self.capacity.iter()) {
            result.utilization.push(format!(
                "{:.2}%",
                ((f64::from(*total - *left) / f64::from(*total)) * 100_f64)
            ))
        }

        result
    }

    pub fn run_random(&self, runs: usize) -> Statistics {
        let mut instances = Vec::with_capacity(runs);
        let mut indexes: Vec<usize> = (0..self.items.len()).collect();
        let result = RwLock::new(Statistics::new());

        let time = Instant::now();

        for _ in 0..runs {
            thread_rng().shuffle(&mut indexes);
            instances.push(indexes.to_vec());
        }

        instances.par_iter().for_each(|instance| {
            let mut capacity_left = self.capacity.clone();
            let mut internal_result = Statistics::new();

            for index in instance {
                let mut item_can_be_used = false;

                for (constraint_index, constraint) in capacity_left.iter().enumerate() {
                    if self.items[*index].weights[constraint_index] > *constraint {
                        item_can_be_used = false;
                        break;
                    } else {
                        item_can_be_used = true;
                    }
                }

                if item_can_be_used {
                    for (constraint_index, constraint) in capacity_left.iter_mut().enumerate() {
                        *constraint -= self.items[*index].weights[constraint_index];
                    }

                    internal_result
                        .picked_items
                        .push(self.items[*index].id.to_string());
                    internal_result.total_profit += u32::from(self.items[*index].profit);
                }
            }

            if internal_result.total_profit > result.read().unwrap().total_profit {
                for (left, total) in capacity_left.iter().zip(self.capacity.iter()) {
                    internal_result.utilization.push(format!(
                        "{:.2}%",
                        ((f64::from(*total - *left) / f64::from(*total)) * 100_f64)
                    ))
                }

                *result.write().unwrap() = internal_result.clone();
            }
        });

        let mut result = result.into_inner().unwrap();

        result.duration = time.elapsed();
        result.runs = runs;

        result
    }
}

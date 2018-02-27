extern crate rand;
extern crate rayon;

pub mod item;
pub mod statistics;

use self::rand::{thread_rng, Rng};
use self::rayon::prelude::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use self::item::Item;
use self::statistics::Statistics;

#[derive(Default, Debug, Clone)]
pub struct Knapsack {
    m: usize,
    n: usize,
    items: Vec<Item>,
    total_capacity: Box<[u16]>,
    capacity_left: Box<[u16]>,
    pub greedy_result: Statistics,
    pub random_result: Statistics,
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
                    knapsack.total_capacity = contents.clone().into_boxed_slice();
                    knapsack.capacity_left = contents.into_boxed_slice();
                }
            }
        }

        for (index, profit) in profits.into_iter().enumerate() {
            let mut item_weights = Vec::with_capacity(weights.len());

            for weight in &weights {
                item_weights.push(weight[index]);
            }

            let weighted_profit = f32::from(profit) / f32::from(item_weights.iter().sum::<u16>());

            knapsack.items.push(Item {
                id: index + 1,
                profit,
                weights: item_weights.into_boxed_slice(),
                weighted_profit,
            });
        }

        knapsack
    }

    pub fn run_greedy(&mut self) {
        let time = Instant::now();

        self.items.sort_unstable_by(|a, b| b.cmp(a));

        let mut item_can_be_used = false;

        for item in &self.items {
            for (index, constraint) in self.capacity_left.iter().enumerate() {
                if item.weights[index] > *constraint {
                    item_can_be_used = false;
                    break;
                } else {
                    item_can_be_used = true
                };
            }

            if item_can_be_used {
                for (index, constraint) in self.capacity_left.iter_mut().enumerate() {
                    *constraint -= item.weights[index];
                }

                self.greedy_result.picked_items.push(item.id.to_string());
                self.greedy_result.total_profit += u32::from(item.profit);
            }
        }

        for (left, total) in self.capacity_left.iter().zip(self.total_capacity.iter()) {
            self.greedy_result.utilization.push(format!(
                "{:.2}%",
                ((f32::from(*total - *left) / f32::from(*total)) * 100_f32)
            ))
        }

        self.greedy_result.duration = time.elapsed();
        self.greedy_result.runs = 1;
    }

    pub fn run_random(&mut self, runs: usize) {
        let mut knapsacks = Vec::new();

        for _ in 0..runs {
            let mut k = self.clone();
            thread_rng().shuffle(&mut k.items);
            knapsacks.push(k);
        }

        debug_assert_eq!(knapsacks.len(), runs);

        let time = Instant::now();

        knapsacks.par_iter_mut().for_each(|knapsack| {
            let mut item_can_be_used = false;

            for item in &mut knapsack.items {
                for (index, constraint) in knapsack.total_capacity.iter().enumerate() {
                    if item.weights[index] > *constraint {
                        item_can_be_used = false;
                        break;
                    } else {
                        item_can_be_used = true;
                    }
                }

                if item_can_be_used {
                    for (index, constraint) in knapsack.total_capacity.iter_mut().enumerate() {
                        *constraint -= item.weights[index];
                    }

                    knapsack
                        .random_result
                        .picked_items
                        .push(item.id.to_string());
                    knapsack.random_result.total_profit += u32::from(item.profit);
                }
            }
        });

        let mut best_profid_index = 0;

        for (index, knapsack) in &mut knapsacks.iter().enumerate() {
            if knapsack.random_result.total_profit
                > knapsacks[best_profid_index].random_result.total_profit
            {
                best_profid_index = index;
            }
        }

        self.random_result = knapsacks[best_profid_index].random_result.clone();
        self.random_result.duration = time.elapsed();
        self.random_result.runs = runs;
    }
}

extern crate clap;
extern crate rand;
extern crate rayon;

mod knapsack;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use clap::{App, Arg};
use rand::{thread_rng, Rng};
use rayon::prelude::*;

use knapsack::Knapsack;
use knapsack::item::Item;

fn main() {
    let matches = App::new("MKP")
        .version("0.1.0")
        .author("Bogdan Arabadzhi <bogdan.today@gmail.com>")
        .about("Construction heuristics for the multidimensional knapsack problem (MKP)")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Sets a custom input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("random")
                .short("r")
                .long("random")
                .value_name("NUMBER")
                .help("Sets a number of random heuristic iterations")
                .takes_value(true),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap_or("input.txt");
    let n = matches
        .value_of("random")
        .unwrap_or("10")
        .parse::<usize>()
        .unwrap();

    let mut knapsack = Knapsack::new();

    let file = File::open(input).expect("Input file is not specified");
    let reader = BufReader::new(file);

    let mut m = 0;
    let mut profits = Vec::new();
    let mut weights = Vec::new();

    for (number, contents) in reader.lines().enumerate() {
        let mut contents: Vec<u16> = contents
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u16>().unwrap())
            .collect();

        match number {
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
            _ if number <= m => {
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
        let mut w = Vec::with_capacity(weights.len());

        for weight in &weights {
            w.push(weight[index]);
        }

        knapsack.items.push(Item {
            profit: profit,
            weights: w.into_boxed_slice(),
            used: false,
            id: index + 1,
        });
    }

    knapsack.heuristic = String::from("Random");

    let mut knapsacks = Vec::with_capacity(n);

    for _ in 0..n {
        let mut k = knapsack.clone();
        thread_rng().shuffle(&mut k.items);
        knapsacks.push(k);
    }

    debug_assert_eq!(knapsacks.len(), n);

    let time = Instant::now();

    knapsacks.par_iter_mut().for_each(|knapsack| {
        let mut item_can_be_used = false;

        for item in &mut knapsack.items {
            for (index, constraint) in knapsack.capacity.iter().enumerate() {
                if item.weights[index] > *constraint {
                    item_can_be_used = false;
                    break;
                } else {
                    item_can_be_used = true;
                }
            }

            if item_can_be_used {
                for (index, constraint) in knapsack.capacity.iter_mut().enumerate() {
                    *constraint -= item.weights[index];
                    item.used = true;
                    knapsack.statistics.total_profit += u32::from(item.profit);
                }
            }
        }
    });

    let elapsed = time.elapsed();

    for knapsack in &mut knapsacks {
        knapsack.statistics.duration = elapsed;
    }

    knapsack.heuristic = String::from("Greedy");

    let time_2 = Instant::now();

    knapsack.items.sort_unstable_by(|a, b| b.cmp(a));

    let mut item_can_be_used = false;

    for item in &mut knapsack.items {
        for (index, constraint) in knapsack.capacity.iter().enumerate() {
            if item.weights[index] > *constraint {
                item_can_be_used = false;
                break;
            } else {
                item_can_be_used = true;
            }
        }

        if item_can_be_used {
            for (index, constraint) in knapsack.capacity.iter_mut().enumerate() {
                *constraint -= item.weights[index];
                item.used = true;
                knapsack.statistics.total_profit += u32::from(item.profit);
            }
        }
    }

    knapsack.statistics.duration = time_2.elapsed();

    println!("{}", knapsack);

    for item in knapsacks {
        println!("{}", item);
    }
}

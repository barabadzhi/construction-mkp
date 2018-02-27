extern crate clap;
extern crate colored;
extern crate rand;
extern crate rayon;

mod knapsack;

use clap::{App, Arg};
use self::colored::*;
// use rand::{thread_rng, Rng};
// use rayon::prelude::*;

use knapsack::Knapsack;

fn main() {
    let (file, _greedy_runs) = read_cmd_arguments();

    let mut knapsack = Knapsack::from(&file);

    knapsack.run_greedy();
    println!("{}{}", "Greedy".cyan().bold(), knapsack.greedy_result);

    // for _ in 0..greedy_runs {
    //     let mut k = knapsack.clone();
    //     thread_rng().shuffle(&mut k.items);
    //     knapsacks.push(k);
    // }

    // debug_assert_eq!(knapsacks.len(), greedy_runs);

    // let time = Instant::now();

    // knapsacks.par_iter_mut().for_each(|knapsack| {
    //     let mut item_can_be_used = false;

    //     for item in &mut knapsack.items {
    //         for (index, constraint) in knapsack.capacity.iter().enumerate() {
    //             if item.weights[index] > *constraint {
    //                 item_can_be_used = false;
    //                 break;
    //             } else {
    //                 item_can_be_used = true;
    //             }
    //         }

    //         if item_can_be_used {
    //             for (index, constraint) in knapsack.capacity.iter_mut().enumerate() {
    //                 *constraint -= item.weights[index];
    //                 item.used = true;
    //                 knapsack.statistics.total_profit += u32::from(item.profit);
    //             }
    //         }
    //     }
    // });

    // let elapsed = time.elapsed();

    // for knapsack in &mut knapsacks {
    //     knapsack.statistics.duration = elapsed;
    // }
}

fn read_cmd_arguments() -> (String, usize) {
    let matches = App::new("MKP")
        .version("0.1.2")
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

    let input = matches.value_of("input").unwrap_or("input.txt").to_string();

    let random = matches
        .value_of("random")
        .unwrap_or("10")
        .parse::<usize>()
        .unwrap();

    (input, random)
}

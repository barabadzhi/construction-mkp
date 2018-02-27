extern crate clap;
extern crate colored;

mod knapsack;

use clap::{App, Arg};
use colored::*;

use knapsack::Knapsack;

fn main() {
    let (file, n_times) = read_cmd_arguments();

    let mut knapsack = Knapsack::from(&file);

    knapsack.run_greedy();
    println!("{}{}", "Greedy".cyan().bold(), knapsack.greedy_result);

    knapsack.run_random(n_times);
    println!("{}{}", "Random".cyan().bold(), knapsack.random_result);
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

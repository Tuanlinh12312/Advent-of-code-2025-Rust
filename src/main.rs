use std::env::args;
use std::fs;

macro_rules! aoc_runner {
  ($($day:ident),*) => {
        $(mod $day; )*

        fn get_solutions() ->Vec<fn(&str) -> (String, String)> {
            vec![
                $(
                    |input: &str| {
                        use crate::$day::*;

                        let parsed = parse(input);
                        (p1(&parsed).to_string(), p2(&parsed).to_string())
                    },
                )*
            ]
        }
    };
}

aoc_runner!(day01, day02, day03, day04, day05, day06, day07);

fn main() {
    let solutions = get_solutions();
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day>");
        return;
    }

    let day_arg = args[1].parse::<usize>().expect("Day must be a number");
    let day_index = day_arg - 1;

    if day_index >= solutions.len() {
        println!("Day {} is not implemented yet.", day_arg);
        return;
    }

    let file_path = format!("inputs/day{:02}.txt", day_arg);
    let input = fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Could not read input file: {}", file_path));

    let solver_fn = solutions[day_index];
    let (p1, p2) = solver_fn(&input);

    println!("Day {} Output:", day_arg);
    println!("  Part 1: {}", p1);
    println!("  Part 2: {}", p2);
}

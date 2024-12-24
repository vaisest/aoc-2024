use crate::solvers::*;
use clap::Parser;
use std::{fs, hint::black_box, time::Instant};

mod solvers;
fn read_input(day: &str) -> String {
    let path = format!("input/day_{day}.txt");
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("could not read input file from {path} with error {e}"))
}

type SolverType = fn(String) -> String;
fn get_function_and_data(day: usize) -> ((SolverType, SolverType), String) {
    match day {
        1 => ((day01::part1, day01::part2), read_input("01")),
        2 => ((day02::part1, day02::part2), read_input("02")),
        3 => ((day03::part1, day03::part2), read_input("03")),
        4 => ((day04::part1, day04::part2), read_input("04")),
        5 => ((day05::part1, day05::part2), read_input("05")),
        6 => ((day06::part1, day06::part2), read_input("06")),
        7 => ((day07::part1, day07::part2), read_input("07")),
        8 => ((day08::part1, day08::part2), read_input("08")),
        9 => ((day09::part1, day09::part2), read_input("09")),
        10 => ((day10::part1, day10::part2), read_input("10")),
        11 => ((day11::part1, day11::part2), read_input("11")),
        12 => ((day12::part1, day12::part2), read_input("12")),
        13 => ((day13::part1, day13::part2), read_input("13")),
        14 => ((day14::part1, day14::part2), read_input("14")),
        15 => ((day15::part1, day15::part2), read_input("15")),
        16 => ((day16::part1, day16::part2), read_input("16")),
        17 => ((day17::part1, day17::part2), read_input("17")),
        18 => ((day18::part1, day18::part2), read_input("18")),
        19 => ((day19::part1, day19::part2), read_input("19")),
        20 => ((day20::part1, day20::part2), read_input("20")),
        21 => ((day21::part1, day21::part2), read_input("21")),
        22 => ((day22::part1, day22::part2), read_input("22")),
        23 => ((day23::part1, day23::part2), read_input("23")),
        24 => ((day24::part1, day24::part2), read_input("24")),
        // 25 => ((day25::part1, day25::part2), read_input("25")),
        _ => {
            todo!();
        }
    }
}
fn run_bench(day: usize, part: usize, f: SolverType, input: &String) {
    let timer = Instant::now();
    let mut run_count = 0;
    const MIN_TIME_MILLIS: u128 = 750;
    while timer.elapsed().as_millis() < MIN_TIME_MILLIS {
        black_box(f(black_box(input.clone())));
        run_count += 1;
        if run_count > 1000 {
            break;
        }
    }
    let total_time = timer.elapsed().as_millis();
    let per_run_millis = total_time as f64 / run_count as f64;
    println!("Day {day:2} part {part} benchmark: {run_count:6} runs in {total_time:3} ms at {per_run_millis:3.2} ms per run");
}

#[derive(Parser, Debug)]
struct Args {
    // Which specific day to run
    #[arg(short, long, default_value=None)]
    day: Option<usize>,

    // Run benchmarks instead of executing normally
    #[arg(short, long, default_value_t = false)]
    benchmark: bool,
}
fn main() {
    let args = Args::parse();

    let from = args.day.unwrap_or(1);
    let to = args.day.unwrap_or(24);
    for day in from..=to {
        let ((p1, p2), input) = get_function_and_data(day);
        if args.benchmark {
            run_bench(day, 1, p1, &input);
            run_bench(day, 2, p2, &input);
            println!();
        } else {
            println!("Day {day:2} part 1: {}", p1(input.clone()));
            println!("Day {day:2} part 2: {}", p2(input.clone()));
        }
    }
}

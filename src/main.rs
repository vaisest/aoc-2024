use std::fs;

mod solvers;

fn read_input(day: &str) -> String {
    let path = format!("input/day_{day}.txt");
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("could not read input file from {path} with error {e}"))
}

fn main() {
    solvers::day01::part1(read_input("01"));
}

use std::fs;

mod solvers;

fn read_input(day: &str) -> String {
    let path = format!("input/day_{day}.txt");
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("could not read input file from {path} with error {e}"))
}

fn main() {
    println!("Day 1 part 1: {}", solvers::day01::part1(read_input("01")));

    println!("Day 1 part 2: {}", solvers::day01::part2(read_input("01")));

    println!("Day 2 part 1: {}", solvers::day02::part1(read_input("02")));

    println!("Day 2 part 2: {}", solvers::day02::part2(read_input("02")));
}

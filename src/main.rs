use std::fs;
mod solvers;

fn read_input(day: &str) -> String {
    let path = format!("input/day_{day}.txt");
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("could not read input file from {path} with error {e}"))
}

fn main() {
    // i wish i knew how to put these in a for loop
    println!("Day 1 part 1: {}", solvers::day01::part1(read_input("01")));
    println!("Day 1 part 2: {}", solvers::day01::part2(read_input("01")));

    println!("Day 2 part 1: {}", solvers::day02::part1(read_input("02")));
    println!("Day 2 part 2: {}", solvers::day02::part2(read_input("02")));

    println!("Day 3 part 1: {}", solvers::day03::part1(read_input("03")));
    println!("Day 3 part 2: {}", solvers::day03::part2(read_input("03")));

    println!("Day 4 part 1: {}", solvers::day04::part1(read_input("04")));
    println!("Day 4 part 2: {}", solvers::day04::part2(read_input("04")));

    println!("Day 5 part 1: {}", solvers::day05::part1(read_input("05")));
    println!("Day 5 part 2: {}", solvers::day05::part2(read_input("05")));

    println!("Day 6 part 1: {}", solvers::day06::part1(read_input("06")));
    println!("Day 6 part 2: {}", solvers::day06::part2(read_input("06")));

    println!("Day 7 part 1: {}", solvers::day07::part1(read_input("07")));
    println!("Day 7 part 2: {}", solvers::day07::part2(read_input("07")));

    println!("Day 8 part 1: {}", solvers::day08::part1(read_input("08")));
    println!("Day 8 part 2: {}", solvers::day08::part2(read_input("08")));

    println!("Day 9 part 1: {}", solvers::day09::part1(read_input("09")));
    println!("Day 9 part 2: {}", solvers::day09::part2(read_input("09")));

    println!("Day 10 part 1: {}", solvers::day10::part1(read_input("10")));
    println!("Day 10 part 2: {}", solvers::day10::part2(read_input("10")));

    println!("Day 11 part 1: {}", solvers::day11::part1(read_input("11")));
    println!("Day 11 part 2: {}", solvers::day11::part2(read_input("11")));

    println!("Day 12 part 1: {}", solvers::day12::part1(read_input("12")));
    println!("Day 12 part 2: {}", solvers::day12::part2(read_input("12")));

    println!("Day 13 part 1: {}", solvers::day13::part1(read_input("13")));
    println!("Day 13 part 2: {}", solvers::day13::part2(read_input("13")));

    println!("Day 14 part 1: {}", solvers::day14::part1(read_input("14")));
    println!("Day 14 part 2: {}", solvers::day14::part2(read_input("14")));

    println!("Day 15 part 1: {}", solvers::day15::part1(read_input("15")));
    println!("Day 15 part 2: {}", solvers::day15::part2(read_input("15")));

    println!("Day 16 part 1: {}", solvers::day16::part1(read_input("16")));
    println!("Day 16 part 2: {}", solvers::day16::part2(read_input("16")));

    println!("Day 17 part 1: {}", solvers::day17::part1(read_input("17")));
    println!("Day 17 part 2: {}", solvers::day17::part2(read_input("17")));

    println!("Day 18 part 1: {}", solvers::day18::part1(read_input("18")));
    println!("Day 18 part 2: {}", solvers::day18::part2(read_input("18")));

    println!("Day 19 part 1: {}", solvers::day19::part1(read_input("19")));
    println!("Day 19 part 2: {}", solvers::day19::part2(read_input("19")));

    println!("Day 20 part 1: {}", solvers::day20::part1(read_input("20")));
    println!("Day 20 part 2: {}", solvers::day20::part2(read_input("20")));
}

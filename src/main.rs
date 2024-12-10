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
}

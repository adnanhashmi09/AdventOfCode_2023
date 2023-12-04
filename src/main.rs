use advent_of_code_2023::day1::{compute_answer_part_1, compute_answer_part_2};
use advent_of_code_2023::day2;
use advent_of_code_2023::day3;
use advent_of_code_2023::day4;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    let (_, ans) = compute_answer_part_1(input.as_str()).unwrap();
    println!("day1 part1 -> {}", ans);

    let input = fs::read_to_string("inputs/day1_2.txt").unwrap();
    let (_, ans) = compute_answer_part_2(input.as_str()).unwrap();
    println!("day1 part2 -> {}", ans);

    let input = fs::read_to_string("inputs/day2.txt").unwrap();
    let (_, ans) = day2::compute_answer(input.as_str()).unwrap();
    println!("day2 part1 -> {}", ans);

    let input = fs::read_to_string("inputs/day2.txt").unwrap();
    let (_, ans) = day2::compute_answer_part_2(input.as_str()).unwrap();
    println!("day2 part2 -> {}", ans);

    let input = fs::read_to_string("inputs/day3.txt").unwrap();
    let ans = day3::compute_answer(input.as_str()).unwrap();
    println!("day3 part1 -> {}", ans);

    let input = fs::read_to_string("inputs/day3.txt").unwrap();
    let ans = day3::compute_answer_part_2(input.as_str()).unwrap();
    println!("day3 part1 -> {}", ans);

    let input = fs::read_to_string("inputs/day4.txt").unwrap();
    let (_, ans) = day4::compute_answer(input.as_str()).unwrap();
    println!("day4 part1 -> {}", ans);

    let input = fs::read_to_string("inputs/day4.txt").unwrap();
    let (_, ans) = day4::compute_answer_part_2(input.as_str()).unwrap();
    println!("day4 part2 -> {}", ans);
}

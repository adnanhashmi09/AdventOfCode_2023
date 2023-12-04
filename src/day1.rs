#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha0, alpha1, alphanumeric0, digit1, newline},
    multi::{many_till, separated_list1},
    IResult,
};

use phf::phf_map;

static SPELLING_TO_INT: phf::Map<&'static str, u32> = phf_map! {
    "one" => 1,
    "eno" => 1,
    "two" => 2,
    "owt" => 2,
    "three" => 3,
    "eerht" => 3,
    "four" => 4,
    "ruof" => 4,
    "five" => 5,
    "evif" => 5,
    "six" => 6,
    "xis" => 6,
    "seven" => 7,
    "neves" => 7,
    "eight" => 8,
    "thgie" => 8,
    "nine" => 9,
    "enin" => 9,
};

fn parse_line(input: &str) -> IResult<&str, u32> {
    let (input, _) = alpha0(input)?;
    let (input, digits) = separated_list1(alpha1, digit1)(input)?;
    let ln = digits.len();

    let num: u32;
    let d1 = digits[0].chars().nth(0).unwrap().to_digit(10).unwrap();
    let d2 = digits[ln - 1].chars().last().unwrap().to_digit(10).unwrap();
    num = d1 * 10 + d2;

    let (input, _) = alpha0(input)?;
    Ok((input, num))
}

pub fn compute_answer_part_1(input: &str) -> IResult<&str, u32> {
    let (input, nums): (&str, Vec<u32>) = separated_list1(newline, parse_line)(input)?;
    let ans: u32 = nums.into_iter().fold(0, |acc, x| acc + x);
    Ok((input, ans))
}

/*+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
// PART 2
/*+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

fn parse_digit_and_spellings(input: &str) -> IResult<&str, &str> {
    alt((
        digit1,
        tag("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
    ))(input)
}

fn parse_digit_and_rev_spellings(input: &str) -> IResult<&str, &str> {
    alt((
        digit1,
        tag("eno"),
        tag("owt"),
        tag("eerht"),
        tag("ruof"),
        tag("evif"),
        tag("xis"),
        tag("neves"),
        tag("thgie"),
        tag("enin"),
    ))(input)
}

fn parse_line_part_2(input: &str) -> IResult<&str, u32> {
    let mut combined_parser = many_till(
        alt((parse_digit_and_spellings, take(1usize))),
        parse_digit_and_spellings,
    );

    let (input, (_, second)) = combined_parser(input)?;
    let digit: u32 = match SPELLING_TO_INT.get(second) {
        Some(&v) => v,
        None => second.chars().nth(0).unwrap().to_digit(10).unwrap(),
    };

    let (input, _) = alphanumeric0(input)?;
    Ok((input, digit))
}

fn parse_line_part_2_rev(input: &str) -> IResult<&str, u32> {
    let mut combined_parser = many_till(
        alt((parse_digit_and_rev_spellings, take(1usize))),
        parse_digit_and_rev_spellings,
    );

    let (input, (_, second)) = combined_parser(input)?;
    let digit: u32 = match SPELLING_TO_INT.get(second) {
        Some(&v) => v,
        None => second.chars().nth(0).unwrap().to_digit(10).unwrap(),
    };

    let (input, _) = alphanumeric0(input)?;
    Ok((input, digit))
}

fn reverse_lines(input: &str) -> String {
    input
        .lines()
        .map(|line| line.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn compute_answer_part_2(input: &str) -> IResult<&str, u32> {
    let rev_input = reverse_lines(input);
    let (input, nums): (&str, Vec<u32>) =
        separated_list1(newline, parse_line_part_2)(input).unwrap();

    let (_, nums_rev): (&str, Vec<u32>) =
        separated_list1(newline, parse_line_part_2_rev)(rev_input.as_str()).unwrap();

    let summed: u32 = nums
        .into_iter()
        .zip(nums_rev.into_iter())
        .map(|(d1, d2)| d1 * 10 + d2)
        .sum();

    Ok((input, summed))
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_compute_answer() {
        let (_, answer) = compute_answer_part_1(INPUT).unwrap();
        let expected_answer = 142;
        assert_eq!(answer, expected_answer);
    }

    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_compute_part_2() {
        let (_, answer) = compute_answer_part_2(INPUT2).unwrap();
        let expected_answer = 281;
        assert_eq!(answer, expected_answer);
    }
}

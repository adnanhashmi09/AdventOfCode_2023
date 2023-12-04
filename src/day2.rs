#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::{
        complete::{digit1, newline},
        is_newline,
        streaming::not_line_ending,
    },
    combinator::complete,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use phf::phf_map;

static COLOR_TO_LIMIT: phf::Map<&'static str, u32> = phf_map! {
    "red" => 12,
    "green"=> 13,
    "blue" => 14
};

fn parse_game(input: &str) -> IResult<&str, &str> {
    preceded(tag("Game "), digit1)(input)
}

fn parse_color_count(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        digit1,
        tag(" "),
        alt((tag("blue"), tag("green"), tag("red"))),
    )(input)
}

fn parse_punctuations(input: &str) -> IResult<&str, &str> {
    alt((tag(", "), tag("; ")))(input)
}

fn parse_line(input: &str) -> IResult<&str, Option<u32>> {
    if is_newline(input.chars().nth(0).unwrap_or('\n') as u8) {
        return Ok((input, Some(0)));
    }
    let (input, game_number) = parse_game(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, count_color_array) =
        many1(terminated(parse_color_count, complete(parse_punctuations)))(input)?;

    for (_, (count, color)) in count_color_array.into_iter().enumerate() {
        if let Some(&limit) = COLOR_TO_LIMIT.get(color) {
            let color_count = count.to_string().parse::<u32>().unwrap();
            if color_count > limit {
                let (input, _) = not_line_ending(input)?;
                return Ok((input, None));
            }
        }
    }

    let (input, (count, color)) = parse_color_count(input)?;
    if let Some(&limit) = COLOR_TO_LIMIT.get(color) {
        let color_count = count.to_string().parse::<u32>().unwrap();
        if color_count > limit {
            return Ok((input, None));
        }
    }

    Ok((input, Some(game_number.to_string().parse::<u32>().unwrap())))
}

pub fn compute_answer(input: &str) -> IResult<&str, u32> {
    let (input, games): (&str, Vec<Option<u32>>) = separated_list1(newline, parse_line)(input)?;
    let sum_of_games = games.into_iter().fold(0, |acc, x| match x {
        Some(x) => acc + x,
        None => acc + 0,
    });

    Ok((input, sum_of_games))
}

fn parse_line_part_2(input: &str) -> IResult<&str, u32> {
    if is_newline(input.chars().nth(0).unwrap_or('\n') as u8) {
        return Ok((input, 0));
    }
    let (input, _) = parse_game(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, count_color_array) =
        many1(terminated(parse_color_count, complete(parse_punctuations)))(input)?;

    let mut max_red: u32 = 1;
    let mut max_green: u32 = 1;
    let mut max_blue: u32 = 1;

    for (_, (count, color)) in count_color_array.into_iter().enumerate() {
        let color_count = count.to_string().parse::<u32>().unwrap();
        if color == "red" {
            max_red = std::cmp::max(max_red, color_count);
        }
        if color == "blue" {
            max_blue = std::cmp::max(max_blue, color_count);
        }
        if color == "green" {
            max_green = std::cmp::max(max_green, color_count);
        }
    }

    let (input, (count, color)) = parse_color_count(input)?;
    let color_count = count.to_string().parse::<u32>().unwrap();
    if color == "red" {
        max_red = std::cmp::max(max_red, color_count);
    }
    if color == "blue" {
        max_blue = std::cmp::max(max_blue, color_count);
    }
    if color == "green" {
        max_green = std::cmp::max(max_green, color_count);
    }

    Ok((input, max_red * max_green * max_blue))
}

pub fn compute_answer_part_2(input: &str) -> IResult<&str, u32> {
    let (input, games): (&str, Vec<u32>) = separated_list1(newline, parse_line_part_2)(input)?;
    let sum_of_games = games.into_iter().fold(0, |acc, x| acc + x);

    Ok((input, sum_of_games))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_compute_answer() {
        let (_, answer) = compute_answer(INPUT1).unwrap();
        let expected_answer = 8;

        assert_eq!(answer, expected_answer);
    }

    #[test]
    fn test_compute_answer_part_2() {
        let (_, answer) = compute_answer_part_2(INPUT1).unwrap();
        let expected_answer = 2286;

        assert_eq!(answer, expected_answer);
    }
}

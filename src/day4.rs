#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{digit1, newline, space0, space1},
    combinator::complete,
    multi::{many1, many_till, separated_list1},
    sequence::terminated,
    IResult,
};

fn parse_card(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    space1(input)
}

fn parse_winnings(input: &str) -> IResult<&str, (Vec<&str>, &str)> {
    many_till(terminated(digit1, space1), tag("| "))(input)
}

fn parse_mine(input: &str) -> IResult<&str, Vec<&str>> {
    many1(alt((terminated(digit1, space1), digit1)))(input)
}

fn parse_line(input: &str) -> IResult<&str, u32> {
    let (input, _) = parse_card(input)?;
    let (input, (winnings, _)) = parse_winnings(input)?;
    let (input, _) = space0(input)?;
    let (input, mine) = parse_mine(input)?;
    let (input, _) = complete(space0)(input)?;

    let winners: Vec<&str> = mine
        .into_iter()
        .filter(|num| winnings.contains(num))
        .collect();

    let total_matchings = winners.len() as u32;
    let base: u32 = 2;
    let mut cards_worth = 0;
    if total_matchings > 0 {
        cards_worth = 1 * base.pow(total_matchings - 1);
    }

    Ok((input, cards_worth))
}

pub fn compute_answer(input: &str) -> IResult<&str, u32> {
    let (input, points): (&str, Vec<u32>) = separated_list1(newline, parse_line)(input.trim())?;
    let total_points: u32 = points.into_iter().fold(0, |acc, x| acc + x);
    Ok((input, total_points))
}

fn parse_line_part_2(input: &str) -> IResult<&str, u32> {
    let (input, _) = parse_card(input)?;
    let (input, (winnings, _)) = parse_winnings(input)?;
    let (input, _) = space0(input)?;
    let (input, mine) = parse_mine(input)?;
    let (input, _) = complete(space0)(input)?;

    let winners: Vec<&str> = mine
        .into_iter()
        .filter(|num| winnings.contains(num))
        .collect();

    let total_matchings = winners.len() as u32;

    Ok((input, total_matchings))
}

pub fn compute_answer_part_2(input: &str) -> IResult<&str, u32> {
    let (input, points): (&str, Vec<u32>) =
        separated_list1(newline, parse_line_part_2)(input.trim())?;

    let mut cards_count: Vec<u32> = vec![1; points.len()];

    for (index, matchings) in points.into_iter().enumerate() {
        let count = cards_count[index];
        let mut pointer = index as u32 + matchings;

        while pointer > index as u32 {
            cards_count[pointer as usize] += count;
            pointer -= 1;
        }
    }

    let total_cards: u32 = cards_count.into_iter().fold(0, |acc, x| acc + x);
    Ok((input, total_cards))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_compute_answer() {
        let (_, answer) = compute_answer(INPUT).unwrap();
        let expected_answer = 13;

        assert_eq!(answer, expected_answer);
    }

    #[test]
    fn test_compute_answer_part_2() {
        let (_, answer) = compute_answer_part_2(INPUT).unwrap();
        let expected_answer = 30;

        assert_eq!(answer, expected_answer);
    }
}

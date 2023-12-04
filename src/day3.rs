#![allow(dead_code)]

use std::{collections::HashMap, vec};

fn check_if_engine_part(r: usize, c1: usize, c: usize, matrix: &Vec<Vec<char>>) -> bool {
    let dx = [1, 1, 0, -1, -1, -1, 0, 1];
    let dy = [0, 1, 1, 1, 0, -1, -1, -1];

    let max_rows = matrix.len() as i32;
    let max_cols = matrix[0].len() as i32;

    for i in c1..=c {
        for j in 0..8 {
            let nc = i as i32 + dx[j];
            let nr = r as i32 + dy[j];

            if nc >= 0
                && nc < max_cols
                && nr >= 0
                && nr < max_rows
                && matrix[nr as usize][nc as usize] != '.'
                && !matrix[nr as usize][nc as usize].is_digit(10)
            {
                return true;
            }
        }
    }

    false
}

pub fn compute_answer(input: &str) -> Result<u64, String> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut tot_sum = 0;

    for (r, row) in matrix.iter().enumerate() {
        let mut c = 0;
        while c < row.len() {
            if matrix[r][c].is_digit(10) {
                let c1 = c;
                let mut num: u64 = matrix[r][c].to_digit(10).unwrap() as u64;
                c += 1;
                while c < row.len() && r < matrix.len() && matrix[r][c].is_digit(10) {
                    num = num * 10 + matrix[r][c].to_digit(10).unwrap() as u64;
                    c += 1;
                }

                if check_if_engine_part(r, c1, c - 1, &matrix) {
                    tot_sum = tot_sum + num;
                }
            } else {
                c += 1;
            }
        }
    }

    Ok(tot_sum)
}

fn get_gear_ratio(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> u32 {
    let dx = [1, 1, 0, -1, -1, -1, 0, 1];
    let dy = [0, 1, 1, 1, 0, -1, -1, -1];
    let max_rows = matrix.len();
    let max_cols = matrix[0].len();
    let mut ctr = 0;
    let mut product = 1;
    let mut visited: HashMap<usize, Vec<bool>> = HashMap::new();

    visited.insert(row, vec![false; max_cols]);

    if row + 1 < max_rows {
        visited.insert(row + 1, vec![false; max_cols]);
    }

    if row > 0 {
        visited.insert(row - 1, vec![false; max_cols]);
    }

    for j in 0..8 {
        let nr = row as i32 + dy[j];
        let mut nc = col as i32 + dx[j];

        if nr < 0 || nc < 0 || nr > max_rows as i32 || nc > max_cols as i32 {
            continue;
        }

        if matrix[nr as usize][nc as usize].is_digit(10) {
            let mut min_col_index = nc;
            let original_col_index = nc;
            let mut flag = 0;

            while nc >= 0 && matrix[nr as usize][nc as usize].is_digit(10) {
                min_col_index = nc;
                if let Some(v) = visited.get_mut(&(nr as usize)) {
                    if v[nc as usize] == true {
                        flag = 1;
                        break;
                    }
                    v[nc as usize] = true;
                }
                nc -= 1;
            }
            let mut max_col_index = original_col_index;
            nc = original_col_index;

            while nc < max_cols as i32 && matrix[nr as usize][nc as usize].is_digit(10) {
                max_col_index = nc;
                if let Some(v) = visited.get_mut(&(nr as usize)) {
                    if v[nc as usize] == true && nc != original_col_index {
                        flag = 1;
                        break;
                    }
                    v[nc as usize] = true;
                }
                nc += 1;
            }

            if flag == 1 {
                continue;
            }

            let row = &matrix[nr as usize];
            let parsed_val = &row[(min_col_index as usize)..=(max_col_index as usize)]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            ctr += 1;
            if ctr > 2 {
                return 0;
            }
            product *= parsed_val;
        }
    }

    if ctr < 2 {
        return 0;
    }
    product
}

pub fn compute_answer_part_2(input: &str) -> Result<u32, String> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut ans = 0;

    for (r, row) in matrix.iter().enumerate() {
        let mut c = 0;
        while c < row.len() {
            if matrix[r][c] == '*' {
                let g_ratio = get_gear_ratio(r, c, &matrix);
                ans += g_ratio;
            }
            c += 1;
        }
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_compute_answer() {
        let answer = compute_answer(INPUT).unwrap();
        let expected_answer = 4361;
        assert_eq!(answer, expected_answer);
    }

    #[test]
    fn test_compute_answer_part_2() {
        let answer = compute_answer_part_2(INPUT).unwrap();
        let expected_answer = 467835;
        assert_eq!(answer, expected_answer);
    }
}

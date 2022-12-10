#![allow(dead_code)]
use crate::utils;
use crate::utils::matrix::Matrix;


pub fn run() -> () {
    let data = parse_data(&utils::read_lines(utils::Source::Day(8)));

    println!("Day 8");
    println!("Part 1: {}", solve1(&data));
    println!("Part 2: {}", solve2(&data));
}


#[inline(always)]
fn mark_visible(
    data: &Matrix<i32>,
    visible: &mut Matrix<bool>,
    last_height: &mut i32,
    (row, col): (usize, usize)
) {
    let height = *data.get_at(row, col);
    if height > *last_height {
        visible.set_at(row, col, true);
        *last_height = height;
    }
}

fn create_mask(data: &Matrix<i32>) -> Matrix<bool> {
    let (rows, cols) = data.shape();
    let mut visible = Matrix::with_shape((rows, cols), false);

    for row in 0..rows {
        let mut last_height = i32::MIN;
        for col in 0..cols {
            mark_visible(data, &mut visible, &mut last_height, (row, col));
        }
        let mut last_height = i32::MIN;
        for col in (0..cols).rev() {
            mark_visible(data, &mut visible, &mut last_height, (row, col));
        }
        // Transposed passes
        let mut last_height = i32::MIN;
        for col in 0..cols {
            mark_visible(data, &mut visible, &mut last_height, (col, row));
        }
        let mut last_height = i32::MIN;
        for col in (0..cols).rev() {
            mark_visible(data, &mut visible, &mut last_height, (col, row));
        }
    };

    visible
}


fn solve1(data: &Matrix<i32>) -> usize {
    create_mask(data).iter().filter(|&e| *e).count()
}

fn find_scores(data: &Matrix<i32>) -> Matrix<i32> {
    let (rows, cols) = data.shape();
    let mut scores = Matrix::with_shape((rows, cols), 1);

    for row in 0..rows {
        for col in 0..cols {
            let mut score = 1;
            let current = *data.get_at(row, col);

            let mut temp = 0;
            for c in (0..col).rev() {
                temp += 1;
                if *data.get_at(row, c) >= current { break };
            }
            score *= temp;

            let mut temp = 0;
            for c in (col+1)..cols {
                temp += 1;
                if *data.get_at(row, c) >= current { break };
            }
            score *= temp;

            let mut temp = 0;
            for r in (0..row).rev() {
                temp += 1;
                if *data.get_at(r, col) >= current { break };
            }
            score *= temp;

            let mut temp = 0;
            for r in (row+1)..rows {
                temp += 1;
                if *data.get_at(r, col) >= current { break };
            }
            score *= temp;

            scores.set_at(row, col, score);
        };
    };

    scores
}

fn solve2(data: &Matrix<i32>) -> i32 {
    find_scores(data).into_iter().reduce(i32::max).unwrap()
}


fn parse_data<T: AsRef<str>>(data: &[T]) -> Matrix<i32> {
    data.iter()
        .flat_map(|line| line.as_ref().chars().map(|c| c.to_digit(10).unwrap() as i32))
        .collect::<Matrix<_>>()
        .reshape_rows(data.len())
}


#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &[&str; 5] = &[
        "30373",
        "25512",
        "65332",
        "33549",
        "35390",
    ];

    #[test]
    fn part1() {
        let data = parse_data(DATA);
        assert_eq!(21, solve1(&data));
    }

    #[test]
    fn part2() {
        let data = parse_data(DATA);
        let scores = find_scores(&data);
        assert_eq!(4, *scores.get_at(1, 2));
        assert_eq!(8, *scores.get_at(3, 2));
        assert_eq!(8, solve2(&data));
    }
}

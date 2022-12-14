use std::collections::HashSet;

use crate::file;

fn intval(v: char) -> i32 {
    if v as u32 >= 97 {
        v as i32 - 96
    } else {
        v as i32 - 38
    }
}

pub fn solve() -> (String, String) {
    let input = file::read("day3");
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> i32 {
    input.split("\n").fold(0, |acc, line| {
        let len = line.len();

        acc + match [line[0..len / 2].to_string(), line[len / 2 ..].to_string()] {
            [x, y] => {
                x.chars().collect::<HashSet<char>>()
                    .intersection(&y.chars().collect::<HashSet<char>>())
                    .fold(0, |acc, c| acc + intval(*c))
            }
        }
    })
}

fn part2(input: &str) -> i32 {
    input.split("\n").collect::<Vec<&str>>().chunks(3).fold(0, |acc, group| {
        acc + match group {
            [x, y, z] => {
                x.chars().collect::<HashSet<char>>()
                    .intersection(&y.chars().collect::<HashSet<char>>()).map(|c| *c ).collect::<HashSet<char>>()
                    .intersection(&z.chars().collect::<HashSet<char>>())
                    .fold(0, |acc, c| acc + intval(*c))
            },
            _ => 0
        }
    })
}

use crate::file;

pub fn solve() -> (String, String) {
    let calories = file::read("day1");
    return (part1(&calories).to_string(), part2(&calories).to_string());
}

fn part1(calories: &str) -> i32 {
    calories.split("\n\n")
        .map(|x| x.lines())
        .map(|elf| elf.map(|item| item.parse::<i32>().unwrap())
             .reduce(|acc, item| acc + item))
        .max().unwrap().unwrap()
}

fn part2(calories: &str) -> i32 {
    let mut calories_by_elves = calories.split("\n\n")
        .map(|x| x.lines())
        .map(|elf| elf.map(|item| item.parse::<i32>().unwrap())
             .reduce(|acc, item| acc + item).unwrap())
        .collect::<Vec<i32>>();

    calories_by_elves.sort();

    calories_by_elves[calories_by_elves.len() - 3..].into_iter().sum()
}

use crate::file;

struct Direction {
    count: usize,
    from: usize,
    to: usize
}

fn build_direction(raw_direction: &str) -> Direction {
    match raw_direction.split(" ").into_iter().fold(Vec::new(), |mut acc, entry| {
        match entry.parse::<usize>() {
            Ok(n) => {
                acc.push(n);
                acc
            },
            _ => acc
        }
    }).as_slice() {
        [count, from, to] => Direction { count: *count, from: *from, to: *to },
        _ => panic!("Bad input")
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Direction>) {
    match input
        .split("\n\n")
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .as_slice() {
            [raw_crates, raw_directions] => {
                let crates = raw_crates.lines().rev().fold(Vec::new(), |mut stacks, crate_height| {
                    for (col, crate_in_col) in crate_height.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                        if stacks.len() < col + 1 {
                            stacks.push(Vec::<char>::new());
                        }
                        if crate_in_col[0] == '[' {
                            stacks[col].push(crate_in_col[1])
                        }
                    }
                    stacks
                });
                let directions = raw_directions.split("\n").into_iter().fold(Vec::new(), |mut acc, raw_direction| {
                    if raw_direction.len() != 0 {
                        acc.push(build_direction(raw_direction));
                    }
                    acc
                });
                (crates, directions)
            },
            _ => panic!("Bad input")
        }
}

pub fn solve() -> (String, String) {
    let input = file::read("day5");
    (part1(&input), part2(&input))
}

fn part1(input: &str) -> String {
    let (mut stacks, directions) = parse_input(input);

    for direction in directions {
        for _count in 0..direction.count {
            let add = stacks[direction.from - 1].pop().unwrap();
            stacks[direction.to - 1].push(add);
        }
    }

    let mut spells = Vec::new();

    for mut stack in stacks {
        spells.push(stack.pop().unwrap().to_string());
    }

    spells.join("")
}

fn part2(input: &str) -> String {
    let (mut stacks, directions) = parse_input(input);

    for direction in directions {
        for count in 0..direction.count {
            let len = stacks[direction.from - 1].len();
            let add = stacks[direction.from - 1].remove(len + count - direction.count);
            stacks[direction.to - 1].push(add);
        }
    }

    let mut spells = Vec::new();

    for mut stack in stacks {
        spells.push(stack.pop().unwrap().to_string());
    }

    spells.join("")
}

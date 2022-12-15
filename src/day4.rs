use crate::file;

pub fn solve() -> String {
    let input = file::read("day4");
    part1(&input).to_string()
}

fn part1(input: &str) -> i32 {
    input.trim().split("\n").fold(Vec::new(), |mut acc, line| {
        line.split(",").for_each(|chunk|{
            chunk.split("-").for_each(|num| {
                match num.parse::<i32>() {
                    Ok(n) => acc.push(n),
                    _ => panic!("bad input")
                };
            });
        });
        acc
    }).as_slice().chunks(4).fold(0, |acc, assignment| {
        if assignment[0] <= assignment[2] && assignment[1] >= assignment[3] {
            acc + 1
        } else if assignment[0] >= assignment[2] && assignment[1] <= assignment[3] {
            acc + 1
        } else {
            acc
        }
    })
}

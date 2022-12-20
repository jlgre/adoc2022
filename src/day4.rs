use crate::file;

pub fn solve() -> (String, String) {
    let input = file::read("day4");
    (part1(&input).to_string(), part2(&input).to_string())
}

fn act_on_line<F>(input: &str, f: F) -> i32 where
    F: FnMut(i32, &[i32]) -> i32 {
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
    }).as_slice().chunks(4).fold(0, f)
}

fn part1(input: &str) -> i32 {
    act_on_line(input, |acc, assignment| {
        if assignment[0] <= assignment[2] && assignment[1] >= assignment[3] {
            acc + 1
        } else if assignment[0] >= assignment[2] && assignment[1] <= assignment[3] {
            acc + 1
        } else {
            acc
        }
    })
}

fn part2(input: &str) -> i32 {
    act_on_line(input, |acc, assignment| {
        // let real_coverage = assignment.iter().max().unwrap() - assignment.iter().min().unwrap();
        // let max_coverage = (assignment[1] - assignment[0]) + (assignment[3] - assignment[2]);
        // acc + (max_coverage - real_coverage)
        if assignment[1] >= assignment[2] && assignment[0] <= assignment[3] {
            acc + 1
        } else {
            acc
        }
    })
}

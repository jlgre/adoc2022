use crate::file;

struct Code {
    points: i32
}

impl Code {
    fn beats(&self, other: &Code) -> bool {
        let result = self.points - other.points;
        result == 1 || result == -2
    }

    fn draws(&self, other: &Code) -> bool {
        self.points == other.points
    }
}

fn mk_code(key: &str) -> Option<Code> {
    match key {
        "A" | "X" => Some(Code { points: 1 }),
        "B" | "Y" => Some(Code { points: 2 }),
        "C" | "Z" => Some(Code { points: 3 }),
        _ => None
    }
}

pub fn solve() -> String {
    let input = file::read("day2");

    return part1(&input).to_string();
}

pub fn part1(keys: &str) -> i32 {
    keys.lines().fold(0, |acc, key| {
        acc + match key.split(" ").map(|val| {
            mk_code(val).unwrap()
        }).collect::<Vec<Code>>().as_slice() {
            [theirs, mine] => {
                if mine.beats(theirs) {
                    mine.points + 6
                } else if mine.draws(theirs) {
                    mine.points + 3
                } else {
                    mine.points + 0
                }
            },
            _ => panic!("bad input")
        }
    })
}

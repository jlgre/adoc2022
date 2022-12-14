use crate::file;

struct Play {
    points: i32
}

enum Outcome {
    Lose,
    Draw,
    Win
}

impl Play {
    fn result(&self, other: &Play) -> Outcome {
        let result = self.points - other.points;
        if result == 0 {
            Outcome::Draw
        } else if result == 1 || result == -2 {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    fn points(&self, outcome: &Outcome) -> i32 {
        match outcome {
            Outcome::Win => self.points + 6,
            Outcome::Draw => self.points + 3,
            Outcome::Lose => self.points + 0
        }
    }

    fn should(&self, outcome: &Outcome) -> Play {
        match outcome {
            Outcome::Win => {
                match (self.points + 1) % 3 {
                    0 => Play { points: 3 },
                    x => Play { points: x }
                }
            },
            Outcome::Draw => Play { points: self.points },
            Outcome::Lose => {
                match self.points - 1 {
                    0 => Play { points: 3 },
                    x => Play { points: x }
                }
            }
        }
    }
}

fn mk_play(key: &str) -> Option<Play> {
    match key {
        "A" | "X" => Some(Play { points: 1 }),
        "B" | "Y" => Some(Play { points: 2 }),
        "C" | "Z" => Some(Play { points: 3 }),
        _ => None
    }
}

fn mk_outcome(key: &str) -> Option<Outcome> {
    match key {
        "X" => Some(Outcome::Lose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None
    }
}

pub fn solve() -> (String, String) {
    let input = file::read("day2");

    return (part1(&input).to_string(), part2(&input).to_string());
}

pub fn part1(keys: &str) -> i32 {
    keys.lines().fold(0, |acc, key| {
        acc + match key.split(" ").map(|val| {
            mk_play(val).unwrap()
        }).collect::<Vec<Play>>().as_slice() {
            [theirs, mine] => {
                mine.points(&mine.result(theirs))
            },
            _ => panic!("bad input")
        }
    })
}

pub fn part2(keys: &str) -> i32 {
    keys.lines().fold(0, |acc, key| {
        acc + match key.split(" ").collect::<Vec<&str>>().as_slice() {
            [theirs, outcome] => {
                let desired_outcome = &mk_outcome(outcome).unwrap();
                mk_play(theirs).unwrap().should(desired_outcome).points(desired_outcome)
            }
            _ => panic!("bad input")
        }
    })
}

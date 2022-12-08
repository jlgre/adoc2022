use crate::file;

pub fn solve() -> (String, String) {
    let calories = file::read("day1");
    return (part1(&calories).to_string(), part2(&calories).to_string());
}

fn part1(calories: &str) -> i32 {
    let mut max = 0;

    calories.split("\n\n").map(|x| x.lines()).for_each(|elf| {
        let carried = elf.map(|x| x.parse::<i32>().unwrap()).reduce(|acc, item| {
            acc + item
        }).unwrap();

        if carried > max {
            max = carried;
        }
    });

    return max;
}

fn part2(calories: &str) -> i32 {
    let mut maxes = [0, 0, 0];

    calories.split("\n\n").map(|x| x.lines()).for_each(|elf| {
        let carried = elf.map(|x| x.parse::<i32>().unwrap()).reduce(|acc, item| {
            acc + item
        }).unwrap();

        for (i, max) in maxes.into_iter().enumerate() {
            if carried > max {
                maxes[i] = carried;
                maxes.sort_unstable();
                break;
            }
        }
    });

    return maxes.into_iter().reduce(|acc, max| acc + max).unwrap();
}

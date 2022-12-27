use std::collections::LinkedList;

use crate::file;

pub fn solve() -> (String, String) {
    let input = file::read("day6");
    (part1(&input), part2(&input))
}

fn index_of_unique(to_search: &str, n: usize) -> i32 {
    let mut tracker = LinkedList::new();

    for (i, c) in to_search.chars().enumerate() {
        if tracker.len() < n {
            tracker.push_back(c);
        }
        if tracker.len() == n {
            let mut sorted: Vec<_> = tracker.clone().into_iter().collect();
            sorted.sort();
            sorted.dedup();

            if sorted.len() == n {
                return (i + 1).try_into().unwrap()
            }

            tracker.pop_front();
        }
    }

    -1
}

fn part1(input: &str) -> String {
    index_of_unique(input, 4).to_string()
}

fn part2(input: &str) -> String {
    index_of_unique(input, 14).to_string()
}

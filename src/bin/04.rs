use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut uniq = HashSet::new();
            line.split_whitespace().all(|word| uniq.insert(word))
        })
        .filter(|x| *x)
        .count() as u32
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            line.split_whitespace()
                .map(|w| {
                    let mut map = HashMap::new();
                    for b in w.as_bytes() {
                        *map.entry(b).or_insert(0) += 1;
                    }
                    map
                })
                .permutations(2)
                .all(|pair| pair[0] != pair[1])
        })
        .count() as u32
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 4), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_two(&input), 3);
    }
}

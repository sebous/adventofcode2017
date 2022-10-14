use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut uniq = HashSet::new();
            line.split_whitespace().all(|bytes| uniq.insert(bytes))
        })
        .filter(|x| *x)
        .count() as u32
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let mut uniq = HashSet::new();
            let words_b = line.split_whitespace();

            for word_b in words_b {
                let char_permutations = word_b.chars().permutations(word_b.len()).unique();
                for char_perm in char_permutations {
                    let is_uniq = uniq.insert(char_perm);
                    if !is_uniq {
                        return false;
                    }
                }
            }
            true
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

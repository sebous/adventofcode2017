use itertools::Itertools;

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|ch| ch.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|ch| ch.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|row| {
            for x in row.into_iter().permutations(2).unique() {
                let first = x[0] as f32;
                let sec = x[1] as f32;
                let res = if sec > first {
                    sec / first
                } else {
                    first / sec
                };
                if res.fract() == 0.0 {
                    return res as u32;
                }
            }
            0
        })
        .sum()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 2), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), 18);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), 9);
    }
}

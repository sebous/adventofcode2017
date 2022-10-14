use itertools::Itertools;

pub fn part_one(input: &str) -> u32 {
    let mut instructions = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect_vec();
    let mut i: usize = 0;
    let mut counter = 0;

    loop {
        let next_i: isize = i as isize + instructions[i];
        counter += 1;

        if next_i < 0 || next_i >= instructions.len() as isize {
            break;
        }

        instructions[i] += 1;
        i = next_i as usize;
    }
    counter
}

pub fn part_two(input: &str) -> u32 {
    let mut instructions = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect_vec();
    let mut i: usize = 0;
    let mut counter = 0;
    let instr_len = instructions.len() as isize;

    loop {
        let offset = instructions[i];
        let next_i: isize = i as isize + offset;
        counter += 1;

        if next_i < 0 || next_i >= instr_len {
            break;
        }

        instructions[i] += if offset >= 3 { -1 } else { 1 };
        i = next_i as usize;
    }
    counter
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 5), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_two(&input), 10);
    }
}

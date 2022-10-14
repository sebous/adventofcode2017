use itertools::Itertools;

fn distribute(input: &str) -> (u32, usize) {
    let input = input.split_whitespace().map(|n| n.parse::<u8>().unwrap());
    let mut history = vec![input.clone().collect_vec()];

    let mut memory = input.collect_vec();
    let memory_len = memory.len();

    loop {
        let largest_val = memory.iter().max().unwrap();
        let start_index = memory
            .iter()
            .enumerate()
            .filter(|(_, x)| x == &largest_val)
            .min_by(|(a, _), (b, _)| a.cmp(b))
            .map(|(i, _)| i)
            .unwrap();

        let mut i = start_index;
        let mut memory_distr = largest_val.to_owned();

        memory[i] = 0;

        while memory_distr > 0 {
            if i == memory_len - 1 {
                i = 0;
            } else {
                i += 1;
            }

            memory[i as usize] += 1;
            memory_distr -= 1;
        }

        if history.iter().any(|x| x == &memory) {
            let loop_len = history
                .iter()
                .enumerate()
                .find(|(_, x)| *x == &memory)
                .map(|(i, _)| i)
                .unwrap();
            return (history.len() as u32, history.len() - loop_len);
        }
        history.push(memory.clone());
    }
}

pub fn part_one(input: &str) -> u32 {
    let (cycles, _) = distribute(input);
    cycles
}

pub fn part_two(input: &str) -> u32 {
    let (_, loop_len) = distribute(input);
    loop_len as u32
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 6), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_two(&input), 4);
    }
}

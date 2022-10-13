fn parse_input(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

pub fn part_one(input: &str) -> u32 {
    let input = parse_input(input);
    let mut sum = 0;
    for (i, num) in input.iter().enumerate() {
        let next_i = if i == input.len() - 1 { 0 } else { i + 1 };
        if num == &input[next_i] {
            sum += num;
        }
    }
    sum
}

pub fn part_two(input: &str) -> u32 {
    let input = parse_input(input);
    let step = input.len() / 2;
    let mut sum = 0;
    for (i, num) in input.iter().enumerate() {
        let next_i = if i + step > input.len() - 1 {
            (i + step) % input.len()
        } else {
            i + step
        };
        if num == &input[next_i] {
            sum += num;
        }
    }
    sum
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 1), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_two(&input), 0);
    }
}

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

enum Operator {
    Higher,
    Lower,
    HigherOrEqual,
    LowerOrEqual,
    Equal,
    NotEqual,
}

fn check_condition(left: &i32, right: &i32, operator: &Operator) -> bool {
    match operator {
        Operator::Higher => left > right,
        Operator::Lower => left < right,
        Operator::HigherOrEqual => left >= right,
        Operator::LowerOrEqual => left <= right,
        Operator::Equal => left == right,
        Operator::NotEqual => left != right,
    }
}

fn parse_input(input: &str) {}

pub fn part_one(input: &str) -> i32 {
    let mut registers: HashMap<&str, i32> = HashMap::new();

    for line in input.lines() {
        let elements = line.split(" ").collect_vec();
        let register = elements[0];
        let increase = elements[1] == "inc";
        let value: i32 = elements[2].parse().unwrap();
        // skip 3
        let reg_condition = elements[4];
        let operator = match elements[5] {
            ">" => Operator::Higher,
            "<" => Operator::Lower,
            ">=" => Operator::HigherOrEqual,
            "<=" => Operator::LowerOrEqual,
            "==" => Operator::Equal,
            "!=" => Operator::NotEqual,
            _ => panic!("unknown operator"),
        };
        let value_condition: i32 = elements[6].parse().unwrap();

        let reg_cond_val = registers.get(reg_condition).unwrap_or(&0);
        if !check_condition(reg_cond_val, &value_condition, &operator) {
            continue;
        }

        if increase {
            *registers.entry(register).or_insert(0) += value;
        } else {
            *registers.entry(register).or_insert(0) -= value;
        }
    }
    registers.values().max().unwrap().to_owned()
}

pub fn part_two(input: &str) -> i32 {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut all_values = HashSet::new();

    for line in input.lines() {
        let elements = line.split(" ").collect_vec();
        let register = elements[0];
        let increase = elements[1] == "inc";
        let value: i32 = elements[2].parse().unwrap();
        // skip 3
        let reg_condition = elements[4];
        let operator = match elements[5] {
            ">" => Operator::Higher,
            "<" => Operator::Lower,
            ">=" => Operator::HigherOrEqual,
            "<=" => Operator::LowerOrEqual,
            "==" => Operator::Equal,
            "!=" => Operator::NotEqual,
            _ => panic!("unknown operator"),
        };
        let value_condition: i32 = elements[6].parse().unwrap();

        let reg_cond_val = registers.get(reg_condition).unwrap_or(&0);
        if !check_condition(reg_cond_val, &value_condition, &operator) {
            continue;
        }

        let mut new_val = registers.get(register).unwrap_or(&0).to_owned();
        all_values.insert(new_val);

        if increase {
            new_val += value;
        } else {
            new_val -= value;
        }

        registers.insert(register, new_val);
    }
    all_values.iter().max().unwrap().to_owned()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 8), part_one, part_two)
}

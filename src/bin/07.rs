use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_one(input: &str) -> String {
    let relations = input
        .lines()
        .filter(|l| l.contains("->"))
        .flat_map(|l| {
            let (left, _) = l.split_once(" ").unwrap();
            let (_, right_things) = l.split_once("->").unwrap();

            let rights = right_things.trim().split(", ").collect_vec();
            rights.iter().map(|r| (left, r.to_owned())).collect_vec()
        })
        .collect_vec();

    let (result, _) = relations
        .iter()
        .find(|(l, _)| !relations.iter().any(|(_, xr)| l == xr))
        .unwrap();

    result.to_string()
}

#[derive(Debug)]
struct NodeInfo {
    weight: u32,
    children: Vec<String>,
}

fn find_bal(key: &String, tree: &HashMap<String, NodeInfo>) -> Option<u32> {
    let mut unique = HashSet::new();
    for child_key in tree.get(key).unwrap().children.iter() {
        let item = tree.get(child_key).unwrap();
        let was_uniq = unique.insert(item.weight);

        dbg!(child_key, item);
        if !was_uniq {
            return Some(1);
        } else {
            return find_bal(child_key, tree);
        }
    }
    None
}

pub fn part_two(input: &str) -> u32 {
    let mut tree: HashMap<String, NodeInfo> = HashMap::new();

    for line in input.lines() {
        let split = line.split(" ").collect_vec();
        let name = split[0];
        let weight: u32 = split[1][1..split[1].len() - 1].parse().unwrap();

        let mut info = NodeInfo {
            weight,
            children: vec![],
        };

        if line.contains("->") {
            let (_, rels) = line.split_once(" -> ").unwrap();
            let rels = rels.split(", ").map(|str| str.to_string());
            info.children = rels.collect_vec();
        }

        tree.insert(name.to_owned(), info);
    }

    let root = tree
        .iter()
        .find(|(key, _)| !tree.iter().any(|(_, info)| info.children.contains(key)))
        .unwrap();

    dbg!(find_bal(root.0, &tree));

    unimplemented!()

    // let failing_parent = tree
    //     .iter()
    //     .find(|(_, info)| {
    //         !tree
    //             .iter()
    //             .filter(|(key, _)| info.children.contains(key))
    //             .map(|(_, info)| info.weight)
    //             .all_equal()
    //     })
    //     .map(|x| x.1)
    //     .unwrap();

    // let values = tree
    //     .iter()
    //     .filter(|(key, _)| failing_parent.children.contains(key))
    //     .map(|(_, info)| info.weight)
    //     .collect_vec();

    // // let failing_weight = values
    // //     .iter()
    // //     .find(|v| values.iter().filter(|x| x == v).count() == 1)
    // //     .unwrap();

    // let good_weight = values
    //     .iter()
    //     .find(|v| values.iter().filter(|x| x == v).count() > 1)
    //     .unwrap();

    // // failing_parent.to_owned()
    // good_weight.to_owned()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 7), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_two() {
    //     use aoc::read_file;
    //     let input = read_file("examples", 7);
    //     assert_eq!(part_two(&input), 0);
    // }
}

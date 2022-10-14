use std::collections::HashMap;

fn create_grid(target: u32) -> HashMap<(i32, i32), u32> {
    let mut grid = HashMap::new();
    grid.insert((0, 0), 1);

    let mut value = 1;
    let mut size: u32 = 1;
    let mut next_target = 1;
    let mut x = 0;
    let mut y = 0;

    while value <= target {
        if value == next_target + 1 {
            size += 2;
            next_target = size.pow(2);
        }
        let last_max = if size == 1 { 1 } else { (size - 2).pow(2) };

        // step right
        if value == last_max + 1 {
            x += 1;
        }

        // right -> top
        if value > last_max + 1 && value < last_max + size {
            y += 1;
        }

        // top -> left
        if value >= last_max + size && value < last_max + size * 2 - 1 {
            x -= 1;
        }

        // left -> down
        if value >= last_max + size * 2 - 1 && value < last_max + size * 3 - 2 {
            y -= 1;
        }

        // down -> right
        if value >= last_max + size * 3 - 2 && value <= next_target {
            x += 1;
        }
        grid.insert((x, y), value);

        value += 1;
    }

    grid
}

fn count_adj(grid: &HashMap<(i32, i32), u32>, x: i32, y: i32) -> u32 {
    let mut sum = 0;
    // println!("-----");
    for x in x - 1..x + 2 {
        for y in y - 1..y + 2 {
            // println!("{x},{y}");
            if let Some(val) = grid.get(&(x, y)) {
                sum += val;
            }
        }
    }
    sum
}

fn create_grid_p2(target: u32) -> u32 {
    let mut grid = HashMap::new();

    grid.insert((0, 0), 1);

    let mut value = 1;
    let mut size: u32 = 1;
    let mut next_target = 1;
    let mut x = 0;
    let mut y = 0;

    while value <= target {
        if value == next_target + 1 {
            size += 2;
            next_target = size.pow(2);
        }
        let last_max = if size == 1 { 1 } else { (size - 2).pow(2) };

        // step right
        if value == last_max + 1 {
            x += 1;
        }

        // right -> top
        if value > last_max + 1 && value < last_max + size {
            y += 1;
        }

        // top -> left
        if value >= last_max + size && value < last_max + size * 2 - 1 {
            x -= 1;
        }

        // left -> down
        if value >= last_max + size * 2 - 1 && value < last_max + size * 3 - 2 {
            y -= 1;
        }

        // down -> right
        if value >= last_max + size * 3 - 2 && value <= next_target {
            x += 1;
        }

        let sum = count_adj(&grid, x, y);
        if sum > target {
            return sum;
        }
        grid.insert((x, y), sum);

        value += 1;
    }

    0
}

pub fn part_one(input: &str) -> u32 {
    let target: u32 = input.parse().unwrap();
    let grid = create_grid(target);

    let (x, y) = grid
        .iter()
        .find_map(|(key, &val)| if val == target { Some(key) } else { None })
        .unwrap();
    let result = (0 - x).abs() + (0 - y).abs();
    result as u32
}

pub fn part_two(input: &str) -> u32 {
    let target: u32 = input.parse().unwrap();
    let sum = create_grid_p2(target);
    sum
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 3), part_one, part_two)
}

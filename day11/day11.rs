use std::fs;
use memoize::memoize;

fn read_input(filename: &str) -> Vec<i64> {
    let contents: String = fs::read_to_string(filename).expect("Can't read file!");
    let input: Vec<i64> = contents
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    return input
}

fn blink(stones: Vec<i64>) -> Vec<i64> { // Consume the original stone list since we're replacing it
    // It's actually easier to work with these as strings funnily enough...
    let mut new_stones: Vec<i64> = Vec::new();
    for s in stones.into_iter() {
        let stone_string: String = s.to_string();
        if stone_string == "0" { 
            new_stones.push(1);
        } else if stone_string.len() % 2 == 0 {
            let left_stone_string: &str = &stone_string[..stone_string.len()/2];
            let right_stone_string:&str = &stone_string[stone_string.len()/2..];
            new_stones.push(left_stone_string.parse::<i64>().unwrap());
            new_stones.push(right_stone_string.parse::<i64>().unwrap());
        } else {
            new_stones.push(s * 2024);
        }
    }
    return new_stones
}

#[memoize]
fn count_stones(stone_number: i64, blinks: u32) -> u64 {
    let new_stones = blink(vec![stone_number]);
    if blinks == 1 { // This is the end of the recursion 
        return new_stones.len() as u64
    } else {
        let mut stone_count: u64 = 0;
        for s in new_stones {
            stone_count += count_stones(s, blinks-1);
        }
        return stone_count
    }
}

fn part1(input: &Vec<i64>) {
    let mut stones = input.clone();
    for _ in 0..25 {
        stones = blink(stones);
    }
    println!("{}", stones.len());
}

fn part2(input: &Vec<i64>) {
    let mut stones = input.clone();
    let mut stone_count: u64 = 0;
    for s in stones {
        stone_count += count_stones(s, 75_u32);
    }
    println!("{}", stone_count);
}

fn main() {
    let input = read_input("input.txt");
    part1(&input);
    part2(&input);
}
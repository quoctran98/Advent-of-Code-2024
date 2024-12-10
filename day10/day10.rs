use std::fs;
use std::collections::HashSet;

fn read_input(filename: &str) -> Vec<Vec<u32>> {
    let mut output: Vec<Vec<u32>> = Vec::new();
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    for line in contents.lines() {
        output.push(line.chars().map(|x| x.to_digit(10).unwrap() as u32).collect());
    }
    return output
}

fn find_trailends(trailhead: (usize, usize), map: &Vec<Vec<u32>>) -> HashSet<(usize, usize)> {
    let mut nines: HashSet<(usize, usize)> = HashSet::new();
    let this_height: u32 = map[trailhead.0][trailhead.1];
    // println!("New branch at ({}, {}) at heigh   t {}", trailhead.0, trailhead.1, this_height);
    for (x, y) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        // Adding i32 to usize and clamping is annoying...
        let next_x: usize = (((trailhead.0 as i32) + x).max(0) as usize).min(map.len() - 1);
        let next_y: usize = (((trailhead.1 as i32) + y).max(0) as usize).min(map[0].len() - 1);
        let next_height: u32 = map[next_x][next_y];
        if this_height + 1 == next_height {
            if next_height == 9 {
                // println!("Completed trail at ({}, {})", next_x, next_y);
                nines.insert((next_x, next_y)); // Don't return here! We have to complete the loop!
            }
            nines.extend(find_trailends((next_x, next_y), map));
        }
    }
    return nines
}

// Lol! I accidentally counted possible trails instead of trail endings for part 1, so this will be easy!
fn rate_trailhead(trailhead: (usize, usize), map: &Vec<Vec<u32>>) -> u32 {
    let mut rating: u32 = 0;
    let this_height: u32 = map[trailhead.0][trailhead.1];
    for (x, y) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        // Adding i32 to usize and clamping is annoying...
        let next_x: usize = (((trailhead.0 as i32) + x).max(0) as usize).min(map.len() - 1);
        let next_y: usize = (((trailhead.1 as i32) + y).max(0) as usize).min(map[0].len() - 1);
        let next_height: u32 = map[next_x][next_y];
        if this_height + 1 == next_height {
            if next_height == 9 {
                rating += 1;
            }
            rating += rate_trailhead((next_x, next_y), map)
        }
    }
    return rating
}

fn part1(map: &Vec<Vec<u32>>) {
    let mut score: u32 = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 0 {
                score += find_trailends((x, y), map).len() as u32;
            }
        }
    }
    println!("{}", score);
}

fn part2(map: &Vec<Vec<u32>>) {
    let mut score: u32 = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 0 {
                score += rate_trailhead((x, y), map);
            }
        }
    }
    println!("{}", score);
}

fn main() {
    let input = read_input("input.txt");
    part1(&input);
    part2(&input);
}
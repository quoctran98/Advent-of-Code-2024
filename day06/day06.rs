use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::collections::HashSet;

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    for line in contents.lines() {
        let char_array = line.chars().collect();
        output.push(char_array);
    }
    return output
}

/// I should write a generic Grid class for AoC...
/// It'll also be good to about Rust classes, etc.
/// This ideally should be a separate importable module (crate?) but this is fine :)
struct Grid {
    width: i32,
    height: i32,
    objects: Vec<(i32, i32)>, // (0, 0) is top left
    walked: Vec<(i32, i32)>,
    done: bool
}

impl Grid {
    fn walk_until_object(&mut self, start_pos: (i32, i32), direction: (i32, i32)) -> ((i32, i32), (i32, i32)) {
        // Returns ((x, y), dir)
        // Direction should be (1, 0), (-1, 0), (0, 1), or (0, -1) :)
        assert!(((direction.0 + direction.1).abs() == 1) && (direction.0 * direction.1 == 0));
        let next_direction = (-1*direction.1, direction.0);
        let mut flag = false;
        if direction.0 != 0 { // Look for objects on the same row
            let obstructions: Vec<(i32, i32)> = self.objects.iter().filter(|&(x, _)| x == &start_pos.0).map(|&(x, y)| (x, y)).collect();
            let mut obstructions_y: Vec<i32> = obstructions.into_iter().map(|(_, y)| y).collect();
            let hit_y: i32;
            // Keep only values greater or less than start_pos.1
            obstructions_y = obstructions_y
                .into_iter()
                .filter(|&y| if direction.0 < 0 { y < start_pos.1 } else { y > start_pos.1 })
                .collect();
            if obstructions_y.len() == 0 {
                // No obstructions; guard's leaving but we still have to mark things down!
                self.done = true;
                hit_y = if direction.0 < 0 { 0 } else { self.height - 1};
            } else {
                if direction.0 < 0 {
                    hit_y = obstructions_y.into_iter().max().unwrap() + 1 // We stop one away from it!
                } else {
                    hit_y = obstructions_y.into_iter().min().unwrap() - 1
                }
            }
            // Mark self.walked! (Rust doesn't allow backwards ranges!!)
            for y in std::cmp::min(start_pos.1, hit_y)..=std::cmp::max(start_pos.1, hit_y) {
                flag = true;
                self.walked.push((start_pos.0, y));
            }
            if !flag {
                println!("Problem between {:?} and {:?}", start_pos, (start_pos.0, hit_y))
            }
            return ((start_pos.0, hit_y), next_direction)

        } else { // Look for objects on the same column
            let obstructions: Vec<(i32, i32)> = self.objects.iter().filter(|&(_, y)| y == &start_pos.1).map(|&(x, y)| (x, y)).collect();
            let mut obstructions_x: Vec<i32> = obstructions.into_iter().map(|(x, _)| x).collect();
            let hit_x: i32;
            // Keep only values greater or less than start_pos.0
            obstructions_x = obstructions_x
                .into_iter()
                .filter(|&x| if direction.1 < 0 { x < start_pos.0 } else { x > start_pos.0 })
                .collect();
            if obstructions_x.len() == 0 {
                // No obstructions; guard's leaving but we still have to mark things down!
                self.done = true;
                hit_x = if direction.1 < 0 { 0 } else { self.width - 1};
            } else {
                // Go up to the obstruction
                if direction.1 < 1 {
                    hit_x = obstructions_x.into_iter().max().unwrap() + 1; // We stop one away from it!
                } else {
                    // We want the MIN! if we're going right/down :)
                    hit_x = obstructions_x.into_iter().min().unwrap() - 1
                }
                
            }
            for x in std::cmp::min(start_pos.0, hit_x)..=std::cmp::max(start_pos.0, hit_x){ 
                flag = true;
                self.walked.push((x, start_pos.1));
            }
            if !flag {
                println!("Problem between {:?} and {:?}", start_pos, (hit_x, start_pos.1))
            }
            return ((hit_x, start_pos.1), next_direction)
        }
    }
}

impl Grid {
    fn write_to_txt(&self, fname: String) -> io::Result<()> {
        let file = File::create(fname)?;
        let mut writer = BufWriter::new(file);
        for row in 0..self.height {
            let mut line: Vec<char> = Vec::new();
            let obstructions: Vec<(i32, i32)> = self.objects.iter().filter(|&(x, _)| x == &row).map(|&(x, y)| (x, y)).collect();
            let obstructions_y: Vec<i32> = obstructions.into_iter().map(|(_, y)| y).collect();
            let walked: Vec<(i32, i32)> = self.walked.iter().filter(|&(x, _)| x == &row).map(|&(x, y)| (x, y)).collect();
            let walked_y: Vec<i32> = walked.into_iter().map(|(_, y)| y).collect();
            for col in 0..self.width {
                if obstructions_y.contains(&col) {
                    line.push('#');
                } else if walked_y.contains(&col) {
                    line.push('X');
                } else {
                    line.push('.');
                }
            }
            // Write this line to the wrtier buffer
            writeln!(writer, "{}", line.into_iter().collect::<String>())?;
        }
        writer.flush()?;
        Ok(())
    }
}


fn part1() {
    let input = read_input("input.txt");
    let mut map = Grid {
        width: input.len() as i32,
        height: input[0].len() as i32,
        objects: Vec::new(),
        walked: Vec::new(),
        done: false
    };

    let mut guard_position: (i32, i32) = (0, 0); // So the compiler doesn't yell at me for an uninitialized var
    let mut guard_direction: (i32, i32) = (0, -1); // Starts facing up

    // Populate the map with "#" as objects
    for i in 0..map.width {
        for j in 0..map.height {
            if input[i as usize][j as usize] == '#' { // Use '' for char, "" for string!
                map.objects.push((i, j));
            }
            // Might as well look for the guard "^" while we're looping :)
            if input[i as usize][j as usize] == '^' {
                guard_position = (i, j);
            }
        }
    }

    // Have the little guy walk around :)
    // let mut counter = 0;
    while !map.done {
        (guard_position, guard_direction) = map.walk_until_object(guard_position, guard_direction);
        // let fname = format!("./logs/walk_{counter}.txt");
        // if let Err(e) = map.write_to_txt(fname) { // consumes fname but it's cool :)
        //     eprintln!("Failed to write to file: {}", e);
        // }
        // counter += 1;
    }

    let unique_walked: HashSet<(i32, i32)> = map.walked.into_iter().collect();
    println!("{}", unique_walked.len()); // Subtract one because we mark the last square the guy walks OUTSIDE the map :)
}

fn part2() {
    // This is so much repeating bad code!
    let input = read_input("input.txt");
    
    // Let's see if brute forcing works!
    let mut loop_positions: Vec<(i32, i32)> = Vec::new();
    for new_obs_x in 0..input.len() {
        println!("Row {}/{}", new_obs_x, input.len());
        for new_obs_y in 0..input[0].len() {

            // Make these i32 so we don't have to do it individaully!
            let new_obs_x: i32 = new_obs_x as i32;
            let new_obs_y: i32 = new_obs_y as i32;

            // println!("Row: {}, Col: {}", new_obs_x, new_obs_y);

            // Make a new grid again :)
            let mut map = Grid {
                width: input.len() as i32,
                height: input[0].len() as i32,
                objects: Vec::new(),
                walked: Vec::new(),
                done: false
            };
    
            let mut guard_position: (i32, i32) = (0, 0); // So the compiler doesn't yell at me for an uninitialized var
            let mut guard_direction: (i32, i32) = (0, -1); // Starts facing up
    
            // Use this to figure out if we're stuck in a loop (we've been in this position and direction before!)
            let mut previous_pos_dir: Vec<((i32, i32), (i32, i32))> = Vec::new();
    
            // Populate the map with "#" as objects
            for i in 0..map.width {
                for j in 0..map.height {
                    if input[i as usize][j as usize] == '#' { // Use '' for char, "" for string!
                        map.objects.push((i, j));
                    }
                    // Might as well look for the guard "^" while we're looping :)
                    if input[i as usize][j as usize] == '^' {
                        guard_position = (i, j);
                    }
                }
            }

            // Try to add a new obstruction here. BUT IF THERE'S ALREADY ONE, SKIP :)
            if map.objects.contains(&(new_obs_x, new_obs_y)) {
                continue
            } else {
                map.objects.push((new_obs_x, new_obs_y));
            }

            // Also skip if this is the guard's initial position... (we're not allowed to...)
            // I failed the first time bc of this!
            if (new_obs_x, new_obs_y) == guard_position {
                continue
            }
    
            // Have the little guy walk around :) but this time with loop detection!
            while !map.done {
                (guard_position, guard_direction) = map.walk_until_object(guard_position, guard_direction);

                if previous_pos_dir.contains(&(guard_position, guard_direction)) {
                    loop_positions.push((new_obs_x, new_obs_y));
                    map.done = true;
                    // println!("Obstruction found at ({}, {})", new_obs_x, new_obs_y);
                }

                // This won't add the initial guard position / direction, but that's what we want.
                previous_pos_dir.push((guard_position, guard_direction));
    
            }
        }
    }
    println!("{}", loop_positions.len());
}

fn main() {
    part1();
    part2();
}
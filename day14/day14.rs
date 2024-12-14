use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use regex::Regex;

fn read_input(filename: &str) -> Vec<String> {
    let mut input = Vec::new();
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    for line in contents.lines() {
        input.push(line.to_string());
    }
    return input;
}

#[derive(Clone)]
struct Room {
    bounds: (i32, i32),
    robots: Vec<Robot>
}

#[derive(Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
    bounds: (i32, i32)
}

impl Room {
    fn add_robot(&mut self, line: &String) {
        let re = Regex::new(r"-?[0-9]\d*(\.\d+)?").unwrap();
        let mut captures: Vec<&str> = Vec::new();
        for m in re.find_iter(&line) {
            captures.push(m.as_str());
        }
        let robot = Robot {
            position: (captures[0].parse::<i32>().unwrap(), captures[1].parse::<i32>().unwrap()),
            velocity: (captures[2].parse::<i32>().unwrap(), captures[3].parse::<i32>().unwrap()),
            bounds: self.bounds.clone()
        };
        self.robots.push(robot);
    }

    fn update(&mut self) {
        for robot in &mut self.robots {
            robot.move_robot();
        }
    }

    fn get_safety_factor(self) -> u32 {
        let mut quadrant_counts: Vec<u32> = vec![0, 0, 0, 0];
        let x_threshold: i32 = self.bounds.0 / 2_i32; // Integer truncation is what we want!
        let y_threshold: i32 = self.bounds.1 / 2_i32;
        for robot in self.robots {
            // Doesn't matter which quadrant is which as long as they're all different!
            if robot.position.0 < x_threshold {
                if robot.position.1 < y_threshold {
                    quadrant_counts[0] += 1;
                } else if robot.position.1 > y_threshold { // Can't use else! position = threshold is nothing :)
                    quadrant_counts[1] += 1;
                }
            } else if robot.position.0 > x_threshold {
                if robot.position.1 < y_threshold {
                    quadrant_counts[2] += 1;
                } else if robot.position.1 > y_threshold {
                    quadrant_counts[3] += 1;
                }
            }
        }
        return quadrant_counts[0] * quadrant_counts[1] * quadrant_counts[2] * quadrant_counts[3]
    }

    // Returns number of robots in a square area of side length
    // Good heuristic, I guess? Didn't work...
    fn robot_density(&self, length: i32) -> i32 {
        let robot_positions: Vec<(i32, i32)> = self.robots.iter().map(|robot| robot.position).collect();
        let mut max_density: i32 = 0;
        // This won't tile perfectly, but it's fine! length=10 should get most of the map:)
        for row in 0..self.bounds.1/length {
            for col in 0..self.bounds.0/length {
                // x is for cols and y is for rows!
                let this_density: i32 = robot_positions
                    .clone()
                    .into_iter()
                    .filter(|(x, y)| x>=&row && x<&(row+length) && y>=&col && y<&(col+length))
                    .collect::<Vec<(i32, i32)>>()
                    .len() as i32;
                if this_density > max_density {
                    max_density = this_density;
                }
            }
        }
        return max_density
    }

    fn write_to_txt(&self, fname: String) -> io::Result<()> {
        let file = File::create(fname)?;
        let mut writer = BufWriter::new(file);
        for row in 0..self.bounds.1 { // Row is Y!
            let mut line: Vec<char> = Vec::new();
            let robots: Vec<(i32, i32)> = self.robots.iter().filter(|&robot| robot.position.1 == row).map(|robot| robot.position).collect();
            let robots_x: Vec<i32> = robots.into_iter().map(|(x, _)| x).collect();
            for col in 0..self.bounds.0 {
                if robots_x.contains(&col) {
                    line.push('#'); // No way the number of robots matters!
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

impl Robot {
    fn move_robot(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        // Positions are zero indexed (bounds aren't!)
        if self.position.0 < 0 {
            self.position.0 += self.bounds.0; 
        } else if self.position.0 >= self.bounds.0 {
            self.position.0 -= self.bounds.0; 
        } 

        if self.position.1 < 0 {
            self.position.1 += self.bounds.1; 
        } else if self.position.1 >= self.bounds.1 {
            self.position.1 -= self.bounds.1; 
        } 
    }
}

fn part1(mut room: Room) {
    for _ in 0..100 {
        room.update();
    }
    println!("{}", room.get_safety_factor());
}

fn part2(mut room: Room) {
    // I see horizontal bands at seconds 484 381 278 175 72 (every 93)
    // I see a vertical band at seconds 406 305 204 103 2 (every 101)
    for i in 0..=10000 {
        // Robot density didn't work...
        // let robot_density = room.robot_density(5);
        // println!("{}: {}", i, &robot_density);
        // if robot_density > 10 { // Better than printing every room! 
        //     let fname = format!("./logs/second_{i}.txt");
        //     if let Err(e) = room.write_to_txt(fname) {
        //         eprintln!("Failed to write to file: {}", e);
        //     }
        // }

        // I could do math to figure out when the patterns line up, but I'll just print every 101 :)
        // Sick, it worked. I saw it!
        if i%101 == 2 {
            let fname = format!("./logs/second_{i}.txt");
            if let Err(e) = room.write_to_txt(fname) {
                eprintln!("Failed to write to file: {}", e);
            }
        }
        
        room.update();
    }
}

fn main() {
    let input: Vec<String> = read_input("input.txt");
    let mut room = Room {
        bounds: (101, 103),
        robots: Vec::new()
    };
    for line in input {
        room.add_robot(&line);
    }
    part1(room.clone());
    part2(room.clone());
}
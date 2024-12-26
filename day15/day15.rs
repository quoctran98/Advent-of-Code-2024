use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};

fn read_input(filename: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let contents  = fs::read_to_string(filename).expect("reading file");
    let mut room: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<char> = Vec::new();
    let mut instr_flag: bool = false;
    for line in contents.lines() {
        if line.is_empty() {
            instr_flag = true;
            continue
        }
        if instr_flag {
            instructions.extend(line.chars().collect::<Vec<char>>());
        } else {
            room.push(line.chars().collect::<Vec<char>>());
        }
    }
    return (room, instructions);
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
enum Thing {
    Robot,
    BoxLeft,
    BoxRight,
    Air,
    Wall
}

#[derive(Clone)]
struct Warehouse {
    map: Vec<Vec<Thing>>,
    bounds: (i32, i32),
    robot: (i32, i32)
}

impl Warehouse {
    fn new(room: Vec<Vec<char>>) -> Warehouse {
        let mut warehouse = Warehouse {
            map: Vec::new(),
            bounds: (room.len() as i32, room[0].len() as i32 * 2),
            robot: (-1, -1)
        };
        for row in 0..room.len() {
            let mut new_row: Vec<Thing> = Vec::new();
            for col in 0..room.len() {
                // Ugh, it's the row/col y/x thing again... but it won't matter :)
                let thing_char: char = room[row][col];
                if thing_char == '#' {
                    new_row.push(Thing::Wall);
                    new_row.push(Thing::Wall);
                } else if thing_char == '.' {
                    new_row.push(Thing::Air);
                    new_row.push(Thing::Air);
                } else if thing_char == '@' {
                    new_row.push(Thing::Robot);
                    new_row.push(Thing::Air);
                    warehouse.robot = (row as i32, (col as i32) * 2);
                } else if thing_char == 'O' {
                    new_row.push(Thing::BoxLeft);
                    new_row.push(Thing::BoxRight);
                } else {
                    panic!("thing not recognized");
                }
            }
            assert!(new_row.len() == warehouse.bounds.1 as usize);
            warehouse.map.push(new_row)
        }
        assert!(warehouse.map.len() == warehouse.bounds.0 as usize);
        return warehouse
    }

    fn swap_things(&mut self, source: (i32, i32), destination: (i32, i32)) {
        let src_thing: Thing =  self.map[source.0 as usize][source.1 as usize].clone();
        dbg!(&src_thing);
        let dest_thing: Thing = self.map[destination.0 as usize][destination.1 as usize].clone();
        dbg!(&dest_thing);
        self.map[destination.0 as usize][destination.1 as usize] = src_thing.clone();
        self.map[source.0 as usize][source.1 as usize] = dest_thing;
        if src_thing == Thing::Robot {
            self.robot = destination;
        }
    }

    fn attempt_move_horiz(&mut self, source: (i32, i32), direction: (i32, i32)) -> bool {
        assert!((direction.0 * direction.1 == 0) && ((direction.0 + direction.1).abs() == 1));
        let mut other_source: Option<(i32, i32)> = None; // For moving other half of the box
        let destination: (i32, i32) = (source.0 + direction.0, source.1 + direction.1);
        let mut other_destination: Option<(i32, i32)> = None; // For moving other half of the box
        let thing: Thing = self.map[destination.0 as usize][destination.1 as usize].clone();

        // If we're moving left/right, the logic is the same!
        let can_move;
        if thing == Thing::Air { // We can move if it's air
            can_move = true;
        } else if thing == Thing::Wall { // Can't move if it's a wall
            can_move = false;
        } else if thing == Thing::BoxLeft ||  thing == Thing::BoxRight { // If it's a box, propogate the movement
            can_move = self.attempt_move(destination, direction); // Have destination thing attempt move!
        } else {
            panic!("bad movement destination");
        }
        // If we can, then swap!
        if can_move {
            assert!(self.map[destination.0 as usize][destination.1 as usize] == Thing::Air); // It should be air now!
            self.swap_things(source, destination);
            return true
        } else {
            return false
        }
    }

    fn attempt_move_vert(&mut self, source: (i32, i32), direction: (i32, i32), queued_swaps: Vec<((i32, i32), (i32, i32))>) -> bool {
        assert!((direction.0 * direction.1 == 0) && ((direction.0 + direction.1).abs() == 1));
        let mut other_source: Option<(i32, i32)> = None; // For moving other half of the box
        let destination: (i32, i32) = (source.0 + direction.0, source.1 + direction.1);
        let mut other_destination: Option<(i32, i32)> = None; // For moving other half of the box
        let thing: Thing = self.map[destination.0 as usize][destination.1 as usize].clone();

        // I have to think about the other case...
        // We can still use the same recursion for the most part, I think?
        // Not really... We can't do any swaps until ALL return true, not the two in this branch...
        let can_move;
        if thing == Thing::Air { // Air is the same
            can_move = true;
        } else if thing == Thing::Wall { // Wall is the same
            can_move = false;
        } else if thing == Thing::BoxLeft || thing == Thing::BoxRight { // Add the opposite box and propogate movement!
            if thing == Thing::BoxLeft {
                // Add the RIGHT box and spot RIGHT of the robot to the swap. Maybe? If it's air?
                other_destination = Some((destination.0, destination.1 + 1));
                other_source = Some((source.0, source.1 + 1));
                assert!(self.map[other_destination.unwrap().0 as usize][other_destination.unwrap().1 as usize] == Thing::BoxRight);
            } else {
                // Add the LEFT box and spot LEFT of the robot to the swap. Maybe? If it's air?
                other_destination = Some((destination.0, destination.1 - 1));
                other_source = Some((source.0, source.1 - 1));
                assert!(self.map[other_destination.unwrap().0 as usize][other_destination.unwrap().1 as usize] == Thing::BoxLeft);
            }
            // Have both destination things attempt move!
            let can_move_left: bool = self.attempt_move(destination, direction);
            let can_move_right: bool = self.attempt_move(other_destination.unwrap(), direction);
            can_move = can_move_left && can_move_right;
        } else {
            panic!("bad movement destination");
        }
        // If we can, then swap!
        if can_move {
            assert!(self.map[destination.0 as usize][destination.1 as usize] == Thing::Air); // It should be air now!
            self.swap_things(source, destination);
            // Also swap other_source and other_destination they're declared
            match other_destination {
                Some(_) => {
                    // But let's make sure that we're not swapping in the case of [] pushing []
                    if self.map[source.0 as usize][source.1 as usize] != self.map[destination.0 as usize][destination.1 as usize] {
                        assert!(self.map[other_destination.unwrap().0 as usize][other_destination.unwrap().1 as usize] == Thing::Air); // It should be air now!
                        self.swap_things(other_source.unwrap(), other_destination.unwrap());
                    }
                },
                None => {}
            }
            return true
        } else {
            return false
        }
    }

    fn execute(&mut self, instructions: &Vec<char>) {
        for (i, instr) in instructions.into_iter().enumerate() {
            dbg!(&i);
            let fname = format!("./logs/{i}_{instr}.txt");
            if let Err(e) = self.write_to_txt(fname) {
                eprintln!("Failed to write to file: {}", e);
            }
            if instr == &'^' {
                dbg!(self.attempt_move_vert(self.robot, (-1, 0), Vec::new()));
            } else if instr == &'v' {
                dbg!(self.attempt_move_vert(self.robot, (1, 0), Vec::new()));
            } else if instr == &'>' {
                dbg!(self.attempt_move_horiz(self.robot, (0, 1)));
            } else if instr == &'<' {
                dbg!(self.attempt_move_horiz(self.robot, (0, -1)));
            }
        }
        let fname = format!("./logs/end.txt");
        if let Err(e) = self.write_to_txt(fname) {
            eprintln!("Failed to write to file: {}", e);
        }
    }

    fn sum_gps(&self) -> i32 {
        let mut total: i32 = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                if self.map[row][col] == Thing::BoxLeft || self.map[row][col] == Thing::BoxRight {
                    total += (100 * row as i32) + col as i32;
                }
            }
        }
        return total
    }

    fn write_to_txt(&self, fname: String) -> io::Result<()> {
        let file = File::create(fname)?;
        let mut writer = BufWriter::new(file);
        for row in 0..self.map.len() { // Row is Y!
            let mut line: Vec<char> = Vec::new();
            for col in 0..self.map[0].len() {
                let thing = &self.map[row][col];
                if thing == &Thing::Air {
                    line.push(' ');
                } else if thing == &Thing::Wall {
                    line.push('#');
                } else if thing == &Thing::BoxLeft {
                    line.push('[');
                } else if thing == &Thing::BoxRight {
                    line.push(']');
                } else if thing == &Thing::Robot {
                    line.push('@');
                } else {
                    panic!("unknown thing");
                }
            }
            // Write this line to the wrtier buffer
            writeln!(writer, "{}", line.into_iter().collect::<String>())?;
        }
        writer.flush()?;
        Ok(())
    }
}

fn part2(mut warehouse: Warehouse, instructions: &Vec<char>) {
    warehouse.execute(instructions); // Since we do &mut self, it doesn't consume it!
    println!("{}", warehouse.sum_gps());
}

fn main() {
    let (room, instructions) = read_input("input.txt");
    let warehouse = Warehouse::new(room);
    part2(warehouse.clone(), &instructions);
    
}
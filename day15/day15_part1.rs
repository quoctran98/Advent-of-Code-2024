use std::fs;

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
enum Thing {
    Robot,
    Box,
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
            bounds: (room.len() as i32, room[0].len() as i32),
            robot: (-1, -1)
        };
        for row in 0..room.len() {
            let mut new_row: Vec<Thing> = Vec::new();
            for col in 0..room.len() {
                // Ugh, it's the row/col y/x thing again... but it won't matter :)
                let thing_char: char = room[row][col];
                if thing_char == '#' {
                    new_row.push(Thing::Wall);
                } else if thing_char == '.' {
                    new_row.push(Thing::Air);
                } else if thing_char == '@' {
                    new_row.push(Thing::Robot);
                    warehouse.robot = (row as i32, col as i32);
                } else if thing_char == 'O' {
                    new_row.push(Thing::Box);
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
        let dest_thing: Thing = self.map[destination.0 as usize][destination.1 as usize].clone();
        self.map[destination.0 as usize][destination.1 as usize] = src_thing.clone();
        self.map[source.0 as usize][source.1 as usize] = dest_thing;
        if src_thing == Thing::Robot {
            self.robot = destination;
        }
    }

    fn attempt_move(&mut self, source: (i32, i32), direction: (i32, i32)) -> bool {
        assert!((direction.0 * direction.1 == 0) && ((direction.0 + direction.1).abs() == 1));
        let destination: (i32, i32) = (source.0 + direction.0, source.1 + direction.1);
        let thing: Thing = self.map[destination.0 as usize][destination.1 as usize].clone();

        // Recursively check if we can move
        let can_move;
        if thing == Thing::Air { // We can move if it's air
            can_move = true;
        } else if thing == Thing::Wall { // Can't move if it's a wall
            can_move = false;
        } else if thing == Thing::Box { // If it's a box, propogate the movement
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

    fn execute(&mut self, instructions: &Vec<char>) {
        for instr in instructions {
            if instr == &'^' {
                self.attempt_move(self.robot, (-1, 0));
            } else if instr == &'v' {
                self.attempt_move(self.robot, (1, 0));
            } else if instr == &'>' {
                self.attempt_move(self.robot, (0, 1));
            } else if instr == &'<' {
                self.attempt_move(self.robot, (0, -1));
            }
        }
    }

    fn sum_gps(&self) -> i32 {
        let mut total: i32 = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                if self.map[row][col] == Thing::Box {
                    total += (100 * row as i32) + col as i32;
                }
            }
        }
        return total
    }
}

fn part1(mut warehouse: Warehouse, instructions: &Vec<char>) {
    warehouse.execute(instructions); // Since we do &mut self, it doesn't consume it!
    println!("{}", warehouse.sum_gps());
}

fn main() {
    let (room, instructions) = read_input("input.txt");
    let warehouse = Warehouse::new(room);
    part1(warehouse.clone(), &instructions);
    
}
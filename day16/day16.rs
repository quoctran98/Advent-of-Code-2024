use std::fs;
use std::collections::HashSet;
use std::cmp::min;

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let contents  = fs::read_to_string(filename).expect("reading file");
    let mut output: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        output.push(line.chars().collect::<Vec<char>>());
    }
    return output
}

struct Maze {
    start: (i32, i32),
    end: (i32, i32),
    direction: (i32, i32),
    spaces: HashSet<(i32, i32)>
}

impl Maze {
    fn new(input: &Vec<Vec<char>>) -> Maze {
        let mut maze = Maze {
            start: (-1, -1),
            end: (-1, -1),
            direction: (0, 1),
            spaces: HashSet::new()
        };
        for r in 0..input.len() {
            for c in 0..input[0].len() {
                let this_char: char = input[r][c];
                if this_char == '.' {
                    maze.spaces.insert((r as i32, c as i32));
                } else if this_char == 'S' {
                    assert!(maze.start == (-1, -1));
                    maze.spaces.insert((r as i32, c as i32));
                    maze.start = (r as i32, c as i32);
                } else if this_char == 'E' {
                    assert!(maze.end == (-1, -1));
                    maze.spaces.insert((r as i32, c as i32));
                    maze.end = (r as i32, c as i32);
                }
            }
        }
        assert!(maze.start != (-1, -1));
        assert!(maze.end != (-1, -1));
        return maze
    }

    fn find_all_paths(&self, path: Vec<(i32, i32)>) -> Vec<Vec<(i32, i32)>> {
        let mut done_paths: Vec<Vec<(i32, i32)>> = Vec::new();
        let last: (i32, i32) = path[path.len()-1];
        for dir in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            // Move one step and see if we can go there
            let next: (i32, i32) = (last.0 + dir.0, last.1 + dir.1);
            if self.spaces.contains(&next) && !path.contains(&next) {
                // Add the next step to this path!
                let mut new_path: Vec<(i32, i32)> = path.clone();
                new_path.push(next);
                if self.end == next { 
                    // If we reach the end the new path, but don't return yet
                    dbg!(&new_path.len());
                    done_paths.push(new_path);
                } else {
                    // Otherwise, keep extending this path!
                    done_paths.extend(self.find_all_paths(new_path));
                }
            }
        }
        return done_paths
    }

    fn find_one_path(&self, path: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let last: (i32, i32) = path[path.len()-1];
        for dir in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            // Move one step and see if we can go there
            let next: (i32, i32) = (last.0 + dir.0, last.1 + dir.1);
            if self.spaces.contains(&next) && !path.contains(&next) {
                // Add the next step to this path!
                let mut new_path: Vec<(i32, i32)> = path.clone();
                new_path.push(next);
                if self.end == next { 
                    // If we reach the end the new path, but don't return yet
                    dbg!(&new_path.len());
                    done_paths.push(new_path);
                } else {
                    // Otherwise, keep extending this path!
                    done_paths.extend(self.find_all_paths(new_path));
                }
            }
        }
        return done_paths
    }

    fn score_path(&self, path: Vec<(i32, i32)>) -> i32 {
        assert!(&path[0] == &self.start);
        let turn_vector: Vec<(i32, i32)> = vec![(0, 1), (-1, 0), (0, -1), (1, 0)];
        let mut score: i32 = 0;
        let mut dir: (i32, i32) = self.direction.clone();
        let mut space: (i32, i32) = path[0].clone();
        for i in 1..path.len() {
            let step: (i32, i32) = path[i];
            let mut turn_found: bool = false;
            for turn in &turn_vector {
                // Let's look for the turn we took!
                if (space.0 + turn.0, space.1 + turn.1) == step {
                    // Add 1000 points per turn turn!
                    let dir_idx: i32 = turn_vector.iter().position(|t| *t == dir.clone()).unwrap() as i32;
                    let turn_idx: i32 = turn_vector.iter().position(|t| *t == turn.clone()).unwrap() as i32;
                    let n_turns: i32 = min((dir_idx - turn_idx).abs(), 4 - (dir_idx - turn_idx).abs());
                    score += 1000 * n_turns;
                    // Do housekeeping stuff!
                    score += 1; // 1 point for going forward
                    dir = turn.clone();
                    space = step.clone();
                    turn_found = true;
                    break
                }
            }
            assert!(turn_found);
        }
        return score
    }
}

fn main() {
    let input: Vec<Vec<char>> = read_input("input.txt");
    let maze: Maze = Maze::new(&input);
    let paths: Vec<Vec<(i32, i32)>> = maze.find_all_paths(vec![maze.start.clone()]);
    let scores: Vec<i32> = paths.into_iter().map(|p| maze.score_path(p)).collect();
    println!("{}", scores.into_iter().min().unwrap());
}
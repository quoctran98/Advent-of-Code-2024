use std::fs;
use regex::Regex;

fn read_input(filename: &str) -> Vec<String> {
    let mut input = Vec::new();
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    for line in contents.lines() {
        input.push(line.to_string());
    }
    return input;
}

fn extract_numbers(input: String) -> (u64, u64) {
    let re = Regex::new(r"X[+=](?<x>\d+), Y[+=](?<y>\d+)").unwrap();
    let captures = re.captures(&input).unwrap();
    return (captures["x"].parse::<u64>().unwrap(), captures["y"].parse::<u64>().unwrap());
}

struct ClawMachine {
    a: (u64, u64), // 3 token cost
    b: (u64, u64), // 1 token cost
    prize: (u64, u64)
}

impl ClawMachine {
    fn new(a_string: &String, b_string: &String, prize_string: &String) -> ClawMachine {
        ClawMachine {
            a: extract_numbers(a_string.to_string()),
            b: extract_numbers(b_string.to_string()),
            prize: extract_numbers(prize_string.to_string())
        }
    }

    // Part 1 gives us a hint that there's a maximum mumber of presses
    // No need to do lin alg... yet, I assume...
    fn find_solutions_naively(self, max_presses: u64) -> Vec<(u64, u64)> {
        let mut solutions: Vec<(u64, u64)> = Vec::new();
        'a_loop: for a_presses in 0..=max_presses {
            'b_loop: for b_presses in 0..=max_presses {
                let location: (u64, u64) = (a_presses*self.a.0 + b_presses*self.b.0, a_presses*self.a.1 + b_presses*self.b.1);
                if location == self.prize {
                    solutions.push((a_presses as u64, b_presses as u64));
                    break 'b_loop // No reason to keep increasing b
                } else if (location.0 > self.prize.0) || (location.1 > self.prize.1) {
                    if b_presses == 0 {
                        break 'a_loop // a has already gotten too large!
                    } else {
                        break 'b_loop // b has gotten too large!
                    }
                }
            }
        }
        return solutions
    }

    fn find_solutions_with_math(self) -> Vec<(u64, u64)> {
        // I didn't even do linear algebra since it's a 2x2 matrix. I just set up a system of equations.
        // But how do I know that there's only one solution? I determine linear dependence?
        // But why does doing a system of equations return one unambiguous answer?
        let b_presses_numerator: i64 = ((self.a.0 * self.prize.1) as i64) - ((self.a.1 * self.prize.0) as i64);
        let b_presses_denominator: i64 = ((self.a.0 * self.b.1) as i64) - ((self.a.1 * self.b.0) as i64);
        let b_presses_float: f64 = (b_presses_numerator as f64)/(b_presses_denominator as f64);
        let b_presses: u64;
        if b_presses_float.fract() == 0.0 {
            b_presses = b_presses_float as u64;
        } else { // No integer solutions found!
            return vec![]
        }

        let a_presses_numerator: i64 = (self.prize.0 as i64) - ((self.b.0 * b_presses) as i64);
        let a_presses_float: f64 = (a_presses_numerator as f64)/(self.a.0 as f64);
        let a_presses: u64;
        if a_presses_float.fract() == 0.0 {
            a_presses = a_presses_float as u64;
        } else { // No integer solutions found!
            return vec![]
        }

        return vec![(a_presses, b_presses)]

    }
}

fn part1(input: &Vec<String>) {
    let mut tokens: u64 = 0;
    for i in 0..input.len()/4 {
        let line_number: usize = i*4;
        let a_string = &input[line_number];
        let b_string = &input[line_number+1];
        let prize_string = &input[line_number+2];
        let machine = ClawMachine::new(a_string, b_string, prize_string);
        let solutions = machine.find_solutions_naively(100);
        if !solutions.is_empty() {
            let mut lowest_cost: u64 = (solutions[0].0 * 3) + solutions[0].1;
            for s in solutions {
                let cost: u64 = (s.0*3) + s.1;
                if cost < lowest_cost {
                    lowest_cost = cost;
                }
            }
            tokens += lowest_cost;
        }
    }
    println!("{}", tokens);
}

// THIS IS SO WEIRD!
// For some reason, part 2 fails to parse the full input (misses the last claw machine)
// I didn't feel like debugging, so I just added a newline to the file.
// Part 1 parses it fine? Oh wait, it probably doesn't, but in part 1 that last machine probably had no soln.
fn part2(input: &Vec<String>) {
    let mut tokens: u64 = 0;
    for i in 0..input.len()/4 {
        let line_number: usize = i*4;
        let a_string = &input[line_number];
        let b_string = &input[line_number+1];
        let prize_string = &input[line_number+2];
        let mut machine = ClawMachine::new(a_string, b_string, prize_string);
        // Add the conversion error here!
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
        dbg!(machine.prize);
        let solutions = machine.find_solutions_with_math();
        if !solutions.is_empty() {
            let mut lowest_cost: u64 = (solutions[0].0 * 3) + solutions[0].1;
            for s in solutions {
                let cost: u64 = (s.0*3) + s.1;
                if cost < lowest_cost {
                    lowest_cost = cost;
                }
            }
            tokens += lowest_cost;
        }
    }
    println!("{}", tokens);
}

fn main() {
    let input: Vec<String> = read_input("input.txt");
    part1(&input);
    part2(&input);
}
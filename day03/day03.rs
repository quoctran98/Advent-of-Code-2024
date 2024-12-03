use std::fs;
use regex::Regex;

fn read_input(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    return contents
}

fn part1() {
    let input = read_input("input.txt");
    // Match mul(n,m), where n and m are 1-3 digit numbers
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let mut total = 0;
    for m in re.find_iter(&input) {
        let mul = &input[m.start()+4..m.end()-1];
        let numstr: Vec<&str> = mul.split(",").collect();
        let num1: i32 = numstr[0].parse().unwrap();
        let num2: i32 = numstr[1].parse().unwrap();
        total += num1 * num2;
    }
    println!("{}", total);
}

fn part2() {
    let input = read_input("input.txt");
    let re_mul = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    // Make a vector of instruction tuples (start location,instruction)
    let mut instructions: Vec<(i32,&str)> = Vec::new();
    for m in re_mul.find_iter(&input) {
        let mul = &input[m.start()+4..m.end()-1];
        instructions.push((m.start().try_into().unwrap(),mul));
    }
    for m in re_do.find_iter(&input) {
        instructions.push((m.start().try_into().unwrap(),"do"));
    }
    for m in re_dont.find_iter(&input) {
        instructions.push((m.start().try_into().unwrap(),"dont"));
    }

    // Sort and parse the instructions
    let mut total = 0;
    instructions.sort_by(|a,b| a.0.cmp(&b.0));
    let mut do_mul = true;
    for i in instructions {
        let instr = i.1;
        if instr == "do" {
            do_mul = true;
        } else if instr == "dont" {
            do_mul = false;
        } else {
            if do_mul {
                let numstr: Vec<&str> = instr.split(",").collect();
                let num1: i32 = numstr[0].parse().unwrap();
                let num2: i32 = numstr[1].parse().unwrap();
                total += num1 * num2;
            }
        }
    }
    println!("{}", total)
}


fn main() {
    part1();
    part2();
}
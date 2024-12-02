use std::fs;

fn read_input(filename: &str) -> Vec<String> {
    let mut input = Vec::new();
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    for line in contents.lines() {
        input.push(line.to_string());
    }
    return input;
}

fn line_to_vec(line: &str) -> Vec<i32> {
    let split_line: Vec<i32> = line
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();
    return split_line
}

fn check_safe(report: &Vec<i32>, min_change: i32, max_change: i32) -> bool {
    let is_increasing = report[1] > report[0];
    for i in 1..report.len() {
        let prev_value = report[i-1];
        let this_value = report[i];
        let val_diff = this_value - prev_value;
        let this_increasing = this_value > prev_value;
        if this_increasing != is_increasing {
            return false
        }
        if (val_diff.abs() < min_change) || (val_diff.abs() > max_change) {
            return false
        }
    }
    return true
}

fn iteratively_check_safe(report: &Vec<i32>, min_change: i32, max_change: i32) -> bool {
    // I love how bad this is!
    let mut is_safe = check_safe(&report, min_change, max_change);
    if !is_safe {
        for i in 0..report.len() {
            let mut sliced_report = report.clone();
            sliced_report.remove(i);
            is_safe = check_safe(&sliced_report, min_change, max_change);
            if is_safe {
                return true
            }
        }
        return false
    }
    return true
}

fn part1() {
    let input = read_input("input.txt");
    let mut n_safe = 0;
    for line in input {
        let report = line_to_vec(&line);
        n_safe += check_safe(&report, 1, 3) as i32;
    }
    println!("{}", n_safe);
}

fn part2() {
    let input = read_input("input.txt");
    let mut n_safe = 0;
    for line in input {
        let report = line_to_vec(&line);
        n_safe += iteratively_check_safe(&report, 1, 3) as i32;
    }
    println!("{}", n_safe);
}


fn main() {
    part1();
    part2();
}
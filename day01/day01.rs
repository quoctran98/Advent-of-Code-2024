use std::fs;

fn read_input(filename: &str) -> Vec<String> {
    let mut input = Vec::new();
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    for line in contents.lines() {
        input.push(line.to_string());
    }
    return input;
}

fn split_input(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input {
        let split_line: Vec<&str> = line.split("   ").collect();
        let val1: i32 = split_line[0].parse().unwrap();
        let val2 = split_line[1].parse::<i32>().unwrap();
        list1.push(val1);
        list2.push(val2);
    }
    return (list1, list2);
}

fn count_occurences(list: &Vec<i32>, value: &i32) -> i32 {
    let mut count = 0;
    for i in list {
        if i == value {
            count += 1;
        }
    }
    return count;
}


fn part1() {
    let input = read_input("input.txt");
    let (mut list1, mut list2) = split_input(input);
    list1.sort();
    list2.sort();
    let mut total_distance: i32 = 0;
    for i in 0..list1.len() {
        let distance = list1[i] - list2[i];
        total_distance += distance.abs();
    }
    println!("{}", total_distance);
}

fn part2() {
    let input = read_input("input.txt");
    let (list1, list2) = split_input(input);
    let mut similarity_score: i32 = 0;
    for i in 0..list1.len() {
        let value: i32 = list1[i];
        let ocurences: i32 = count_occurences(&list2, &value);
        similarity_score += ocurences * value;

    }
    println!("{}", similarity_score);
}

fn main() {
    part1();
    part2();
}
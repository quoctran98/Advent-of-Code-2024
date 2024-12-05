use std::fs;
use std::collections::HashSet;
// use itertools::Itertools;

fn read_input(filename: &str) -> Vec<String> {
    let mut input = Vec::new();
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    for line in contents.lines() {
        input.push(line.to_string());
    }
    return input;
}

fn split_line_to_int(line: &str, sep: &str) -> Vec<i32> {
    let split_line: Vec<i32> = line
        .split(sep)
        .map(|s| s.parse().unwrap())
        .collect();
    return split_line
}

// Takes in raw input lines then returns rules (vector of (i32, i32)) and updates (vector of vector of i32)
// Take ownership of raw input since we're just using this in a chain!
fn parse_input(input: Vec<String>) -> (Vec<Vec<i32>>,Vec<Vec<i32>>) {
    let mut rules: Vec<Vec<i32>> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    for line in input.iter() { // Takes ownership of input (I think?) doesn't matter regardless.
        if line.contains("|") {
            // Process line as rule
            rules.push(split_line_to_int(&line, &String::from("|")));
        } else if line.contains(",") {
            // Process line as update
            updates.push(split_line_to_int(&line, &String::from(",")));
        }
    }
    return (rules, updates)
}

// THIS MIGHT JUST NOT BE POSSIBLE!! SOME PAGES MIGHT JUST NEVER COEXIST IN THE SAME UPDATE!
fn combine_rules(rules: Vec<Vec<i32>>) -> Vec<i32> {
    let unique_pages: HashSet<i32> = rules.clone().into_iter()
                                          .flat_map(|v| v.into_iter())
                                          .collect();
    let mut all_possible_orderings: Vec<Vec<i32>> = vec![Vec::new()];
    // Go through each unique page to see where we can add it.
    for (npages, page) in unique_pages.iter().enumerate() {
        // Maybe this will be fast enough.. brute force adding pages!
        // This got more complicated. Just because an insertion is allowed, doesn't mean it'll work in the future too!
        // I have another algorithm! Go through each possible ordering
        let mut new_orderings: Vec<Vec<i32>> = Vec::new();
        for ordering in all_possible_orderings.iter() {
            // Find a spot for this in that ordering
            for idx in 0..ordering.len()+1 {
                let mut temp_pages = ordering.clone();
                temp_pages.insert(idx, page.clone());
                // If possible, clone the ordering, then insert the page into this ordering
                // DO THIS FOR ALL POSSIBLE INSERTIONS! DON'T BREAK EARLY AS BEFORE!
                if check_update(&temp_pages, &rules) {
                    let mut new_ordering = ordering.clone();
                    new_ordering.insert(idx, page.clone());
                    new_orderings.push(new_ordering); // Collect new orderings separately
                }
            }
        }
        // Extend the original vector with the new orderings
        all_possible_orderings.extend(new_orderings);
        // Remove orderings that have less than npages
        all_possible_orderings.retain(|ordering| ordering.len() > npages);
    }
    return all_possible_orderings[0].clone();
}

// I'm implementing this so that I can use my cool combine_rules() func
fn filter_rules(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut filtered_rules: Vec<Vec<i32>> = Vec::new();
    for rule in rules.iter() {
        if update.contains(&rule[0]) && update.contains(&rule[1]) {
            filtered_rules.push(rule.clone());
        }
    }
    return filtered_rules
}

fn search_rules(p1: &i32, p2: &i32, rules: &Vec<Vec<i32>>) -> Option<Vec<i32>> {
    for rule in rules.iter() {
        if rule.contains(&p1) && rule.contains(&p2) {
            return Some(rule.clone())
        }
    }
    return None
}

fn check_update(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    for (i, this_page) in update.iter().enumerate() {
        for (j, other_page) in update.iter().enumerate() {
            if i != j {
                match search_rules(&this_page, &other_page, rules) {
                    Some(rule) => {
                        if (i<j) != (rule[0]==*this_page) {
                            return false
                        }
                    }
                    None => {}
                }
            }
        }
    }
    return true
}

fn part1() {
    let (rules, updates) = parse_input(read_input("input.txt"));
    let mut total = 0;
    for update in updates.iter() {
        let middle_value = &update[(update.len()-1)/2..((update.len()-1)/2)+1][0];
        total += (check_update(update, &rules) as i32) * *middle_value;
    }
    println!("{}", total);
}

fn part2() {
    let (rules, updates) = parse_input(read_input("input.txt"));
    let mut total = 0;
    for update in updates.iter() {
        if !check_update(update, &rules) { // Only do this for things that initially fail!
            let combined_filtered_rules = combine_rules(filter_rules(&update, &rules));
            if combined_filtered_rules.len() != update.len() {
                // No need to even sort based on this list! This filtered rulset IS the correct update :)
                // I didn't even think about this :) It's almost elegant!
                panic!()
            }
            let new_update = combined_filtered_rules;
            total += &new_update[(new_update.len()-1)/2..((new_update.len()-1)/2)+1][0];
        }
    }
    println!("{}", total);
}

fn main() {
    part1();
    part2();
}
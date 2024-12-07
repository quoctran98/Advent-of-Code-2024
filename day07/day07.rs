use std::fs;

fn read_input(filename: &str) -> Vec<(i64, Vec<i64>)> {
    let mut input = Vec::new();
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    for line in contents.lines() {
        let line_vec: Vec<&str> = line.split(": ").collect();   
        let total: i64 = line_vec[0].parse().unwrap();
        let values: Vec<i64> = line_vec[1].split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        input.push((total,values))
    }
    return input;
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate
}

// The itertools version was too confusing...
fn cartesian_product<T: Clone>(set: Vec<T>, n: usize) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = vec![Vec::new()];
    for _ in 0..n {
        let mut new_result = Vec::new();
        for item in &set {
            for r in &result {
                let mut new_r: Vec<T> = r.clone();
                new_r.push(item.clone());
                new_result.push(new_r);
            }
        }
        result = new_result;
    }
    return result
}

fn evaluate(values: Vec<i64>, operators: Vec<Operator>) -> i64 {
    assert!(values.len() - operators.len() == 1);
    let mut total = values[0];
    for (i, op) in operators.iter().enumerate() {
        if *op == Operator::Add {
            total += values[i+1];
        }
        if *op == Operator::Multiply {
            total *= values[i+1];
        }
        if *op == Operator::Concatenate {
            let temp_total = total.to_string();
            let temp_value = values[i+1].to_string();
            total = format!("{}{}", temp_total, temp_value).parse().expect("Something went wrong when concatenating!");
        }
    }
    return total
}

fn part1(input: &Vec<(i64, Vec<i64>)> ) {
    let mut answer = 0;
    for (i, eq) in input.iter().enumerate() {
        let total = eq.0.clone();
        let values = eq.1.clone();

        // Quick heuristic with the bounds of the calculation
        let min = evaluate(values.clone(), vec![Operator::Add; values.len() - 1]);
        let max = evaluate(values.clone(), vec![Operator::Multiply; values.len() - 1]);
        if (min > total) && (max < total) {
            continue
        }
        
        // But beyond this, I'll just brute force?
        // I truly think that there's a way to do this calculation iteratively 
        // and guarantee that the total slowly increases. But I'll have to think more :)
        let operators = vec![Operator::Add, Operator::Multiply];
        let op_combinations = cartesian_product(operators, values.len() - 1);
        for op in op_combinations.iter() {
            if total == evaluate(values.clone(), op.to_vec()) {
                answer += total;
                break
            }
        }
    }
    println!("{}", answer); // It's surprisingly fast! I forget how good computers are at arithmetic sometimes.
}


fn part2(input: &Vec<(i64, Vec<i64>)> ) {
    let mut answer = 0;
    for (i, eq) in input.iter().enumerate() {
        println!("{}/{}", i+1, input.len());
        let total = eq.0.clone();
        let values = eq.1.clone();

        // The heuristic no longer works here!
        
        let operators = vec![Operator::Add, Operator::Multiply, Operator::Concatenate];
        let op_combinations = cartesian_product(operators, values.len() - 1);
        for op in op_combinations.iter() {
            if total == evaluate(values.clone(), op.to_vec()) {
                answer += total;
                break
            }
        }
    }
    println!("{}", answer);
}

fn main() {
    let input = read_input("input.txt");
    part1(&input);
    part2(&input);
}
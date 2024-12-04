use std::fs;

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    for line in contents.lines() {
        let char_array = line.chars().collect();
        output.push(char_array);
    }
    return output
}

fn find_word(row: usize, col: usize, wordsearch: &Vec<Vec<char>>, direction: (i32, i32)) -> bool {
    let word = "XMAS";
    // Inelegant but easy.
    if ((direction.0 == -1) && (row < 3)) || ((direction.0 == 1) && (row > wordsearch.len() - 4)) {
        return false
    }
    if ((direction.1 == -1) && (col < 3)) || ((direction.1 == 1) && (col > wordsearch[0].len() - 4)) {
        return false
    }
    for i in 1..4 { // No need to start at X
        let next_row = (row as i32 + direction.0 * i) as usize;
        let next_col = (col as i32 + direction.1 * i) as usize;
        let next_letter = word[i as usize..(i + 1) as usize].chars().next().unwrap();        
        if &wordsearch[next_row][next_col] != &next_letter {
            return false
        }
    }
    return true
}

fn find_xmas(row: usize, col: usize, wordsearch: &Vec<Vec<char>>) -> bool {
    // Make sure the starting A isn't at any edge!
    if row < 1 || col < 1 || row > wordsearch.len()-2 || col > wordsearch[0].len()-2 {
        return false
    }
    // Inelegant but very readable :)
    let top_left = &wordsearch[row-1][col-1];
    let top_right = &wordsearch[row-1][col+1];
    let bottom_left = &wordsearch[row+1][col-1];
    let bottom_right = &wordsearch[row+1][col+1];
    let mut valid_upright = false;
    let mut valid_downleft = false;
    if *top_left == 'M' && *bottom_right == 'S' {
        valid_downleft = true;
    } 
    if *top_left == 'S' && *bottom_right == 'M' {
        valid_downleft = true;
    } 
    if *bottom_left == 'M' && *top_right == 'S' {
        valid_upright = true;
    } 
    if *bottom_left == 'S' && *top_right == 'M' {
        valid_upright = true;
    } 
    return (valid_downleft && valid_upright)
}

fn part1() {
    let wordsearch = read_input("input.txt");
    let mut total = 0;
    for i in 0..wordsearch.len() {
        let row = &wordsearch[i];
        for j in 0..row.len() {
            let this_letter = &wordsearch[i][j];
            if *this_letter == 'X' {
                let directions = vec![
                    (-1,-1), (-1,0), (-1,1),
                    (0,-1), (0,1),
                    (1,-1), (1,0), (1,1)
                ];
                for dir in directions {
                    total += find_word(i, j, &wordsearch, dir) as i32;
                }
            }
        }
    }
    println!("{}", total);
}

fn part2() {
    let wordsearch = read_input("input.txt");
    let mut total = 0;
    for i in 0..wordsearch.len() {
        let row = &wordsearch[i];
        for j in 0..row.len() {
            let this_letter = &wordsearch[i][j];
            if *this_letter == 'A' { // Start from the center of the X-MAS
                total += find_xmas(i, j, &wordsearch) as i32;
            }
        }
    }
    println!("{}", total);
}


fn main() {
    part1();
    part2();
}
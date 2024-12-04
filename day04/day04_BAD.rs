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

fn find_words(row: usize, col: usize, wordsearch: &Vec<Vec<char>>, word: String, path: Vec<(usize,usize)>) -> i32 {
    // Check if the next letter in the word is at this position (convert to char!)
    let next_letter = &word[0..1].chars().next().unwrap();
    let this_letter = &wordsearch[row][col];

    if next_letter != this_letter {
        // No word found in this path
        return 0;
    }
    if (next_letter == this_letter) && (word.len() == 1) {
        // Full word found in this path
        dbg!(path);
        return 1;
    }

    // Recursively search all adjacent paths
    let mut total = 0;
    let remaining_word = &word[1..word.len()];
    // This part is so inelegant, ugh!
    let mut adjacent_rows: Vec<usize> = vec![row];
    if row != 0 {
        adjacent_rows.push(row-1); // Order doesn't matter. To prevent subtract w/ overflow!
    }
    if row < wordsearch.len() - 1 {
        adjacent_rows.push(row+1);
    }
    let mut adjacent_cols: Vec<usize> = vec![col];
    if col != 0 {
        adjacent_cols.push(col-1); // Order doesn't matter. To prevent subtract w/ overflow!
    }
    if col < wordsearch[0].len() - 1 {
        adjacent_cols.push(col+1);
    }
    // Now we can finally do recursion.
    for next_row in &adjacent_rows {
        for next_col in &adjacent_cols {
            let mut next_path = path.clone();
            next_path.push((*next_row,*next_col));
            total += find_words(*next_row, *next_col, &wordsearch, (&remaining_word).to_string(), next_path);
        }
    }
    return total;
}

fn part1() {
    let wordsearch = read_input("input.txt");
    let mut total = 0;
    for i in 0..wordsearch.len() {
        let row = &wordsearch[i];
        for j in 0..row.len() {
            let new_words = find_words(i, j, &wordsearch, String::from("XMAS"), vec![(i,j)]);
            total += new_words;
            dbg!((i, j));
            dbg!(new_words);
        }
    }
    println!("{}", total);
}

fn main() {
    part1();
}
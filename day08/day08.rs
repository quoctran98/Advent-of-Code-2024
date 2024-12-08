use std::fs;
use std::collections::HashSet;
use itertools::iproduct;
use num::integer::gcd;
use std::cmp;

// use std::fs::File;
// use std::io::{self, BufWriter, Write};

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    for line in contents.lines() {
        let char_array = line.chars().collect();
        output.push(char_array);
    }
    return output
}

/// I'm adapting the Grid from day 6. I should make it more generic :)
struct Grid {
    width: i32,
    height: i32,
    antennas: Vec<(char, i32, i32)>,
    antinodes: Vec<(i32, i32)>
}

impl Grid {
    fn find_antennas(&self, freq: char) -> Vec<(char, i32, i32)> {
        return self.antennas.iter().filter(|(f, _, _)| freq == *f).cloned().collect();
    }
}

impl Grid {
    fn in_bounds(&self, coords: (i32, i32)) -> bool {
        // First coord is row! So it's height not width!
        return (coords.0 < self.height) && (coords.0 >= 0) && (coords.1 < self.width) && (coords.1 >= 0)
    }
}

// impl Grid {
//     fn write_to_txt(&self, fname: String) -> io::Result<()> {
//         let file = File::create(fname)?;
//         let mut writer = BufWriter::new(file);
//         for row in 0..self.height {
//             let mut line: Vec<char> = Vec::new();
//             let antennas_y: Vec<i32> = self.antennas.iter().filter(|&(_, x, _)| x == &row).map(|&(_, _, y)| y).collect();
//             let antinodes_y: Vec<i32> = self.antinodes.iter().filter(|&(x, _)| x == &row).map(|&(_, y)| y).collect();
//             for col in 0..self.width {
//                 if antennas_y.contains(&col) {
//                     line.push('0');
//                 } else if antinodes_y.contains(&col) {
//                     line.push('#');
//                 } else {
//                     line.push('.');
//                 }
//             }
//             // Write this line to the wrtier buffer
//             writeln!(writer, "{}", line.into_iter().collect::<String>())?;
//         }
//         writer.flush()?;
//         Ok(())
//     }
// }

fn get_antinodes_part1(antenna1: (char, i32, i32), antenna2: (char, i32, i32)) -> [(i32, i32); 2] {
    // I should have better error handling, but this works for now :)
    assert!(antenna1.0 == antenna2.0); // Make sure they're the same frequency 
    assert!((antenna1 != antenna2)); // Make sure this isn't the same antenna
    // I'm visualizing this as antenna1 is to the upper left of antenna2, but this should be general...
    let dx: i32 = antenna2.1 - antenna1.1;
    let dy: i32 = antenna2.2 - antenna1.2;
    let antinode1: (i32, i32) = (antenna1.1 - dx, antenna1.2 - dy);
    let antinode2: (i32, i32) = (antenna2.1 + dx, antenna2.2 + dy);
    return [antinode1, antinode2]
}

fn get_antinodes_part2(map: &Grid, antenna1: (char, i32, i32), antenna2: (char, i32, i32)) -> Vec<(i32, i32)> {
    assert!(antenna1.0 == antenna2.0); // Make sure they're the same frequency 
    assert!((antenna1 != antenna2)); // Make sure this isn't the same antenna
    // We can do the same thing to start with but then simplify the slope.
    // x and y notation are backwards but consistent...
    let mut dx: i32 = antenna2.1 - antenna1.1;
    let mut dy: i32 = antenna2.2 - antenna1.2;
    let this_gcd: i32 = gcd::<i32>(dx, dy);
    dx = dx / this_gcd;
    dy = dy / this_gcd;
    // Now we can just start moving in both directions (from antenna1) but (TRY) to stay in bounds!
    let max_steps: i32 = cmp::max((map.height / dx).abs(), (map.width / dy).abs()); // i32 truncation makes this easy!
    // We'll definitely escape bounds, but we'll deal with that after!
    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    for step in 0..=max_steps { // Don't forget step 0! And the last step!
        antinodes.push((antenna1.1 - (dx * step), antenna1.2 - (dy * step)));
        antinodes.push((antenna1.1 + (dx * step), antenna1.2 + (dy * step)));
    }
    return antinodes
}

fn part1(map: &mut Grid) {
    let frequencies: HashSet<char> = map.antennas.iter().map(|(freq, _, _)| freq).cloned().collect();
    for f in frequencies.into_iter() {
        let antennas = &map.find_antennas(f);
        for (a1, a2) in iproduct!(antennas, antennas) {
            // We waste calculations on (a1, a2) vs (a2, a1) but it's okay :)
            if a1 != a2 {
                let antinodes = get_antinodes_part1(*a1, *a2);
                for antinode in antinodes.into_iter() {
                    if map.in_bounds(antinode) {
                        map.antinodes.push(antinode);
                    }
                }

            }
        }
    }
    let antinodes: HashSet<(i32, i32)> = map.antinodes.iter().cloned().collect();
    println!("{}", antinodes.len());           
}

fn part2(map: &mut Grid) {
    let frequencies: HashSet<char> = map.antennas.iter().map(|(freq, _, _)| freq).cloned().collect();
    for f in frequencies.into_iter() {
        let antennas = &map.find_antennas(f);
        for (a1, a2) in iproduct!(antennas, antennas) {
            // We waste calculations on (a1, a2) vs (a2, a1) but it's okay :)
            if a1 != a2 {
                let antinodes = get_antinodes_part2(map, *a1, *a2);
                for antinode in antinodes.into_iter() {
                    if map.in_bounds(antinode) {
                        map.antinodes.push(antinode);
                    }
                }

            }
        }
    }
    let antinodes: HashSet<(i32, i32)> = map.antinodes.iter().cloned().collect();
    println!("{}", antinodes.len());           
}

fn main() {
    let input = read_input("input.txt");

    let mut map = Grid {
        width: input.len() as i32,
        height: input[0].len() as i32,
        antennas: Vec::new(),
        antinodes: Vec::new()
    };

    // Populate the map with anything not '.' as antennas
    for i in 0..map.width {
        for j in 0..map.height {
            let maybe_antenna: char = input[i as usize][j as usize];
            if maybe_antenna != '.' {
                map.antennas.push((maybe_antenna, i, j));
            }
        }
    }

    part1(&mut map);
    part2(&mut map);
}
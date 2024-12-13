use std::fs;
use std::collections::HashMap;
// use std::collections::HashSet; // All these regions SHOULD be HashSets but too late now :)
use itertools::iproduct;

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    let contents = fs::read_to_string(filename).expect("Can't read file!");
    for line in contents.lines() {
        let char_array = line.chars().collect();
        output.push(char_array);
    }
    return output
}

struct CharGrid {
    width: usize,
    height: usize,
    symbols: HashMap<char, Vec<(usize, usize)>>
}

impl CharGrid {
    fn new(grid_input: &Vec<Vec<char>>) -> CharGrid {
        let mut grid: CharGrid = CharGrid {
            // Width and height are "backwards" from how we look at things
            // but consistent with nested vector notation i.e. vec[x][y]
            width: grid_input.len(),
            height: grid_input[0].len(),
            symbols: HashMap::new()
        };

        // Add each unique character as a vector of coordinate tuples
        for x in 0..grid.width {
            for y in 0..grid.height {
                let symbol: char = grid_input[x][y];
                grid.add_symbol(symbol, (x, y));
            }
        }

        return grid
    }

    fn add_symbol(&mut self, symbol: char, coordinate: (usize, usize)) {
        if let Some(symbols_vec) = self.symbols.get_mut(&symbol) {
            symbols_vec.push(coordinate); // Push to vector on exisiting key
        } else {
            self.symbols.insert(symbol, vec![coordinate]); // Add new vector at key
        }
    }

    fn get_symbols(&self, symbol: char) -> Option<Vec<(usize, usize)>> {
        return self.symbols.get(&symbol).cloned()
    }
}

// Recursively find contiguous coordinates from a starting coordinate
fn find_contig_coords(mut current_region: Vec<(usize, usize)>, all_coords: &Vec<(usize, usize)>,) -> Vec<(usize, usize)> {
    let region_snapshot = current_region.clone(); // Clone the region to avoid borrowing issues
    for coord in region_snapshot {
        // Add immediately adjacent coordinates (if they exist and we haven't yet)
        for (x, y) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] { 
            let next_x: usize = (coord.0 as i32 + x).max(0) as usize;
            let next_y: usize = (coord.1 as i32 + y).max(0) as usize;
            if (all_coords.contains(&(next_x, next_y))) && (!current_region.contains(&(next_x, next_y))) {
                current_region.push((next_x, next_y));
                // Recurse to add coordinates adjacet to those, etc.
                current_region = find_contig_coords(current_region, all_coords);
            }
        }
    }
    return current_region
}

// Split each vector of a single symbol into multiple vectors for each region!
fn split_regions(mut coordinates: Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut all_regions: Vec<Vec<(usize, usize)>> = Vec::new();
    while coordinates.len() > 0 {
        let this_region: Vec<(usize, usize)> = find_contig_coords(vec![coordinates[0]], &coordinates);
        coordinates.retain(|c| !&this_region.contains(c)); // Remove this section from the vector
        all_regions.push(this_region);
    }
    return all_regions
}

// Helper for find_sides and find_perimeter
fn get_neighbors(coord: (usize, usize), region: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    for (x, y) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let next_x: usize = (coord.0 as i32 + x).max(0) as usize;
        let next_y: usize = (coord.1 as i32 + y).max(0) as usize;
        if (next_x, next_y) != coord { // Ignore itself!
            if region.contains(&(next_x, next_y)) {
                neighbors.push((next_x, next_y));
            }
        }
    }
    return neighbors
}

fn find_perimeter(region: &Vec<(usize, usize)>) -> u32 {
    let mut perimeter: u32 = (region.len() as u32) * 4_u32; // Everything has all four sides!
    for coord in region {
        //  Subtract 1 perimeter for each neighbor (no perimeter there!)
        perimeter -= get_neighbors(*coord, &region).len() as u32;
    }
    return perimeter
}

fn count_corners(region: &Vec<(usize, usize)>) -> u32 {
    let mut corners: f32 = 0.0;
    for coord in region {
        // Search for internal corners
        let neighbors: &Vec<(usize, usize)> = &get_neighbors(*coord, &region);
        if neighbors.len() == 0 {
            corners += 4.0 // We could just do an early return here! But we won't :)
        } else if neighbors.len() == 1 {
            corners += 2.0; // A protrusion creates 2 internal corners
        } else if neighbors.len() == 2 {
            if !((neighbors[0].0 == neighbors[1].0) || (neighbors[0].1 == neighbors[1].1)) {
                // If both neighbors share no coordinates, then it's an internal corner
                // Otherwise, they're parallel edges!
                corners += 1.0;
            }
        }

        // Search for external corners (each is EXPLICITLY associated with one internal spot)
        // For each neighbor pair, their intersections (0 (for n1==n2), 1, 2) will be in the region OR NOT!
        for (n1, n2) in iproduct!(neighbors, neighbors) {
            let intersect1_inside: bool = region.contains(&(n1.0, n2.1));
            let intersect2_inside: bool = region.contains(&(n2.0, n1.1));
            let n_inside: i32 = (intersect1_inside as i32) + (intersect2_inside as i32);
            assert!((n_inside == 1) || (n_inside == 2)); // I can't imagine a case where it's 0...
            // Where n1 and n2 are across from each other, n_inside is 2 and we have no external corners
            if n_inside == 1 {
                // println!("Found 1 external corner at ({}, {})", coord.0, coord.1);
                // println!("with neighbors ({}, {}) and ({}, {})", n1.0, n1.1, n2.0, n2.1);
                // WE'RE DOUBLE COUNTING BECUASE OF HOW IPRODUCT WORKS! 
                // easiest workaround is to just add 0.5 each time :)
                corners += 0.5;
            }
        }
    }
    assert!(corners.fract() == 0.0);
    return corners as u32
}

fn part1(grid: &CharGrid) {
    let mut price: u32 = 0; 
    for (_symbol, coords) in &grid.symbols {
        let regions: Vec<Vec<(usize, usize)>> = split_regions(coords.to_vec());
        for region in regions {
            let area: u32 = region.len() as u32;
            let perimeter: u32 = find_perimeter(&region);
            price += area * perimeter;
        }
    }
    println!("{}", price);
}

fn part2(grid: &CharGrid) {
    let mut price: u32 = 0; 
    for (_symbol, coords) in &grid.symbols {
        // println!("{}", _symbol);
        let regions: Vec<Vec<(usize, usize)>> = split_regions(coords.to_vec());
        for region in regions {
            let area: u32 = region.len() as u32;
            let sides: u32 = count_corners(&region);
            // println!("Region {} has area {} and {} sides.", _symbol, area, sides);
            price += area * sides;
        }
    }
    println!("{}", price);
}

fn main() {
    let grid = CharGrid::new(&read_input("input.txt"));
    part1(&grid);
    part2(&grid);
}
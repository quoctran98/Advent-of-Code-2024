use std::fs;

fn read_input(filename: &str) -> Vec<i32> {
    let contents: String = fs::read_to_string(filename).expect("Can't read file!");
    let input: Vec<i32> = contents
        .chars()
        .filter(|&c| c != '\n') // There's a newline at the very end... 
        .map(|c| c.to_digit(10).expect("Can't parse as digit...") as i32)
        .collect();
    return input
}

fn parse_to_disk(disk_map: &Vec<i32>) -> Vec<i32> {
    // Naively the disk is Vec<i32> where each index is a block with the file id (-1 is empty)
    // This might be WAY too big... We SHOULD deal with this in the condensed format, mathwise...
    // It's only 70K blocks long? Is that what we expect?
    // 20K * 0.5 (for half empty) * 5.5 (expected val. of 5.5 blocks for file) = 55K :)
    let mut disk: Vec<i32> = Vec::new();
    let mut file_flag: bool = true;
    let mut file_id: i32 = 0;
    for file_len in disk_map.into_iter() {
        let new_files: Vec<i32>;
        if file_flag {
            // What happens in the edge case where file_len == 0 for an actual file?
            assert!(*file_len != 0);
            new_files = vec![file_id; (*file_len as usize).try_into().unwrap()];
            file_id += 1;
        } else {
            new_files = vec![-1; (*file_len as usize).try_into().unwrap()];
        }
        file_flag = !file_flag; // Toggle between files and free space!
        disk.extend(new_files);
    }
    return disk
}

fn parse_to_disk2(chunk_sizes: &Vec<i32>, chunk_ids: &Vec<i32>) -> Vec<i32> {
    let mut disk: Vec<i32> = Vec::new();
    for i in 0..chunk_sizes.len() {
        let size = chunk_sizes[i];
        let id = chunk_ids[i];
        let new_files = vec![id; (size as usize).try_into().unwrap()];
        disk.extend(new_files);
    }
    return disk
}

// Probably worse than just using .find and .reverse, but this is nicer somehow...
fn move_head(disk: &Vec<i32>, head_start: usize, target: &str, direction: &str) -> Option<usize> {
    let mut head = head_start;
    let found_block = |block: i32| -> bool { // Oh it's like an anonymous function!
        if target == "file" {
            block >= 0
        } else {
            block < 0
        }
    };
    while !found_block(disk[head]) {
        if direction == "fwd" {
            // Ensure we're not about to go out of bounds
            if head == disk.len() - 1 {
                return None
            } else {
                head += 1;
            }
        } else {
            // Ensure we're not about to overflow with usize
            if head == 0 {
                return None
            } else {
                head -= 1;
            }
        }
    }
    return Some(head);
}

fn checksum(disk: &Vec<i32>, enforce_empty_blocks: bool) -> i64 {
    let mut sum: i64 = 0;
    let mut seen_empty_blocks = false;
    for (index, block) in disk.iter().enumerate() {
        if block >= &0 { // How does it make sense that I have to borrow a zero literal?
            if enforce_empty_blocks {
                assert!(!seen_empty_blocks);
            }
            sum += &(index as i64) * &(*block as i64);
        } else {
            seen_empty_blocks = true;
        }
    }
    return sum
}

fn find_empty_chunk(chunk_sizes: &Vec<i32>, chunk_ids: &Vec<i32>, this_size: i32, this_index: usize) -> Option<usize> {
    for index in 0..this_index { // Loop starting at the left until this_index (exclusive)
        if chunk_ids[index] == -1 { // Only consider open spots
            if chunk_sizes[index] >= this_size { // With enough size
                return Some(index)
            }
        }
    }
    return None
}

fn part1(input: &Vec<i32>) {
    let mut disk = parse_to_disk(input); 
    let mut fwd_head: Option<usize> = move_head(&disk, 0, "empty", "fwd"); // Start at first empty block
    let mut rev_head: Option<usize> = move_head(&disk, disk.len() - 1, "file", "rev"); // Start at last file block
    let mut done: bool = false;

    while !done {
        match (fwd_head, rev_head) {
            (Some(fwd), Some(rev)) => {
                // Make sure the heads haven't passed each other!
                if fwd > rev { break }
                // println!("{}", disk.iter().map(|x| if *x == -1 { ".".to_string() } else { x.to_string() }).collect::<String>());
                // println!("{}", disk.iter().enumerate().map(|(i,_)| if i == Some(fwd_head) || i == Some(rev_head) { "^" } else { "-" }).collect::<String>());
                // Swap the two blocks
                let temp_left = disk[fwd as usize];
                let temp_right = disk[rev as usize];
                assert!((temp_left == -1) && temp_right >= 0);
                disk[fwd as usize] = temp_right;
                disk[rev as usize] = temp_left;
                // Move the heads
                fwd_head = move_head(&disk, fwd, "empty", "fwd");
                rev_head = move_head(&disk, rev, "file", "rev");
            }
            _ => done = true,
        }
    }
    println!("{}", checksum(&disk, true));
}

fn part2(input: &Vec<i32>) {
    // Harder to work with the actual disk, so let's work with the input chunks
    let mut chunk_sizes: Vec<i32> = input.clone();
    let mut chunk_ids: Vec<i32> = Vec::new();
    let mut file_flag: bool = true;
    let mut file_id: i32 = 0;
    for _ in chunk_sizes.iter() {
        if file_flag {
            chunk_ids.push(file_id);
            file_id += 1;
        } else {
            chunk_ids.push(-1);
        }
        file_flag = !file_flag;
    }
    assert!(chunk_sizes.len() == chunk_ids.len());

    // Let's sort chunks in this format (we can't do a for loop because we're modifying so many things)
    let mut head: usize = chunk_sizes.len() - 1;
    while head >= 0 {
        // let disk = parse_to_disk2(&chunk_sizes, &chunk_ids);
        // println!("{}", disk.iter().map(|x| if *x == -1 { ".".to_string() } else { x.to_string() }).collect::<String>());
        // Look for empty block given current chunks
        let size = chunk_sizes[head];
        let id = chunk_ids[head];

        if id != -1 { // Skip swapping empty chunks (without this I failed but not sure why since swapping empty chunks should be equivalent)
            let chunk_result: Option<usize> = find_empty_chunk(&chunk_sizes, &chunk_ids, size, head);
            // Check and move chunks
            match chunk_result {
                Some(left_i) => {

                    // Make sure it's left of current index! Function doesn't check :)
                    let right_i = head.clone();
                    assert!(left_i < right_i);
                    assert!(chunk_ids[left_i] == -1);

                    // Do a direct swap
                    chunk_sizes[right_i] = chunk_sizes[left_i];
                    chunk_ids[right_i] = chunk_ids[left_i];
                    chunk_sizes[left_i] = size;
                    chunk_ids[left_i] = id;

                    // But we may need to add chunks and adjust sizes!
                    if chunk_sizes[left_i] < chunk_sizes[right_i] {
                        let size_diff: i32 = chunk_sizes[right_i] - chunk_sizes[left_i];
                        chunk_sizes[right_i] -= size_diff; // Take away free empty space that got moved right
                        // Add an empty space chunk after the file chunk that got moved left
                        chunk_sizes.insert(left_i + 1, size_diff);
                        chunk_ids.insert(left_i + 1, -1);
                    } 
                }
                _ => {}
        }
        
        }
        // Decrement counter
        // If I moved things around, I won't have to worry about underflow, but this is cleaner.
        if head == 0 {
            break
        } else {
            head -= 1;
        }
    }

    // Parse to disk then calculate checksum
    let disk = parse_to_disk2(&chunk_sizes, &chunk_ids);
    println!("{}", checksum(&disk, false));
}

fn main() {
    let input = read_input("input.txt");
    part1(&input);
    part2(&input);
}
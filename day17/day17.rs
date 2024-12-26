use std::fs;

fn read_input(filename: &str) -> ([u64; 3], Vec<u64>) {
    let mut registers: [u64; 3] = [0; 3];
    let mut program: Vec<u64> = Vec::new();
    for (i, line) in fs::read_to_string(filename).unwrap().lines().enumerate() {
        if i < 3 { // Registers
            registers[i] = line.split(": ").nth(1).unwrap()
                .parse::<u64>()
                .unwrap();
        }
        if i == 4 { // Program
            program = line.split(": ").nth(1).unwrap()
                .split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
        }
    }
    return (registers, program)
}

struct Computer {
    registers: [u64; 3], // A, B, C
    pointer: u64,
    output: Vec<u64>,
    program: Vec<u64>
}

impl Computer {
    fn new(registers: [u64; 3], program: Vec<u64>) -> Computer {
        return Computer {
            registers,
            pointer: 0,
            output: Vec::new(),
            program
        }
    }

    fn combo(&self, operand: u64) -> u64 {
        match operand {
            0 => { return 0 }, 
            1 => { return 1 }, 
            2 => { return 2 }, 
            3 => { return 3 }, 
            4 => { return self.registers[0].clone() }, 
            5 => { return self.registers[1].clone() }, 
            6 => { return self.registers[2].clone() }, 
            _ => { panic!("Bad combo operand!"); }
        }
    }

    fn adv(&mut self, operand: u64) -> (bool, bool) {
        let denominator: u64 = 2_u32.pow(self.combo(operand) as u32) as u64;
        self.registers[0] = self.registers[0] / denominator; 
        // Integer division is automatically truncated :)
        return (true, false)
    }

    fn bxl(&mut self, operand: u64) -> (bool, bool) {
        self.registers[1] = self.registers[1] ^ operand;
        return (true, false)
    }

    fn bst(&mut self, operand: u64) -> (bool, bool) {
        self.registers[1] = self.combo(operand) % 8_u64;
        return (true, false)
    }

    fn jnz(&mut self, operand: u64) -> (bool, bool)  {
        if self.registers[0] != 0 {
            self.pointer = operand;
            // Make sure to not move pointer!
            return (false, false)
        } else {
            return (true, false)
        }
    }

    fn bxc(&mut self, _operand: u64) -> (bool, bool) {
        self.registers[1] = self.registers[1] ^ self.registers[2];
        return (true, false)
    }

    fn out(&mut self, operand: u64) -> (bool, bool) {
        self.output.push(self.combo(operand) % 8_u64);
        // Check to make sure this new output shouldn't stop the program!
        return (true, !self.compare_output()) // don't forget to invert!
    }

    fn bdv(&mut self, operand: u64) -> (bool, bool) {
        let denominator: u64 = 2_u32.pow(self.combo(operand) as u32) as u64;
        self.registers[1] = self.registers[0] / denominator; 
        return (true, false)
    }

    fn cdv(&mut self, operand: u64) -> (bool, bool) {
        let denominator: u64 = 2_u32.pow(self.combo(operand) as u32) as u64;
        self.registers[2] = self.registers[0] / denominator; 
        return (true, false)
    }

    fn execute(&mut self) -> Vec<u64> {
        loop {
            // Check if we've moved the pointer out of bounds!
            if self.pointer >= self.program.len() as u64 { break }
            // Get the instruction and operand
            let instruction: u64 = self.program[self.pointer as usize];
            let operand: u64 = self.program[(self.pointer + 1_u64) as usize];
            // Execute the instruction
            let move_pointer: bool;
            match instruction {
                0 => { (move_pointer, _) = self.adv(operand); }, 
                1 => { (move_pointer, _) = self.bxl(operand); }, 
                2 => { (move_pointer, _) = self.bst(operand); }, 
                3 => { (move_pointer, _) = self.jnz(operand); }, 
                4 => { (move_pointer, _) = self.bxc(operand); }, 
                5 => { (move_pointer, _) = self.out(operand); }, 
                6 => { (move_pointer, _) = self.bdv(operand); }, 
                7 => { (move_pointer, _) = self.cdv(operand); }, 
                _ => { panic!("Unknown instruction!"); }
            }
            // Move the pointer (except if jnz returned false)
            if move_pointer { self.pointer += 2; }
        }
        // Return the output
        return self.output.clone()
    }

    // Return true if the output so far matches the program
    fn compare_output(&self) -> bool {
        if self.output.len() > self.program.len() {
            // Check if we're already longer!
            return false 
        } else {
            // Check if the output so far is correct
            for (i, out) in self.output.iter().enumerate() {
                if out != &self.program[i] {
                    return false
                }
            }
        }
        return true
    }

    fn execute_with_stop(&mut self) -> bool {
        loop {
            // Check if we've moved the pointer out of bounds!
            if self.pointer >= self.program.len() as u64 { break }
            // Get the instruction and operand
            let instruction: u64 = self.program[self.pointer as usize];
            let operand: u64 = self.program[(self.pointer + 1_u64) as usize];
            // Execute the instruction
            let move_pointer: bool;
            let stop_program: bool;
            match instruction {
                0 => { (move_pointer, stop_program) = self.adv(operand); }, 
                1 => { (move_pointer, stop_program) = self.bxl(operand); }, 
                2 => { (move_pointer, stop_program) = self.bst(operand); }, 
                3 => { (move_pointer, stop_program) = self.jnz(operand); }, 
                4 => { (move_pointer, stop_program) = self.bxc(operand); }, 
                5 => { (move_pointer, stop_program) = self.out(operand); },  // Will handle early input stopping!
                6 => { (move_pointer, stop_program) = self.bdv(operand); }, 
                7 => { (move_pointer, stop_program) = self.cdv(operand); }, 
                _ => { panic!("Unknown instruction!"); }
            }
            // Stop the program if an instruction wants us to do so!
            if stop_program { return false }
            // Move the pointer (except if jnz returned false)
            if move_pointer { self.pointer += 2; }
        }
        // The program terminated! Check that everything is right! Has to be same length and all equal!
        if self.output.len() == self.program.len() {
            return self.compare_output()
        } else {
            return false
        }
    }
}

fn part1(registers: [u64; 3], program: Vec<u64>) {
    let mut computer = Computer::new(registers, program);
    let output = computer.execute(); // Also accessible as computer.output
    println!("{}", output.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
}

// So the only jnz command is the last one. And the only out command is right before that.
// So we know that it has to cycle x times for a program of length x.
// Each cycle, it will output B mod 8.
// What if I just brute force while I'm in this meeting
fn part2(registers: [u64; 3], program: Vec<u64>) {
    let mut register_a = 5389000000;
    loop {
        if register_a % 1000000 == 0 {
            println!("{}", register_a);
        }
        let mut computer: Computer = Computer::new(registers.clone(), program.clone());
        computer.registers[0] = register_a.clone();
        let done = computer.execute_with_stop();
        if done {
            break
        } else {
            register_a += 1;
        }
    }
    println!("{}", register_a);
}

fn main() {
    let (registers, program) = read_input("input.txt");
    part1(registers.clone(), program.clone());
    part2(registers.clone(), program.clone());
}
use std::collections::VecDeque;

advent_of_code::solution!(17);

struct Computer {
    registers: [isize; 3],
    program: Vec<u8>,
    pointer: usize,
    output: VecDeque<u8>,
}

impl Computer {
    fn new(a: isize, b: isize, c: isize, program: Vec<u8>) -> Self {
        Self {
            registers: [a, b, c],
            program,
            pointer: 0,
            output: VecDeque::new(),
        }
    }

    fn run(&mut self) -> String {
        while self.pointer < self.program.len() {
            let opcode = self.program[self.pointer];
            let operand = self.program[self.pointer + 1];
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!("Unknown opcode: {}", opcode),
            }
            if opcode != 3 || self.registers[0] == 0 {
                self.pointer += 2;
            }
        }
        self.output.iter().map(|&v| v.to_string()).collect::<Vec<_>>().join(",")
    }

    fn get_combo_operand(&self, operand: u8) -> isize {
        match operand {
            0..=3 => operand as isize,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            7 => panic!("Invalid combo operand 7"),
            _ => unreachable!(),
        }
    }

    fn adv(&mut self, operand: u8) {
        let divisor = 2_isize.pow(self.get_combo_operand(operand) as u32);
        self.registers[0] /= divisor;
    }

    fn bxl(&mut self, operand: u8) {
        self.registers[1] ^= operand as isize;
    }

    fn bst(&mut self, operand: u8) {
        let value = self.get_combo_operand(operand) % 8;
        self.registers[1] = value;
    }

    fn jnz(&mut self, operand: u8) {
        if self.registers[0] != 0 {
            self.pointer = operand as usize;
        }
    }

    fn bxc(&mut self) {
        self.registers[1] ^= self.registers[2];
    }

    fn out(&mut self, operand: u8) {
        let value = self.get_combo_operand(operand) % 8;
        self.output.push_back(value as u8);
    }

    fn bdv(&mut self, operand: u8) {
        let divisor = 2_isize.pow(self.get_combo_operand(operand) as u32);
        self.registers[1] = self.registers[0] / divisor;
    }

    fn cdv(&mut self, operand: u8) {
        let divisor = 2_isize.pow(self.get_combo_operand(operand) as u32);
        self.registers[2] = self.registers[0] / divisor;
    }
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let mut registers = [0; 3];
        let mut program = Vec::new();
    
        for line in value.lines() {
            if line.starts_with("Register A") {
                registers[0] = line.split(": ").nth(1).unwrap().parse::<isize>().unwrap();
            } else if line.starts_with("Register B") {
                registers[1] = line.split(": ").nth(1).unwrap().parse::<isize>().unwrap();
            } else if line.starts_with("Register C") {
                registers[2] = line.split(": ").nth(1).unwrap().parse::<isize>().unwrap();
            } else if line.starts_with("Program") {
                program = line
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .map(|num| num.parse::<u8>().unwrap())
                    .collect();
            }
        }
    
        Computer {
            registers,
            program,
            pointer: 0,
            output: VecDeque::new(),
        }
    }
}


fn find_lowest_a(program: &[u8]) -> isize {
    let program_output = program.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");

    for a in 1.. {
        let mut computer = Computer::new(a, 0, 0, program.to_vec());
        let output = computer.run();
        
        if output == program_output {
            return a;
        }
    }
    unreachable!("No valid A value found");
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::from(input);
    let result = computer.run();
    Some(result)
}

pub fn part_two(input: &str) -> Option<isize> {
    let computer = Computer::from(input);
    let result = find_lowest_a(&computer.program);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&binding);
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let binding = advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(&binding);
        assert_eq!(result, Some(117440));
    }
}

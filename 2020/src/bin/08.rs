// Day 8: Handheld Halting
fn main() {
    let file = aoc_util::read_input_file(2020, 8);
    let sol = solver1(&file);
    println!("Result Part 1: {}", sol);
    let sol = solver2(&file);
    println!("Result Part 2: {}", sol);
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse(s: &str) -> Instruction {
    let (op, arg) = s.split_at(3);
    let arg = arg.trim().parse().unwrap();
    match op {
        "nop" => Instruction::Nop(arg),
        "acc" => Instruction::Acc(arg),
        "jmp" => Instruction::Jmp(arg),
        _ => panic!("Unknown instruction"),
    }
}

fn solver1(s: &str) -> usize {
    let instructions = s.lines().map(parse).collect::<Vec<_>>();
    let mut visited = vec![false; instructions.len()];
    let mut acc = 0;
    let mut counter = 0;
    loop {
        if visited[counter] {
            break;
        }
        visited[counter] = true;
        match instructions[counter] {
            Instruction::Nop(_) => counter += 1,
            Instruction::Acc(arg) => {
                acc += arg;
                counter += 1;
            }
            Instruction::Jmp(arg) => counter = (counter as i32 + arg) as usize,
        };
    }
    acc as usize
}

fn solver2(s: &str) -> usize {
    let mut instructions = s.lines().map(parse).collect::<Vec<_>>();
    let mut visited = vec![false; instructions.len()];
    let mut last_tested = 0;
    let mut last_instruction;
    loop {
        {
            loop {
                let instruction = &mut instructions[last_tested];
                match instruction {
                    Instruction::Nop(arg) => {
                        last_instruction = Instruction::Nop(*arg);
                        *instruction = Instruction::Jmp(*arg);
                        break;
                    }
                    Instruction::Jmp(arg) => {
                        last_instruction = Instruction::Jmp(*arg);
                        *instruction = Instruction::Nop(*arg);
                        break;
                    }
                    _ => last_tested += 1,
                }
            }
        }
        visited.iter_mut().for_each(|v| *v = false);
        let mut acc = 0;
        let mut counter = 0;
        loop {
            if visited[counter] {
                instructions[last_tested] = last_instruction;
                last_tested += 1;
                break;
            }
            visited[counter] = true;
            match instructions[counter] {
                Instruction::Nop(_) => counter += 1,
                Instruction::Acc(arg) => {
                    acc += arg;
                    counter += 1;
                }
                Instruction::Jmp(arg) => counter = (counter as i32 + arg) as usize,
            };
            if counter >= instructions.len() {
                return acc as usize;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_solver1() {
        let s = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let r = solver1(s);
        assert_eq!(r, 5);
    }

    #[test]
    fn test_solver2() {
        let s = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let r = solver2(s);
        assert_eq!(r, 8);
    }
}

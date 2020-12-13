use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleComputer {
    instructions: Vec<Instruction>,
}

impl SimpleComputer {
    fn execute(&self) -> (i32, bool) {
        let mut pc = 0;
        let mut seen = HashSet::new();
        let mut accumulator = 0;

        while !seen.contains(&pc) && pc != self.instructions.len() {
            seen.insert(pc);

            match self.instructions[pc] {
                Instruction::Acc(n) => accumulator += n,
                Instruction::Jmp(n) => {
                    if n.is_negative() {
                        pc -= usize::try_from(n.abs()).unwrap()
                    } else {
                        pc += usize::try_from(n).unwrap()
                    }

                    continue;
                }
                Instruction::Nop(_) => (),
            }
            pc += 1;
        }

        (accumulator, pc == self.instructions.len())
    }

    fn swap(&mut self, n: usize) -> bool {
        self.instructions[n].swap()
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    fn swap(&mut self) -> bool {
        *self = match self {
            Instruction::Jmp(n) => Instruction::Nop(*n),
            Instruction::Nop(n) => Instruction::Jmp(*n),
            _ => return false,
        };

        true
    }
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> SimpleComputer {
    let instructions = input
        .lines()
        .map(|line| {
            let ins = &line[..3];
            let num = line[4..].parse().unwrap();

            match ins {
                "acc" => Instruction::Acc(num),
                "jmp" => Instruction::Jmp(num),
                "nop" => Instruction::Nop(num),
                _ => panic!("unknown instruction"),
            }
        })
        .collect();

    SimpleComputer { instructions }
}

#[aoc(day8, part1)]
pub fn part1(computer: &SimpleComputer) -> i32 {
    computer.execute().0
}

#[aoc(day8, part2)]
pub fn part2(computer: &SimpleComputer) -> i32 {
    let len = computer.instructions.len();
    let mut patched_computer = computer.clone();

    for i in 0..len {
        if patched_computer.swap(i) {
            let (n, terminated) = patched_computer.execute();
            if terminated {
                return n;
            }

            patched_computer.swap(i);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    pub fn test_input() {
        use Instruction::*;
        // println!("{:?}", generator(SAMPLE));

        assert_eq!(
            generator(SAMPLE),
            SimpleComputer {
                instructions: vec![
                    Nop(0),
                    Acc(1),
                    Jmp(4),
                    Acc(3),
                    Jmp(-3),
                    Acc(-99),
                    Acc(1),
                    Jmp(-4),
                    Acc(6)
                ]
            }
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 5);
    }

    #[test]
    pub fn test2() {
        assert_eq!(generator(SAMPLE).execute().1, false);

        assert_eq!(part2(&generator(SAMPLE)), 8);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day8.txt");
        const ANSWERS: (i32, i32) = (1489, 1539);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

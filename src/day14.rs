use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct ProgramFragment {
    mask: Vec<(usize, char)>,
    cmds: Vec<(usize, usize)>,
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<ProgramFragment> {
    input
        .split("mask ")
        .filter(|group| !group.is_empty())
        .map(|group| parse_group(group))
        .collect()
}

fn parse_group(group: &str) -> ProgramFragment {
    let mut line_iter = group.lines();

    // Discard everything left of the = on the first line, collect chars into array.
    let mask = line_iter.next().unwrap().split('=').nth(1).unwrap().trim();
    let mask = mask.chars().rev().enumerate().collect();

    // Rest of input from group
    let cmds = line_iter
        .map(|line| {
            // Split on the equal
            let mut cmd_iter = line.split('=');

            // Get the characters between [ ]
            let memory_loc = cmd_iter
                .next()
                .unwrap()
                .split(&['[', ']'][..])
                .nth(1)
                .unwrap();

            // return memory location and value
            (
                memory_loc.parse().unwrap(),
                cmd_iter.next().unwrap().trim().parse().unwrap(),
            )
        })
        .collect::<Vec<(_, _)>>();

    ProgramFragment { mask, cmds }
}

#[aoc(day14, part1)]
pub fn part1(program: &[ProgramFragment]) -> usize {
    let mut memory = HashMap::new();

    for fragment in program {
        for &(memory_location, mut value) in fragment.cmds.iter() {
            for &(mask_index, mask_char) in fragment.mask.iter() {
                // get current bit
                let current = (value >> mask_index) & 0x1;

                // If they are different build a mask that flips it
                let mask = match (mask_char, current) {
                    ('1', 0) => 1 << mask_index,
                    ('0', 1) => 1 << mask_index,
                    _ => continue,
                };

                value ^= mask;
            }

            memory.insert(memory_location, value);
        }
    }

    memory.values().sum()
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Bit {
    One,
    Zero,
    Floating,
}

fn int_to_bitarray(mut num: usize) -> [Bit; 36] {
    let mut ans = [Bit::Zero; 36];

    for ans_ptr in ans.iter_mut().rev() {
        let bit = num & 0x1 > 0;
        *ans_ptr = if bit { Bit::One } else { Bit::Zero };
        num >>= 1;
    }

    ans
}

fn bitarray_to_int(array: [Bit; 36]) -> usize {
    array.iter().fold(0, |total, bit| {
        (total << 1) + if *bit == Bit::One { 1 } else { 0 }
    })
}

fn permute(mut stack: Vec<[Bit; 36]>) -> Vec<[Bit; 36]> {
    let mut ans = Vec::new();

    'next: while let Some(mut array) = stack.pop() {
        for i in 0..36 {
            if array[i] == Bit::Floating {
                array[i] = Bit::One;
                stack.push(array);

                array[i] = Bit::Zero;
                stack.push(array);

                continue 'next;
            }
        }

        ans.push(array);
    }

    ans
}

#[aoc(day14, part2)]
pub fn part2(program: &[ProgramFragment]) -> usize {
    let mut memory = HashMap::new();

    for fragment in program {
        for &(memory_location, value) in fragment.cmds.iter() {
            let mut bit_mem_loc = int_to_bitarray(memory_location);

            for &(mask_index, mask_v) in fragment.mask.iter() {
                match mask_v {
                    '1' => bit_mem_loc[35 - mask_index] = Bit::One,
                    'X' => bit_mem_loc[35 - mask_index] = Bit::Floating,
                    '0' => {}
                    _ => unreachable!(),
                }
            }

            for l in permute(vec![bit_mem_loc]) {
                memory.insert(bitarray_to_int(l), value);
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

    const SAMPLE2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 165);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE2)), 208);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day14.txt");
        const ANSWERS: (usize, usize) = (6386593869035, 4288986482164);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

use std::{collections::HashMap, iter::FromIterator};

#[derive(Debug, PartialEq)]
pub struct ProgramFragment {
    mask: BitNumber,
    cmds: Vec<(usize, usize)>,
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<ProgramFragment> {
    input
        .split("mask ")
        .filter_map(|group| {
            if !group.is_empty() {
                Some(parse_group(group))
            } else {
                None
            }
        })
        .collect()
}

fn parse_group(group: &str) -> ProgramFragment {
    let mut line_iter = group.lines();

    // Discard everything left of the = on the first line, collect chars into array.
    let mask = BitNumber::from(line_iter.next().unwrap().split('=').nth(1).unwrap().trim());

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
        .collect();

    ProgramFragment { mask, cmds }
}

#[aoc(day14, part1)]
pub fn part1(program: &[ProgramFragment]) -> usize {
    let mut memory = HashMap::new();

    for fragment in program {
        for &(memory_location, mut value) in fragment.cmds.iter() {
            for (mask_index, &mask_char) in fragment.mask.0.iter().enumerate() {
                // get current bit
                let current = (value >> mask_index) & 0x1;

                // If they are different build a mask that flips it
                let mask = match (mask_char, current) {
                    (Bit::One, 0) | (Bit::Zero, 1) => 1 << mask_index,
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

impl From<char> for Bit {
    fn from(x: char) -> Self {
        match x {
            '1' => Bit::One,
            '0' => Bit::Zero,
            'X' => Bit::Floating,
            _ => panic!("invalid character"),
        }
    }
}

#[derive(PartialEq)]
struct BitNumber([Bit; 36]);

impl BitNumber {
    fn permute(self) -> Vec<BitNumber> {
        let mut stack = vec![(0, self.0)];
        let mut ans = Vec::new();

        'next: while let Some((start_idx, mut array)) = stack.pop() {
            for i in start_idx..36 {
                if array[i] == Bit::Floating {
                    array[i] = Bit::One;
                    stack.push((i + 1, array));

                    array[i] = Bit::Zero;
                    stack.push((i + 1, array));

                    continue 'next;
                }
            }

            ans.push(BitNumber(array));
        }

        ans
    }
}

impl From<usize> for BitNumber {
    fn from(num: usize) -> Self {
        let mut num = num;
        let mut ans = [Bit::Zero; 36];

        for ans_ptr in ans.iter_mut().rev() {
            let bit = num & 0x1 > 0;
            *ans_ptr = if bit { Bit::One } else { Bit::Zero };
            num >>= 1;
        }

        Self(ans)
    }
}

impl From<&str> for BitNumber {
    fn from(s: &str) -> Self {
        s.chars().collect()
    }
}

impl From<BitNumber> for usize {
    fn from(array: BitNumber) -> Self {
        array.0.iter().fold(0, |total, bit| {
            (total << 1) + if *bit == Bit::One { 1 } else { 0 }
        })
    }
}

impl FromIterator<char> for BitNumber {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut ans = [Bit::Zero; 36];

        for (bit, target) in iter.into_iter().map(|x| x.into()).zip(ans.iter_mut().rev()) {
            *target = bit;
        }

        Self(ans)
    }
}

impl std::fmt::Display for BitNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in self.0.iter().rev() {
            let c = match d {
                Bit::One => '1',
                Bit::Zero => '0',
                Bit::Floating => 'X',
            };

            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for BitNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[aoc(day14, part2)]
pub fn part2(program: &[ProgramFragment]) -> usize {
    let mut memory = HashMap::new();

    for fragment in program {
        for &(memory_location, value) in fragment.cmds.iter() {
            let mut bit_mem_loc = BitNumber::from(memory_location);

            for (mask_index, &mask_v) in fragment.mask.0.iter().enumerate() {
                match mask_v {
                    Bit::One => bit_mem_loc.0[35 - mask_index] = Bit::One,
                    Bit::Floating => bit_mem_loc.0[35 - mask_index] = Bit::Floating,
                    Bit::Zero => {}
                }
            }

            for l in bit_mem_loc.permute() {
                memory.insert(usize::from(l), value);
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const SAMPLE2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    pub fn test_input() {
        assert_eq!(
            generator(SAMPLE1),
            vec![ProgramFragment {
                mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".into(),
                cmds: vec![(8, 11), (7, 101), (8, 0)],
            }]
        );

        assert_eq!(
            generator(SAMPLE2),
            vec![
                ProgramFragment {
                    mask: "000000000000000000000000000000X1001X".into(),
                    cmds: vec![(42, 100)],
                },
                ProgramFragment {
                    mask: "00000000000000000000000000000000X0XX".into(),
                    cmds: vec![(26, 1)],
                }
            ]
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE1)), 165);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE2)), 208);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day14.txt");
        const ANSWERS: (usize, usize) = (6_386_593_869_035, 4_288_986_482_164);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

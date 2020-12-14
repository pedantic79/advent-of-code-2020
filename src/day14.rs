use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Object {
    mask: HashMap<usize, char>,
    cmds: Vec<(usize, usize)>,
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<Object> {
    input
        .split("mask ")
        .filter(|x| !x.is_empty())
        .map(|group| parse_group(group))
        .collect()
}

fn parse_group(input: &str) -> Object {
    let mut liter = input.lines();

    let mask_s = liter.next().unwrap().split('=').nth(1).unwrap().trim();

    let mask = mask_s.chars().rev().enumerate().collect();

    let cmds = liter
        .map(|line| {
            let mut l = line.split('=');
            (
                l.next()
                    .unwrap()
                    .split(&['[', ']'][..])
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap(),
                l.next().unwrap().trim().parse().unwrap(),
            )
        })
        .collect::<Vec<(_, _)>>();

    Object { mask, cmds }
}

#[aoc(day14, part1)]
pub fn part1(group: &[Object]) -> usize {
    let mut memory = HashMap::new();

    for inputs in group {
        for (location, value) in inputs.cmds.iter() {
            let mut value = *value;
            for (mloc, mval) in inputs.mask.iter() {
                let current = (value >> mloc) & 0x1;
                let mask = match (*mval, current) {
                    ('1', 0) => 1 << mloc,
                    ('0', 1) => 1 << mloc,
                    _ => continue,
                };

                value ^= mask;
            }

            memory.insert(*location, value);
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

fn int_to_bitarray(mut input: usize) -> [Bit; 36] {
    let mut ans = [Bit::Zero; 36];
    let mut i: usize = 35;
    while input > 0 {
        let bit = input & 0x1 > 0;
        ans[i] = if bit { Bit::One } else { Bit::Zero };
        i -= 1;
        input >>= 1;
    }

    ans
}

fn bitarray_to_int(array: [Bit; 36]) -> usize {
    array.iter().fold(0, |num, bit| {
        (num << 1) + if *bit == Bit::One { 1 } else { 0 }
    })
}

fn permute(mut stack: Vec<[Bit; 36]>) -> Vec<[Bit; 36]> {
    let mut ans = vec![];
    while let Some(mut array) = stack.pop() {
        let mut changed = false;
        for i in 0..36 {
            if array[i] == Bit::Floating {
                array[i] = Bit::One;
                stack.push(array);

                array[i] = Bit::Zero;
                stack.push(array);

                changed = true;
                break;
            }
        }

        if !changed {
            ans.push(array);
        }
    }

    ans
}

#[aoc(day14, part2)]
pub fn part2(group: &[Object]) -> usize {
    let mut memory = HashMap::new();

    for inputs in group {
        for &(mem_location, value) in inputs.cmds.iter() {
            let mut locations = vec![];

            let mut ml = int_to_bitarray(mem_location);

            for (mask_l, mask_v) in inputs.mask.iter() {
                if *mask_v == '0' {
                    continue;
                }
                match *mask_v {
                    '1' => ml[35 - *mask_l] = Bit::One,
                    '0' => {}
                    _ => ml[35 - *mask_l] = Bit::Floating,
                }
            }
            locations.push(ml);

            locations = permute(locations);
            for l in locations {
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

use std::{collections::HashMap, convert::TryInto};

const PART1_ITERATIONS: usize = 2020;
const PART2_ITERATIONS: usize = 30_000_000;

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc_generator(day15, part2, thirtytwo)]
pub fn generator_32(input: &str) -> Vec<u32> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

pub fn solve(inputs: &[usize], limit: usize) -> usize {
    let mut seen: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut last = 0;

    for i in 0..limit {
        let val = if i < inputs.len() {
            inputs[i]
        } else if let Some((one_ago, two_ago)) = seen.get(&last) {
            one_ago - two_ago
        } else {
            0
        };

        last = val;
        seen.entry(val)
            .and_modify(|(o, t)| {
                *t = *o;
                *o = i + 1;
            })
            .or_insert((i + 1, i + 1));
    }

    last
}

pub fn solve_alt(inputs: &[usize], limit: usize) -> usize {
    let mut seen = vec![None; limit];
    let mut last = 0;

    for (i, t) in inputs.iter().copied().zip(1..) {
        seen[i] = Some(t);
        last = i;
    }

    for t in inputs.len()..limit {
        let speak = seen[last].map_or(0, |n| t - n);
        seen[last] = Some(t);
        last = speak;
    }

    last
}

pub fn solve_32(inputs: &[u32], limit: u32) -> u32 {
    let mut seen = vec![0; limit as usize];
    let mut last = 0;

    for (&i, t) in inputs.iter().zip(1..) {
        seen[i as usize] = t;
        last = i;
    }

    let l = inputs.len().try_into().unwrap();
    for i in l..limit {
        let v = seen[last as usize];
        seen[last as usize] = i;
        last = if v == 0 { 0 } else { i - v };
    }

    last
}

#[aoc(day15, part1)]
pub fn part1(inputs: &[usize]) -> usize {
    solve(inputs, PART1_ITERATIONS)
}

#[aoc(day15, part1, alt)]
pub fn part1_alt(inputs: &[usize]) -> usize {
    solve_alt(inputs, PART1_ITERATIONS)
}

#[cfg(slow)]
#[aoc(day15, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    solve(inputs, PART2_ITERATIONS)
}

#[aoc(day15, part2, alt)]
pub fn part2_alt(inputs: &[usize]) -> usize {
    solve_alt(inputs, PART2_ITERATIONS)
}

#[aoc(day15, part2, thirtytwo)]
pub fn part2_32(inputs: &[u32]) -> u32 {
    solve_32(inputs, PART2_ITERATIONS.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"0,3,6";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        assert_eq!(generator(SAMPLE), vec![0, 3, 6]);
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 436);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2_32(&generator_32(SAMPLE)), 175594);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day15.txt");
        const ANSWERS: (usize, u32) = (1618, 548531);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1_alt(&generator(input)), ANSWERS.0);
            assert_eq!(part2_32(&generator_32(input)), ANSWERS.1);
        }
    }
}

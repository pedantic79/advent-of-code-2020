use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashSet},
};

const TARGET: i32 = 2020;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1, BTreeSet)]
pub fn part1(inputs: &[i32]) -> i32 {
    let mut seen = BTreeSet::new();

    for input in inputs {
        let remainder = TARGET - *input;
        if seen.contains(&remainder) {
            return remainder * input;
        }

        seen.insert(*input);
    }

    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2(inputs: &[i32]) -> i32 {
    let mut inputs = inputs.to_vec();
    inputs.sort_unstable();
    let len = inputs.len();

    for (i, a) in inputs[0..(len - 2)].iter().enumerate() {
        let mut left = i + 1;
        let mut right = len - 1;
        while left < right {
            let b = inputs[left];
            let c = inputs[right];
            let sum = a + b + c;

            match sum.cmp(&TARGET) {
                Ordering::Less => left += 1,
                Ordering::Equal => return a * b * c,
                Ordering::Greater => right -= 1,
            }
        }
    }

    unreachable!()
}

#[aoc(day1, part2, cache)]
pub fn part2_set(inputs: &[i32]) -> i32 {
    let cache = inputs.iter().collect::<BTreeSet<_>>();

    for (i, &a) in inputs.iter().enumerate() {
        for &b in inputs.iter().skip(i) {
            let target = TARGET - a - b;

            if cache.contains(&target) {
                return a * b * target;
            }
        }
    }

    unreachable!()
}

#[aoc(day1, part1, HashSet)]
pub fn part1_hashset(inputs: &[i32]) -> i32 {
    let mut seen = HashSet::new();

    for input in inputs {
        let remainder = TARGET - *input;
        if seen.contains(&remainder) {
            return remainder * input;
        }

        seen.insert(input);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    pub fn test_input() {
        assert_eq!(generator(SAMPLE), vec![1721, 979, 366, 299, 675, 1456]);
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 1721 * 299)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 979 * 366 * 675)
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day1.txt");
        const ANSWERS: (i32, i32) = (1_018_944, 8_446_464);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

use std::{cmp::Ordering, collections::HashSet};

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Option<Vec<i64>> {
    input.lines().map(|x| x.parse().ok()).collect()
}

fn sum2(nums: &[i64], total: i64) -> Option<i64> {
    let mut seen = HashSet::new();

    for &num in nums {
        let target = total - num;
        if seen.contains(&target) {
            return None;
        } else {
            seen.insert(num);
        }
    }

    Some(total)
}

fn find_range_bounds(inputs: &[i64], target: i64) -> (usize, usize) {
    for start in 0..inputs.len() {
        let mut sum = 0;
        for (i, input) in inputs.iter().enumerate().skip(start) {
            sum += input;

            match sum.cmp(&target) {
                Ordering::Less => {}
                Ordering::Equal => return (start, i),
                Ordering::Greater => break,
            }
        }
    }

    unreachable!()
}

fn find_invalid(input: &[i64], preamble: usize) -> i64 {
    input
        .windows(preamble + 1)
        .find_map(|range| sum2(&range[..preamble], range.last().copied().unwrap()))
        .unwrap()
}

#[aoc(day9, part1)]
pub fn part1(inputs: &[i64]) -> i64 {
    find_invalid(inputs, 25)
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[i64]) -> i64 {
    let target = find_invalid(inputs, 25);
    let (left, right) = find_range_bounds(inputs, target);

    let min = inputs[left..=right].iter().min().unwrap();
    let max = inputs[left..=right].iter().max().unwrap();

    min + max
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    pub fn test_input() {
        assert_eq!(
            generator(SAMPLE),
            Some(vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576
            ])
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(find_invalid(&generator(SAMPLE).unwrap(), 5), 127);
    }

    #[test]
    pub fn test2() {
        let inputs = generator(SAMPLE).unwrap();

        let (left, right) =
            find_range_bounds(&inputs, find_invalid(&generator(SAMPLE).unwrap(), 5));

        let min = inputs[left..right].iter().min().unwrap();
        let max = inputs[left..right].iter().max().unwrap();

        assert_eq!(min + max, 62)
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day9.txt");
        const ANSWERS: (i64, i64) = (1504371145, 183278487);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
            assert_eq!(part1(&generator(input).unwrap()), ANSWERS.0);
            assert_eq!(part2(&generator(input).unwrap()), ANSWERS.1);
        }
    }
}

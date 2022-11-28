use super::common::MinMaxIterator;
use std::cmp::Ordering::{Equal, Greater, Less};

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Option<Vec<usize>> {
    input.lines().map(|line| line.parse().ok()).collect()
}

fn find_invalid((total, haystack): (&usize, &[usize])) -> Option<usize> {
    haystack
        .iter()
        .find(|&&num| num < *total && haystack.contains(&(total - num)))
        .copied()
        .xor(Some(*total))
}

fn find_range_bounds(inputs: &[usize], target: usize) -> (usize, usize) {
    (0..inputs.len())
        .find_map(|start| {
            let mut sum = 0;
            for (end, num) in inputs.iter().enumerate().skip(start) {
                sum += num;

                return match sum.cmp(&target) {
                    Less => continue,
                    Equal => Some((start, end)),
                    Greater => None,
                };
            }

            unreachable!()
        })
        .unwrap()
}

fn part1_with_size(inputs: &[usize], preamble: usize) -> usize {
    inputs
        .windows(preamble + 1)
        .find_map(|range| find_invalid(range.split_last().unwrap()))
        .unwrap()
}

#[aoc(day9, part1)]
pub fn part1(inputs: &[usize]) -> usize {
    part1_with_size(inputs, 25)
}

fn part2_with_size(inputs: &[usize], preamble: usize) -> usize {
    let (left, right) = find_range_bounds(inputs, part1_with_size(inputs, preamble));
    let (min, max) = inputs[left..=right].iter().min_max().unwrap();

    min + max
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    part2_with_size(inputs, 25)
}

#[aoc(day9, part2, prefix1)]
pub fn part2_prefix1(inputs: &[usize]) -> usize {
    let (left, right) = find_range_bounds_prefix1(inputs, part1_with_size(inputs, 25));
    let (min, max) = inputs[left..=right].iter().min_max().unwrap();

    min + max
}

#[aoc(day9, part2, prefix2)]
pub fn part2_prefix2(inputs: &[usize]) -> usize {
    let (left, right) = find_range_bounds_prefix2(inputs, part1_with_size(inputs, 25));
    let (min, max) = inputs[left..=right].iter().min_max().unwrap();

    min + max
}

#[aoc(day9, part2, simple)]
pub fn part2_simple(inputs: &[usize]) -> usize {
    let target = part1_with_size(inputs, 25);
    let mut sum = 0;
    let mut start = 0;
    let mut end = 0;

    loop {
        while sum < target {
            sum += inputs[end];
            end += 1;
        }

        while sum > target {
            sum -= inputs[start];
            start += 1;
        }

        if sum == target {
            break;
        }
    }

    let (min, max) = inputs[start..end].iter().min_max().unwrap();

    min + max
}

fn find_range_bounds_prefix1(inputs: &[usize], target: usize) -> (usize, usize) {
    let prefixes = inputs
        .iter()
        .scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        })
        .collect::<Vec<_>>();

    prefixes
        .iter()
        .enumerate()
        .find_map(|(start, &prefix)| {
            for (end, sum) in prefixes
                .iter()
                .enumerate()
                .skip(start + 2)
                .map(|(end, &last)| (end, last - prefix))
            {
                return match sum.cmp(&target) {
                    Less => continue,
                    Equal => Some((start, end)),
                    Greater => None,
                };
            }

            unreachable!()
        })
        .unwrap()
}

fn find_range_bounds_prefix2(inputs: &[usize], target: usize) -> (usize, usize) {
    let prefixes = inputs
        .iter()
        .scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        })
        .collect::<Vec<_>>();

    prefixes
        .iter()
        .enumerate()
        .find_map(|(start, &prefix)| {
            prefixes
                .iter()
                .enumerate()
                .skip(start + 2)
                .map(|(end, &last)| (end, last - prefix))
                .take_while(|&(_, sum)| sum <= target)
                .find_map(|(end, sum)| {
                    if sum == target {
                        Some((start, end))
                    } else {
                        None
                    }
                })
        })
        .unwrap()
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
        assert_eq!(part1_with_size(&generator(SAMPLE).unwrap(), 5), 127);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2_with_size(&generator(SAMPLE).unwrap(), 5), 62);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day9.txt");
        const ANSWERS: (usize, usize) = (1_504_371_145, 183_278_487);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
            assert_eq!(part1(&generator(input).unwrap()), ANSWERS.0);
            assert_eq!(part2(&generator(input).unwrap()), ANSWERS.1);
        }
    }
}

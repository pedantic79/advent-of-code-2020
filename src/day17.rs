use std::collections::{HashMap, HashSet};

fn neighbors(n: usize) -> Vec<Vec<i64>> {
    fn helper(ans: &mut Vec<Vec<i64>>, v: &mut Vec<i64>, n: usize) {
        if n == 0 {
            ans.push(v.clone());
            return;
        }

        for i in -1..=1 {
            v.push(i);
            helper(ans, v, n - 1);
            v.pop();
        }
    }

    let mut ans = vec![];
    helper(&mut ans, &mut vec![], n);
    ans
}

pub fn tick(state: HashSet<Vec<i64>>) -> HashSet<Vec<i64>> {
    let mut counts = HashMap::new();
    for point in state.iter() {
        for mut neighbor in neighbors(point.len()) {
            for (n, p) in neighbor.iter_mut().zip(point.iter()) {
                *n += p;
            }

            *counts.entry(neighbor).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .filter_map(|(point, count)| {
            if count == 3 || count == 4 && state.contains(&point) {
                Some(point)
            } else {
                None
            }
        })
        .collect()
}

#[aoc_generator(day17)]
pub fn generator(input: &str) -> HashSet<(i64, i64)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (y as i64, x as i64))
        })
        .collect()
}

pub fn solve(cs: &HashSet<(i64, i64)>, extra_dim: bool) -> usize {
    let mut state = cs
        .iter()
        .map(|&(x, y)| {
            let mut p = vec![x, y, 0];
            if extra_dim {
                p.push(0);
            }
            p
        })
        .collect::<HashSet<_>>();

    for _ in 0..6 {
        state = tick(state)
    }

    state.len()
}

#[aoc(day17, part1)]
pub fn part1(cs: &HashSet<(i64, i64)>) -> usize {
    solve(cs, false)
}

#[aoc(day17, part2)]
pub fn part2(cs: &HashSet<(i64, i64)>) -> usize {
    solve(cs, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r".#.
..#
###";

    #[test]
    pub fn test_input() {
        // println!("{:?}", neighbors(&[2, 0, 0]).collect::<Vec<_>>());

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 112);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 848);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day17.txt");
        const ANSWERS: (usize, usize) = (348, 2236);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    iter::from_fn,
};

pub struct Point2D(i32, i32);

#[derive(PartialEq, Eq, Hash)]
pub struct CoordN<const N: usize>([i32; N]);

fn neighbors<const N: usize>(n: usize) -> impl Iterator<Item = [i32; N]> {
    let mut stack = Vec::with_capacity(3_usize.pow(n.try_into().unwrap()));
    stack.push(([0; N], 0));

    from_fn(move || {
        while let Some((mut current, pos)) = stack.pop() {
            if pos == n {
                return Some(current);
            }

            for i in -1..=1 {
                current[pos] = i;
                stack.push((current, pos + 1));
                current[pos] = 0;
            }
        }

        None
    })
}

fn tick<const N: usize>(state: HashSet<CoordN<N>>) -> HashSet<CoordN<N>> {
    let mut counts = HashMap::with_capacity(state.iter().next().unwrap().0.len());

    for coord in state.iter() {
        for mut neighbor in neighbors(coord.0.len()) {
            for (n, c) in neighbor.iter_mut().zip(coord.0.iter()) {
                *n += c
            }

            *counts.entry(neighbor).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .filter_map(|(coord, count)| {
            let current = CoordN(coord);

            // one more because we're including ourself
            if count == 3 || count == 4 && state.contains(&current) {
                Some(current)
            } else {
                None
            }
        })
        .collect()
}

fn solve<const N: usize>(points: &[Point2D]) -> usize {
    let mut state = points
        .iter()
        .map(|&Point2D(y, x)| {
            let mut point = [0; N];
            point[0] = y;
            point[1] = x;
            CoordN(point)
        })
        .collect();

    for _ in 0..6 {
        state = tick(state);
    }

    state.len()
}

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Vec<Point2D> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Point2D(y.try_into().unwrap(), x.try_into().unwrap()))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &[Point2D]) -> usize {
    solve::<3>(input)
}

#[aoc(day17, part2)]
pub fn part2(cs: &[Point2D]) -> usize {
    solve::<4>(cs)
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

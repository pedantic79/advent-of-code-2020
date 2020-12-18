use std::{
    collections::{HashMap, HashSet},
    iter::from_fn,
};

pub struct Point2D(i32, i32);

#[derive(PartialEq, Eq, Hash)]
pub struct CoordN(Vec<i32>);

fn neighbors(n: usize) -> impl Iterator<Item = Vec<i32>> {
    let mut stack = Vec::with_capacity(3usize.pow(n as u32));
    stack.push(Vec::new());

    from_fn(move || {
        while let Some(mut current) = stack.pop() {
            if current.len() == n {
                return Some(current);
            }

            for i in -1..=1 {
                current.push(i);
                stack.push(current.clone());
                current.pop();
            }
        }

        None
    })
}

fn tick(state: HashSet<CoordN>) -> HashSet<CoordN> {
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

fn solve(points: &[Point2D], dimensions: usize) -> usize {
    let mut state = points
        .iter()
        .map(|&Point2D(y, x)| {
            let mut point = vec![y, x];
            point.resize(dimensions, 0);
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
                    Some(Point2D(y as i32, x as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &[Point2D]) -> usize {
    solve(input, 3)
}

#[aoc(day17, part2)]
pub fn part2(cs: &[Point2D]) -> usize {
    solve(cs, 4)
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

use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, VecDeque},
    hash::{Hash, Hasher},
};

use nohash_hasher::IntSet;

#[derive(Debug, PartialEq)]
pub struct Players(VecDeque<usize>, VecDeque<usize>);

fn parse_player(input: &str) -> VecDeque<usize> {
    let mut line = input.lines();
    assert!(line.next().unwrap().starts_with("Player "));
    line.map(|l| l.parse().unwrap()).collect()
}

fn get_hash<T: Hash>(deque: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    deque.hash(&mut hasher);
    hasher.finish()
}

#[aoc_generator(day22)]
pub fn generator(input: &str) -> Players {
    let mut section = input.split("\n\n");

    let player1 = parse_player(section.next().unwrap());
    let player2 = parse_player(section.next().unwrap());
    Players(player1, player2)
}

#[aoc(day22, part1)]
pub fn part1(inputs: &Players) -> usize {
    let Players(player1, player2) = inputs;

    let mut player1 = player1.clone();
    let mut player2 = player2.clone();

    while let (Some(&p1), Some(&p2)) = (player1.front(), player2.front()) {
        player1.pop_front();
        player2.pop_front();

        let result = match p1.cmp(&p2) {
            Ordering::Less => false,
            Ordering::Equal => panic!("cards"),
            Ordering::Greater => true,
        };

        if result {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }

    if player1.is_empty() { player2 } else { player1 }
        .iter()
        .copied()
        .rev()
        .zip(1..)
        .map(|(n, i)| i * n)
        .sum()
}

fn solve2(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) -> usize {
    let mut seen1 = IntSet::default();
    // let mut seen2 = IntSet::default();

    let mut loop_count = 0;
    while let (Some(&p1), Some(&p2)) = (player1.front(), player2.front()) {
        // Repeated occurance optimization. Hashing is quite expensive, so we
        // try to avoid hashing every time through.
        if loop_count % 4 == 0 && !seen1.insert(get_hash(&player1)) {
            return 1;
        }
        loop_count += 1;

        // if !seen1.insert(get_hash(&player1)) && !seen2.insert(get_hash(&player2)) {
        //     return 1;
        // }

        player1.pop_front();
        player2.pop_front();

        let result = if p1 <= player1.len() && p2 <= player2.len() {
            let mut p1_copy = player1.clone();
            let mut p2_copy = player2.clone();

            p1_copy.truncate(p1);
            p2_copy.truncate(p2);

            solve2(&mut p1_copy, &mut p2_copy) == 1
        } else {
            match p1.cmp(&p2) {
                Ordering::Less => false,
                Ordering::Equal => panic!("cards equal"),
                Ordering::Greater => true,
            }
        };

        if result {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }

    if player1.is_empty() {
        2
    } else {
        1
    }
}

#[aoc(day22, part2)]
pub fn part2(inputs: &Players) -> usize {
    let Players(player1, player2) = inputs;

    let mut player1 = player1.clone();
    let mut player2 = player2.clone();

    let ans = solve2(&mut player1, &mut player2);

    if ans == 1 { player1 } else { player2 }
        .iter()
        .copied()
        .rev()
        .zip(1..)
        .map(|(n, i)| i * n)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    pub fn test_input() {
        let p1 = [9, 2, 6, 3, 1].iter().copied().collect();
        let p2 = [5, 8, 4, 7, 10].iter().copied().collect();
        assert_eq!(generator(SAMPLE), Players(p1, p2));
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 306);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 291);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day22.txt");
        const ANSWERS: (usize, usize) = (33098, 35055);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

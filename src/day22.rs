use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::{HashSet, VecDeque},
};

#[derive(Debug, PartialEq)]
pub struct Players(VecDeque<usize>, VecDeque<usize>);

fn parse_player(input: &str) -> VecDeque<usize> {
    let mut line = input.lines();
    line.next();
    line.map(|l| l.parse().unwrap()).collect()
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
        match p1.cmp(&p2) {
            Less => {
                player2.push_back(p2);
                player2.push_back(p1);
            }
            Equal => {
                panic!("What happens when they're equal")
            }
            Greater => {
                player1.push_back(p1);
                player1.push_back(p2);
            }
        }
        player1.pop_front();
        player2.pop_front();
    }

    // println!("{:?} {:?}", player1, player2);

    let player = if player1.is_empty() { player2 } else { player1 };

    player
        .iter()
        .copied()
        .rev()
        .enumerate()
        .map(|(i, n)| (i + 1) * n)
        .sum()
}

fn solve2(
    player1: &mut VecDeque<usize>,
    player2: &mut VecDeque<usize>,
    depth: &mut usize,
) -> usize {
    let mut seen1 = HashSet::new();
    let mut seen2 = HashSet::new();

    *depth += 1;
    // println!(
    //     "Start of new game {:?} {:?} {} {}",
    //     player1,
    //     player2,
    //     seen1.len(),
    //     seen2.len()
    // );

    while let (Some(&p1), Some(&p2)) = (player1.front(), player2.front()) {
        // println!("* {:?} {:?}", player1, player2);

        if seen1.contains(player1) && seen2.contains(player2) {
            // println!("SEEN");
            return 1;
        } else {
            seen1.insert(player1.clone());
            seen2.insert(player2.clone());
        }

        player1.pop_front();
        player2.pop_front();

        if p1 <= player1.len() && p2 <= player2.len() {
            let mut p1_copy = player1.clone();
            let mut p2_copy = player2.clone();
            // println!("draining: {}.., {}..", p1, p2);
            p1_copy.drain(p1..);
            p2_copy.drain(p2..);
            // println!("drained >{:?} {:?}", p1_copy, p2_copy);

            if solve2(&mut p1_copy, &mut p2_copy, depth) == 1 {
                player1.push_back(p1);
                player1.push_back(p2);
            } else {
                player2.push_back(p2);
                player2.push_back(p1);
            }
        } else {
            // println!("{} {:?} {} {:?}", p1, player1, p2, player2);

            match p1.cmp(&p2) {
                Less => {
                    player2.push_back(p2);
                    player2.push_back(p1);
                }
                Equal => {
                    panic!("What happens when they're equal")
                }
                Greater => {
                    player1.push_back(p1);
                    player1.push_back(p2);
                }
            }
        }
    }

    // dbg!(if player1.is_empty() { 2 } else { 1 })
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

    let ans = solve2(&mut player1, &mut player2, &mut 0);

    // println!("{} {:?} {:?}", ans, player1, player2);

    let player = if ans == 1 { player1 } else { player2 };

    player
        .iter()
        .copied()
        .rev()
        .enumerate()
        .map(|(i, n)| (i + 1) * n)
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
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Players());
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

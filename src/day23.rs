use std::time::Instant;

#[derive(Debug, PartialEq)]
pub struct Cups {}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect()
}

#[aoc(day23, part1)]
pub fn part1(inputs: &[usize]) -> String {
    let len = inputs.len();
    let mut cups = inputs.to_vec();

    // let mut pos = 0;
    let mut last = None;

    for mv in 0.. {
        println!("\n-- move {} --", mv + 1);
        let (mut pos, current) = if let Some(l) = last {
            // println!("searching for {}", l);
            let l_pos = cups.iter().position(|&x| x == l).unwrap();

            last = Some(cups[(l_pos + 1) % len]);
            (l_pos + 1, last.unwrap())
        } else {
            last = Some(cups[0]);
            (0, cups[0])
        };

        println!("cups ({}) {:?}", current, cups);

        if mv == 100 {
            let current_pos = cups.iter().position(|&x| x == 1).unwrap();
            cups.rotate_left(current_pos);

            return cups[1..]
                .iter()
                .map(|&d| std::char::from_digit(d as u32, 10).unwrap())
                .collect();
        }

        if pos + 4 > len {
            cups.rotate_left(4);
            pos -= 4;
        }

        let mut iter = cups.drain((pos + 1)..(pos + 4));

        let three_cups = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];
        drop(iter);
        println!("picks up: {:?}", three_cups);

        let mut dest = current - 1;
        if dest == 0 {
            dest = len;
        }

        while three_cups.contains(&dest) {
            dest -= 1;

            if dest == 0 {
                dest = len;
            }
        }
        println!("destination: {}", dest);
        let dest_index = cups.iter().position(|&x| x == dest).unwrap();

        if mv == 3 {
            println!("{:?}", cups);
            println!("inserting at pos: {}", dest_index);
        }

        cups.insert((dest_index + 1) % (cups.len() + 1), three_cups[0]);
        if mv == 3 {
            println!("{:?}", cups);
        }
        cups.insert((dest_index + 2) % (cups.len() + 1), three_cups[1]);
        if mv == 3 {
            println!("{:?}", cups);
        }
        cups.insert((dest_index + 3) % (cups.len() + 1), three_cups[2]);
        if mv == 3 {
            println!("{:?}", cups);
        }
    }

    unreachable!()
}

#[aoc(day23, part2)]
pub fn part2(inputs: &[usize]) -> String {
    let len = 1_000_000;
    let mut cups = inputs
        .iter()
        .copied()
        .chain((10..=1_000_000).into_iter())
        .collect::<Vec<_>>();

    // let mut pos = 0;
    let mut last = None;

    let start = Instant::now();
    for mv in 0.. {
        if mv % 1024 == 0 {
            println!(
                "\n-- move {} {:?} --",
                mv + 1,
                Instant::now().duration_since(start)
            );
        }
        let (mut pos, current) = if let Some(l) = last {
            // println!("searching for {}", l);
            let l_pos = cups.iter().position(|&x| x == l).unwrap();

            last = Some(cups[(l_pos + 1) % len]);
            (l_pos + 1, last.unwrap())
        } else {
            last = Some(cups[0]);
            (0, cups[0])
        };

        // println!("cups ({}) {:?}", current, cups);

        if mv == 10_000_000 {
            let current_pos = cups.iter().position(|&x| x == 1).unwrap();
            cups.rotate_left(current_pos);

            return cups[1].to_string() + &cups[2].to_string();
        }

        if pos + 4 > len {
            cups.rotate_left(4);
            pos -= 4;
        }

        let mut iter = cups.drain((pos + 1)..(pos + 4));

        let three_cups = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];
        drop(iter);
        // println!("picks up: {:?}", three_cups);

        let mut dest = current - 1;
        if dest == 0 {
            dest = len;
        }

        while three_cups.contains(&dest) {
            dest -= 1;

            if dest == 0 {
                dest = len;
            }
        }
        // println!("destination: {}", dest);
        let dest_index = cups.iter().position(|&x| x == dest).unwrap();

        cups.insert((dest_index + 1) % (cups.len() + 1), three_cups[0]);
        cups.insert((dest_index + 2) % (cups.len() + 1), three_cups[1]);
        cups.insert((dest_index + 3) % (cups.len() + 1), three_cups[2]);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "389125467";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        assert_eq!(generator(SAMPLE), vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
    }

    #[test]
    // #[ignore]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), "67384529");
    }

    #[test]
    #[ignore]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), "149245887792");
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day23.txt");
        const ANSWERS: (usize, usize) = (32658947, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            // assert_eq!(part1(&generator(input)), ANSWERS.0);
            // assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

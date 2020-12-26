#[derive(Debug, PartialEq)]
enum BusLine {
    Empty,
    ID(usize),
}

impl From<&str> for BusLine {
    fn from(i: &str) -> Self {
        if i == "x" {
            BusLine::Empty
        } else {
            BusLine::ID(i.parse().unwrap())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BusSchedule {
    start: usize,
    ids: Vec<BusLine>,
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> BusSchedule {
    let mut iter = input.lines();

    let start = iter.next().unwrap().parse().unwrap();
    let ids = iter.next().unwrap().split(',').map(|x| x.into()).collect();

    BusSchedule { start, ids }
}

#[aoc(day13, part1)]
pub fn part1(schedule: &BusSchedule) -> usize {
    for i in schedule.start.. {
        for id in schedule.ids.iter() {
            if let BusLine::ID(bus_id) = id {
                if i % bus_id == 0 {
                    return (i - schedule.start) * *bus_id;
                }
            }
        }
    }

    unreachable!()
}

#[aoc(day13, part2)]
pub fn part2(schedule: &BusSchedule) -> usize {
    /*
    Let's take the input to be: x,17,13,19

    Our first bus has frequency 17, with an offset of 1.
    This means that the answer for this frequency is 16 to have the congruence be true
    (x + offset) % frequency == 0 i.e. (16 + 1) % 17 == 0

    This is done via a naïve modular inverse, where we step by 1.
    e.g. for every number 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, etc.
         we do (x + 1) % 17 and check if that is 0. The first value where that
         is true is 16.

    Our second bus has frequency 13, with an offset of 2. For the intial congruence
    to remain true we need to step by 17.

    16, 33, 50, 67, etc.

    These numbers are all (number + 1) % 17 == 0.

    Using this list of numbers, we need to find a number we need to find the first one
    that the congruence to be. i.e (x + 2) % 13 == 0 is true. That number is 50.
    (50 + 1) % 17 == 0 && (50 + 2) % 13 == 0

    Our third bus has frequency 19, with an offset of 3, For the first two congruence
    to remain true, we need to step by 17 * 13, so that this remains true.
    forall.x, (x + 1) % 17 == 0 && (x + 2) % 13 == 0

    50, 271, 492, 713, 934, 1155, 1376, 1597, 1818, 2039, 2260, 2481, 2702, etc.
    See: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c831f5b1a07cbff938f718fdf1d6959d

    We're searching for a number that is also (x + 3) % 19 == 0

    Which is 2923, our answer. We can check that.

    (2923 + 1) % 17 == 0 && (2923 + 2) % 13 == 0 && (2923 + 3) % 19 == 0
    */

    let mut step = 1;
    let mut time = 0;

    for (offset, bl) in schedule.ids.iter().enumerate() {
        if let BusLine::ID(modulo) = bl {
            while (time + offset) % modulo != 0 {
                // println!("time: {}", time);
                time += step;
            }

            step *= modulo;
            // println!("sum: {}, step: {}", time, step);
        }
    }
    time
}

#[aoc(day13, part2, crt)]
pub fn part2_crt(schedule: &BusSchedule) -> usize {
    crate::chinese_remainder_theorem(schedule.ids.iter().enumerate().filter_map(
        |(idx, bus_line)| {
            if let BusLine::ID(bl) = bus_line {
                Some((bl - idx, *bl))
            } else {
                None
            }
        },
    ))
}

#[aoc(day13, part2, brute)]
pub fn part2_brute(_schedule: &BusSchedule) -> usize {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    const START: usize = 407; // MAX - OFFSET
    const STEP: usize = 457; // MAX

    #[inline(always)]
    fn check_mod(m: usize, n: usize) -> bool {
        n % m == 0
    }

    // 644_101_264_100 * 457 + 407 is the right answer
    (644_101_200_000..=usize::MAX)
        .into_par_iter()
        .find_first(|multiplier| {
            let answer = STEP * multiplier + START;

            check_mod(13, answer + 32)
                && check_mod(17, answer + 67)
                && check_mod(19, answer)
                && check_mod(23, answer + 27)
                && check_mod(29, answer + 48)
                && check_mod(37, answer + 13)
                && check_mod(41, answer + 60)
                && check_mod(383, answer + 19)
                && check_mod(457, answer + 50)
        })
        .unwrap()
        * STEP
        + START
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"939
7,13,x,x,59,x,31,19";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        assert_eq!(
            generator(SAMPLE),
            BusSchedule {
                start: 939,
                ids: vec![
                    BusLine::ID(7),
                    BusLine::ID(13),
                    BusLine::Empty,
                    BusLine::Empty,
                    BusLine::ID(59),
                    BusLine::Empty,
                    BusLine::ID(31),
                    BusLine::ID(19),
                ]
            }
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 295);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator("0\nx,17,13,19")), 2923);
        assert_eq!(part2_crt(&generator(SAMPLE)), 1068781);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day13.txt");
        const ANSWERS: (usize, usize) = (1915, 294_354_277_694_107);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

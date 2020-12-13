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
    This means that the sum for this number is 16 to have the congruence be true
    (sum + offset) % frequency == 0 i.e. (16 + 1) % 17 == 0

    This is done via a naïve modular inverse, where we step by 1 (step at this time).

    Our second bus has frequency 13, with an offset of 2. For the intial congruence
    to remain true we need to step by 17.

    16, 33, 50, 67, etc.

    We are also looking for the congruence to be. i.e (sum + 2) % 13 == 0
    The first number were that is true is 50.

    Our third bus has frequency 19, with an offset of 3, For the first two congruence
    to remain true, we need to step by lcm(17 * 13), so that this remains true.
    forall.x, (x + 1) % 17 == 0 && (x + 2) % 13 == 0


    50, 271, 492, 713, 934, 1155, 1376, 1597, 1818, 2039, 2260, 2481, 2702, etc.
    See: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c831f5b1a07cbff938f718fdf1d6959d

    we're searching for a number that is also (x + 3) % 19 == 0

    Which is 2923, our answer.
    */

    let mut step = 1;
    let mut sum = 0;

    for (offset, bl) in schedule.ids.iter().enumerate() {
        if let BusLine::ID(modulo) = bl {
            while (sum + offset) % modulo != 0 {
                // println!("sum: {}", sum);
                sum += step;
            }

            // Since all the numbers are co-prime, this isn't necessary
            // step = step.lcm(bus_id);
            step *= modulo;
            // println!("sum: {}, step: {}", sum, step);
        }
    }
    sum
}

#[aoc(day13, part2, crt)]
pub fn part2_crt(schedule: &BusSchedule) -> usize {
    let product = schedule
        .ids
        .iter()
        .map(|bl| if let BusLine::ID(x) = bl { *x } else { 1 })
        .product::<usize>();

    let mut sum = 0;
    for (i, bl) in schedule.ids.iter().enumerate() {
        if let BusLine::ID(m) = bl {
            let a = product / m;
            let y = crate::mod_inv_unsigned(a, *m);
            sum += (m - i) * a * y;
        }
    }
    sum % product
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
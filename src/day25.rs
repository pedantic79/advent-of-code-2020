#[derive(Debug, PartialEq)]
pub struct Object {}

#[aoc_generator(day25)]
pub fn generator(input: &str) -> (u64, u64) {
    let mut iter = input.lines();

    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}

fn loop_size(target: u64) -> usize {
    let subject = 7;
    let mut count = 0;
    let mut value = 1;

    while target != value {
        count += 1;
        value *= subject;
        value %= 20201227;
    }

    count
}

fn encryption_key(public_key: u64, loop_count: usize) -> u64 {
    let mut value = 1;

    for _ in 0..loop_count {
        value *= public_key;
        value %= 20201227;
    }

    value
}

#[aoc(day25, part1)]
pub fn part1((card, door): &(u64, u64)) -> u64 {
    encryption_key(*card, loop_size(*door))
}

// #[aoc(day25, part2)]
// pub fn part2((card, door): &(u64, u64)) -> u64 {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "5764801\n17807724";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        assert_eq!(generator(SAMPLE), (5764801, 17807724));
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 14897079);
    }

    #[test]
    pub fn test2() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day25.txt");
        const ANSWERS: (u64, usize) = (1478097, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            // assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

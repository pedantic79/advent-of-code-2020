#[derive(Debug, PartialEq)]
pub struct Object {}

#[aoc_generator(dayN)]
pub fn generator(input: &str) -> Object {
    unimplemented!()
}

#[aoc(dayN, part1)]
pub fn part1(inputs: &Object) -> usize {
    unimplemented!()
}

#[aoc(dayN, part2)]
pub fn part2(inputs: &Object) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"..##.......";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 7);
    }

    // #[test]
    // pub fn test2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 336);
    // }

    // mod regression {
    //     use super::*;

    //     const INPUT: &str = include_str!("../input/2020/dayN.txt");
    //     const ANSWERS: (usize, usize) = (0, 0);

    //     #[test]
    //     pub fn test() {
    //         let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
    //         assert_eq!(part1(&generator(input)), ANSWERS.0);
    //         assert_eq!(part2(&generator(input)), ANSWERS.1);
    //     }
    // }
}

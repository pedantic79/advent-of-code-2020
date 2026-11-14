use aoc_harness::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct Input {}

#[aoc_generator(day@DAY@)]
pub fn generator(_input: &str) -> Input {
    todo!()
}

#[aoc(day@DAY@, part1)]
pub fn part1(_input: &Input) -> usize {
    todo!()
}

#[aoc(day@DAY@, part2)]
pub fn part2(_input: &Input) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "";

    #[test]
    #[ignore = "fill in the sample and its part 1 answer"]
    fn part1_test() {
        let input = generator(SAMPLE);

        assert_eq!(part1(&input), 0);
    }

    #[test]
    #[ignore = "fill in the sample and its part 2 answer"]
    fn part2_test() {
        let input = generator(SAMPLE);

        assert_eq!(part2(&input), 0);
    }

    mod regression {
        use super::*;

        const ANSWERS: (usize, usize) = (0, 0);

        #[test]
        #[ignore = "fill in both confirmed answers"]
        fn test() {
            let input = generator(&aoc_harness::input!(@DAY@));

            assert_eq!(part1(&input), ANSWERS.0);
            assert_eq!(part2(&input), ANSWERS.1);
        }
    }
}

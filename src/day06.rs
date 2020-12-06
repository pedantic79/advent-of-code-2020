use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(inputs: &[Vec<String>]) -> usize {
    inputs
        .iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|s| s.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(inputs: &[Vec<String>]) -> usize {
    inputs
        .iter()
        .map(|group| {
            let mut tally = [0; 26];

            for person in group {
                for ans in person.chars() {
                    let idx = ((ans as u8) - b'a') as usize;
                    tally[idx] += 1;
                }
            }

            tally.iter().filter(|&&ans| ans == group.len()).count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    pub fn test_input() {
        fn to_owned(input: &[&[&str]]) -> Vec<Vec<String>> {
            input
                .iter()
                .map(|row| row.iter().map(|s| s.to_string()).collect())
                .collect()
        }

        assert_eq!(
            generator(SAMPLE),
            to_owned(
                &[
                    &["abc"][..],
                    &["a", "b", "c"][..],
                    &["ab", "ac"][..],
                    &["a", "a", "a", "a"][..],
                    &["b"][..]
                ][..]
            )
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 11);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 6);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day6.txt");
        const ANSWERS: (usize, usize) = (6443, 3232);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

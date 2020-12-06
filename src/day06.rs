#[derive(Debug, PartialEq)]
pub struct Group {
    people: Vec<String>,
}

impl Group {
    pub fn new(people: Vec<String>) -> Self {
        Self { people }
    }

    pub fn count_vote<F: Fn(usize) -> bool>(&self, predicate: F) -> usize {
        let mut tally = [0; 26];

        for person in self.people.iter() {
            for vote in person.bytes() {
                let idx = vote - b'a';
                tally[idx as usize] += 1;
            }
        }

        tally.iter().filter(|&&n| predicate(n)).count()
    }

    pub fn unique_votes_hs(&self) -> usize {
        use std::collections::HashSet;

        self.people
            .iter()
            .flat_map(|ballot| ballot.bytes())
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn intersected_votes_hm(&self) -> usize {
        use std::collections::HashMap;

        let tally = self.people.iter().flat_map(|ballot| ballot.bytes()).fold(
            HashMap::new(),
            |mut hm, vote| {
                *hm.entry(vote).or_insert(0) += 1;
                hm
            },
        );

        tally
            .values()
            .filter(|&&ans| ans == self.people.len())
            .count()
    }
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Group> {
    input
        .split("\n\n")
        .map(|group| Group::new(group.lines().map(|person| person.to_string()).collect()))
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(inputs: &[Group]) -> usize {
    inputs
        .iter()
        .map(|group| group.count_vote(|ans| ans > 0))
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(inputs: &[Group]) -> usize {
    inputs
        .iter()
        .map(|group| group.count_vote(|ans| ans == group.people.len()))
        .sum()
}

#[aoc(day6, part1, hs)]
pub fn part1_hs(inputs: &[Group]) -> usize {
    inputs.iter().map(|group| group.unique_votes_hs()).sum()
}

#[aoc(day6, part2, hm)]
pub fn part2_hm(inputs: &[Group]) -> usize {
    inputs
        .iter()
        .map(|group| group.intersected_votes_hm())
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
        fn to_owned(input: &[&[&str]]) -> Vec<Group> {
            input
                .iter()
                .map(|row| Group::new(row.iter().map(|s| s.to_string()).collect()))
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

#[derive(PartialEq, Debug)]
pub struct Policy {
    letter: char,
    left: usize,
    right: usize,
}

impl Policy {
    fn validate_part1(&self, input: &str) -> bool {
        let count = input.chars().filter(|&c| c == self.letter).count();
        self.left <= count && count <= self.right
    }

    fn validate_part2(&self, input: &str) -> bool {
        [self.left, self.right]
            .iter()
            .filter(|&&i| input.chars().nth(i - 1) == Some(self.letter))
            .count()
            == 1
    }
}

#[aoc_generator(day2)]
pub fn day2_generator(input: &str) -> Option<Vec<(Policy, String)>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<(Policy, String)> {
    let mut iter = line
        .split([':', '-', ' '].as_ref())
        .filter(|s| !s.is_empty());

    Some((
        Policy {
            left: iter.next()?.parse().ok()?,
            right: iter.next()?.parse().ok()?,
            letter: iter.next()?.chars().next()?,
        },
        iter.next()?.to_owned(),
    ))
}

#[aoc(day2, part1)]
pub fn part1(inputs: &[(Policy, String)]) -> usize {
    inputs.iter().filter(|(p, s)| p.validate_part1(s)).count()
}

#[aoc(day2, part2)]
pub fn part2(inputs: &[(Policy, String)]) -> usize {
    inputs.iter().filter(|(p, s)| p.validate_part2(s)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    fn sample_input() -> Vec<(Policy, String)> {
        day2_generator(SAMPLE).unwrap()
    }

    #[test]
    pub fn input_test() {
        assert_eq!(
            sample_input(),
            vec![
                (
                    Policy {
                        letter: 'a',
                        left: 1,
                        right: 3
                    },
                    "abcde".to_owned()
                ),
                (
                    Policy {
                        letter: 'b',
                        left: 1,
                        right: 3
                    },
                    "cdefg".to_owned()
                ),
                (
                    Policy {
                        letter: 'c',
                        left: 2,
                        right: 9
                    },
                    "ccccccccc".to_owned()
                )
            ]
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&sample_input()), 2)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&sample_input()), 1)
    }
}

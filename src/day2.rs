#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Policy {
    letter: char,
    left: usize,
    right: usize,
}

impl Policy {
    fn validate_part1(self, input: &str) -> bool {
        let count = input.chars().filter(|&c| c == self.letter).count();
        self.left <= count && count <= self.right
    }

    fn validate_part2(self, input: &str) -> bool {
        input
            .chars()
            .enumerate()
            .filter(|&(i, _)| i == self.left - 1 || i == self.right - 1)
            .filter(|&(_, x)| x == self.letter)
            .count()
            == 1
    }
}

#[aoc_generator(day2)]
pub fn day2_generator(input: &str) -> Vec<(Policy, String)> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (Policy, String) {
    let input = line
        .split(|c| ":- ".contains(c))
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    (
        Policy {
            letter: input[2].chars().next().unwrap(),
            left: input[0].parse().unwrap(),
            right: input[1].parse().unwrap(),
        },
        input[3].to_owned(),
    )
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

    #[test]
    pub fn input_test() {
        assert_eq!(
            day2_generator(SAMPLE),
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
        assert_eq!(part1(&day2_generator(SAMPLE)), 2)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&day2_generator(SAMPLE)), 1)
    }
}

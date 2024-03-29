use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Rule {
    Char(char),
    Subrule(Vec<Vec<usize>>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Input {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

fn starts_with(s: &str, c: char) -> Option<&str> {
    if s.starts_with(c) {
        Some(&s[1..])
    } else {
        None
    }
}

impl Input {
    fn solve_other<'a>(&'a self, message: &'a str, rule: &[usize]) -> Vec<&'a str> {
        rule.iter().fold(vec![message], |acc, rule| {
            acc.iter()
                .flat_map(|message| self.solve(message, *rule))
                .collect()
        })
    }

    fn solve<'a>(&'a self, message: &'a str, rule: usize) -> Vec<&'a str> {
        if message.is_empty() {
            return Vec::new();
        }

        match &self.rules[&rule] {
            Rule::Char(c) => starts_with(message, *c).map_or_else(Vec::new, |m| vec![m]),
            Rule::Subrule(v) => v
                .iter()
                .flat_map(|rule| self.solve_other(message, rule))
                .collect(),
        }
    }

    fn check(&self, message: &str, rule_no: usize) -> bool {
        self.check_message(message, rule_no)
            .iter()
            .any(|x| x.is_empty())
    }

    fn check_sub<'a>(&self, message: &'a str, rules: &[usize]) -> Vec<&'a str> {
        let mut acc = vec![message];

        for rule in rules {
            let new_m = acc
                .iter()
                .flat_map(|message| self.check_message(message, *rule))
                .collect();
            acc = new_m;
        }

        acc
    }

    fn check_message<'a>(&self, message: &'a str, rule_no: usize) -> Vec<&'a str> {
        if message.is_empty() {
            return vec![];
        }

        match &self.rules.get(&rule_no) {
            None => vec![],
            Some(Rule::Char(c)) => {
                if let Some(rest) = starts_with(message, *c) {
                    vec![rest]
                } else {
                    vec![]
                }
            }
            Some(Rule::Subrule(v)) => v
                .iter()
                .flat_map(|irl| self.check_sub(message, irl))
                .collect::<Vec<_>>(),
        }
    }
}

fn parse_rule_line(line: &str) -> (usize, Rule) {
    let mut colon = line.split(": ");

    let n = colon.next().unwrap().parse::<usize>().unwrap();
    let rest = colon.next().unwrap();

    let rule = if rest.starts_with('"') {
        Rule::Char(rest.chars().nth(1).unwrap())
    } else {
        Rule::Subrule(
            rest.split(" | ")
                .map(|nums| nums.split(' ').map(|n| n.trim().parse().unwrap()).collect())
                .collect(),
        )
    };

    (n, rule)
}

fn parse_rules(input: &str) -> HashMap<usize, Rule> {
    input.lines().map(parse_rule_line).collect()
}

fn parse_messages(input: &str) -> Vec<String> {
    input.lines().map(|x| x.chars().collect()).collect()
}

#[aoc_generator(day19)]
pub fn generator(input: &str) -> Input {
    let mut section = input.split("\n\n");

    let rules = parse_rules(section.next().unwrap());
    let messages = parse_messages(section.next().unwrap());
    Input { rules, messages }

    //19min
}

#[aoc(day19, part1)]
pub fn part1(inputs: &Input) -> usize {
    inputs
        .messages
        .iter()
        .filter(|message| inputs.solve(message, 0).iter().any(|v| v.is_empty()))
        .count()
}

#[aoc(day19, part1, orig)]
pub fn part1_orig(inputs: &Input) -> usize {
    inputs
        .messages
        .iter()
        .filter(|x| inputs.check(x, 0))
        .count()
}

#[aoc(day19, part2)]
pub fn part2(inputs: &Input) -> usize {
    let mut inputs = inputs.clone();
    inputs
        .rules
        .insert(8, Rule::Subrule(vec![vec![42], vec![42, 8]]));
    inputs
        .rules
        .insert(11, Rule::Subrule(vec![vec![42, 31], vec![42, 11, 31]]));

    inputs
        .messages
        .iter()
        .filter(|message| inputs.solve(message, 0).iter().any(|v| v.is_empty()))
        .count()
}

#[aoc(day19, part2, orig)]
pub fn part2_orig(inputs: &Input) -> usize {
    let mut inputs = inputs.clone();
    inputs
        .rules
        .insert(8, Rule::Subrule(vec![vec![42], vec![42, 8]]));
    inputs
        .rules
        .insert(11, Rule::Subrule(vec![vec![42, 31], vec![42, 11, 31]]));

    inputs
        .messages
        .iter()
        .filter(|message| inputs.check(message, 0))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    // baabbbbbabbbbbbaabaaabaaa

    const SAMPLE2: &str = r#"0: 8 11
1: "a"
2: 1 24 | 14 4
3: 5 14 | 16 1
4: 1 1
5: 1 14 | 15 1
6: 14 14 | 1 14
7: 14 5 | 1 21
8: 42 | 42 8
9: 14 27 | 1 26
10: 23 14 | 28 1
11: 42 31 | 2 11 31
12: 24 14 | 19 1
13: 14 3 | 1 12
14: "b"
15: 1 | 14
16: 15 1 | 14 14
17: 14 2 | 1 7
18: 15 15
19: 14 1 | 14 14
20: 14 14 | 1 15
21: 14 1 | 1 14
22: 14 14
23: 25 1 | 22 14
24: 14 1
25: 1 1 | 1 14
26: 14 22 | 1 20
27: 1 6 | 14 18
28: 16 1
31: 14 17 | 1 13
42: 9 14 | 10 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 2);
    }

    #[test]
    pub fn test2() {
        let mut inputs = generator(SAMPLE2);
        inputs
            .rules
            .insert(8, Rule::Subrule(vec![vec![42], vec![42, 8]]));
        inputs
            .rules
            .insert(11, Rule::Subrule(vec![vec![42, 31], vec![42, 11, 31]]));

        // println!();

        // dbg!(inputs.check(&"babbbbaabbbbbabbbbbbaabaaabaaa".chars().collect::<Vec<_>>()));
        // dbg!(inputs.check_message(&"baabbbbbabbbbbbaabaaabaaa".chars().collect::<Vec<_>>(), 11,));

        assert_eq!(part2(&inputs), 12);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day19.txt");
        const ANSWERS: (usize, usize) = (233, 396);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

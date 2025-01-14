use std::collections::HashMap;

use arrayvec::ArrayVec;

#[derive(Debug, PartialEq, Clone)]
enum Rule {
    Char(char),
    Subrule(ArrayVec<ArrayVec<usize, 3>, 2>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Input {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

impl Input {
    fn replace_rule(&mut self, key: usize, a: &[usize], b: &[usize]) {
        fn mk_array<T: Clone, const N: usize>(slice: &[T]) -> ArrayVec<T, N> {
            ArrayVec::try_from(slice).unwrap()
        }

        self.rules.insert(
            key,
            Rule::Subrule(ArrayVec::from([mk_array(a), mk_array(b)])),
        );
    }

    fn solve_dfs(&self, message: &str, rules: &mut Vec<usize>) -> bool {
        let Some(last) = rules.pop() else {
            return message.is_empty();
        };

        let res = match &self.rules[&last] {
            Rule::Char(c) => {
                if let Some(m) = message.strip_prefix(*c) {
                    self.solve_dfs(m, rules)
                } else {
                    false
                }
            }
            Rule::Subrule(subrule) => {
                for sb in subrule {
                    let len = rules.len();
                    rules.extend(sb.iter().rev());
                    if self.solve_dfs(message, rules) {
                        return true;
                    }
                    rules.truncate(len);
                    assert_eq!(rules.len(), len);
                }

                false
            }
        };

        if !res {
            rules.push(last);
        }
        res
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
    solve_part1(inputs, |i, m| Input::solve_dfs(i, m, &mut vec![0]))
}

fn solve_part1(inputs: &Input, f: impl Fn(&Input, &str) -> bool) -> usize {
    inputs
        .messages
        .iter()
        .filter(|message| f(inputs, message))
        .count()
}

fn solve_part2(inputs: &Input, f: impl Fn(&Input, &str) -> bool) -> usize {
    let mut inputs = inputs.clone();
    inputs.replace_rule(8, &[42], &[42, 8]);
    inputs.replace_rule(11, &[42, 31], &[42, 11, 31]);

    inputs
        .messages
        .iter()
        .filter(|message| f(&inputs, message))
        .count()
}

#[aoc(day19, part2)]
pub fn part2(inputs: &Input) -> usize {
    solve_part2(inputs, |i, m| Input::solve_dfs(i, m, &mut vec![0]))
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
        inputs.replace_rule(8, &[42], &[42, 8]);
        inputs.replace_rule(11, &[42, 31], &[42, 11, 31]);

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

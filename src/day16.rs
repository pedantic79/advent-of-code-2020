use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub struct TicketRules {
    rules: Vec<[usize; 4]>,
    ticket: Vec<usize>,
    nearby: Vec<Vec<usize>>,
    important: BTreeSet<usize>,
}

impl TicketRules {
    fn find_invalid(&self, ticket: &[usize]) -> Option<usize> {
        ticket.iter().find_map(|field| {
            let f = self.rules.iter().any(|rule| {
                let [one_l, one_r, two_l, two_r] = rule;
                one_l <= field && field <= one_r || two_l <= field && field <= two_r
            });

            if f {
                None
            } else {
                Some(*field)
            }
        })
    }

    fn validate_rule_col(&self, tickets: &[&Vec<usize>], rule: usize, col: usize) -> bool {
        let [one_l, one_r, two_l, two_r] = self.rules[rule];

        tickets
            .iter()
            .map(|ticket| ticket[col])
            .all(|x| one_l <= x && x <= one_r || two_l <= x && x <= two_r)
    }
}

fn parse_rules(section: &str) -> (BTreeSet<usize>, Vec<[usize; 4]>) {
    let mut important = BTreeSet::new();
    let mut rules = vec![];

    for (line_no, line) in section.lines().enumerate() {
        let mut line = line.split(':');

        if line.next().unwrap().starts_with("departure") {
            important.insert(line_no);
        }

        let right = line.next().unwrap();
        let mut row = [0; 4];

        // This could be done collect if [T; 4] implemented FromIterator, but
        // it doesn't.
        for (entry, row_entry) in right
            .split("or")
            .flat_map(|or_split| {
                or_split
                    .trim()
                    .split('-')
                    .map(|x| x.parse::<usize>().unwrap())
            })
            .zip(row.iter_mut())
        {
            *row_entry = entry;
        }

        rules.push(row)
    }

    (important, rules)
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> TicketRules {
    let mut section = input.split("\n\n");

    let (important, rules) = parse_rules(section.next().unwrap());

    let ticket = section
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let lines = section.next().unwrap().lines().skip(1);
    let nearby = lines
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    TicketRules {
        rules,
        ticket,
        nearby,
        important,
    }
}

#[aoc(day16, part1)]
pub fn part1(inputs: &TicketRules) -> usize {
    inputs
        .nearby
        .iter()
        .filter_map(|x| inputs.find_invalid(x))
        .sum()
}

#[aoc(day16, part2)]
pub fn part2(inputs: &TicketRules) -> usize {
    part2_solve(inputs).iter().map(|x| x.1).product()
}

pub fn part2_solve(inputs: &TicketRules) -> Vec<(usize, usize)> {
    let tickets = inputs
        .nearby
        .iter()
        .filter(|x| inputs.find_invalid(x).is_none())
        .collect::<Vec<_>>();

    let rules_len = inputs.rules.len();
    let mut candidates = vec![BTreeSet::new(); rules_len];

    for (rule, cand) in candidates.iter_mut().enumerate() {
        for col in 0..rules_len {
            if inputs.validate_rule_col(tickets.as_slice(), rule, col) {
                cand.insert(col);
            }
        }
    }
    let mut ans = vec![(0, 0); inputs.important.len()];

    while let Some((rule, column)) = candidates.iter().enumerate().find_map(|(p, x)| {
        if x.len() == 1 {
            Some((p, *x.iter().next().unwrap()))
        } else {
            None
        }
    }) {
        for cand in candidates.iter_mut() {
            cand.remove(&column);
        }

        if inputs.important.contains(&rule) {
            ans[rule] = (column, inputs.ticket[column]);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    const SAMPLE2: &str = r"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    pub fn test_input() {
        assert_eq!(
            generator(SAMPLE),
            TicketRules {
                rules: vec![[1, 3, 5, 7], [6, 11, 33, 44], [13, 40, 45, 50]],
                ticket: vec![7, 1, 14],
                nearby: vec![
                    vec![7, 3, 47],
                    vec![40, 4, 50],
                    vec![55, 2, 20],
                    vec![38, 6, 12]
                ],
                important: BTreeSet::new()
            }
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 71);
    }

    #[test]
    pub fn test2() {
        let mut rules = generator(SAMPLE2);
        rules.important = (0..=2usize).collect();

        assert_eq!(part2_solve(&rules), vec![(1, 12), (0, 11), (2, 13)]);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day16.txt");
        const ANSWERS: (usize, usize) = (21071, 3429967441937);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct BagRules {
    rules: HashMap<BagColor, Vec<(BagColor, usize)>>,
}

impl BagRules {
    fn contains(&self, needle: &str, target: &str) -> bool {
        self.rules[needle]
            .iter()
            .any(|(color, _)| color == target || self.contains(color, target))
    }

    fn count_contained(&self, color: &str) -> usize {
        self.rules[color]
            .iter()
            .map(|(c, count)| count * self.count_contained(c))
            .sum::<usize>()
            + 1
    }
}

type BagColor = String;

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        combinator::{complete, map, map_res},
        multi::separated_list1,
        sequence::{pair, terminated, tuple},
        IResult,
    };

    fn word(s: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c.is_alphabetic())(s)
    }

    fn numeric(s: &str) -> IResult<&str, usize> {
        map_res(take_while1(|c: char| c.is_numeric()), |x: &str| {
            x.parse::<usize>()
        })(s)
    }

    fn adjective_color(s: &str) -> IResult<&str, (&str, &str)> {
        tuple((terminated(word, tag(" ")), word))(s)
    }

    fn bag(s: &str) -> IResult<&str, &str> {
        alt((tag("bags"), tag("bag")))(s)
    }

    fn color_bag(s: &str) -> IResult<&str, (&str, &str)> {
        terminated(terminated(adjective_color, tag(" ")), bag)(s)
    }

    fn count_color_bag(s: &str) -> IResult<&str, (usize, &str, &str)> {
        map(
            pair(terminated(numeric, tag(" ")), color_bag),
            |(num, (adj, color))| (num, adj, color),
        )(s)
    }

    fn multiple_color_bag(s: &str) -> IResult<&str, Vec<(usize, &str, &str)>> {
        alt((
            map(tag("no other bags"), |_| vec![]),
            separated_list1(tag(", "), count_color_bag),
        ))(s)
    }

    #[allow(clippy::type_complexity)]
    pub fn rule(line: &str) -> IResult<&str, ((&str, &str), Vec<(usize, &str, &str)>)> {
        complete(terminated(
            pair(terminated(color_bag, tag(" contain ")), multiple_color_bag),
            tag("."),
        ))(line)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sample() {
            assert_eq!(rule("vibrant maroon bags contain 3 dark fuchsia bags, 3 plaid turquoise bags, 1 pale silver bag, 4 shiny cyan bags.").unwrap().1,
            (("vibrant", "maroon"), vec![(3, "dark", "fuchsia"), (3, "plaid", "turquoise"), (1, "pale", "silver"), (4, "shiny", "cyan")]))
        }

        #[test]
        fn multi() {
            assert_eq!(
                count_color_bag("3 dark fuschia bags").unwrap().1,
                (3, "dark", "fuschia")
            )
        }

        #[test]
        fn number() {
            assert_eq!(numeric("3 dark fuschia bags").unwrap().1, 3)
        }
    }
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Option<BagRules> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        let ((adj, color), bag_rules) = parser::rule(line).ok()?.1;

        let entry = rules.entry(format!("{} {}", adj, color)).or_insert(vec![]);
        for (count, adj, color) in bag_rules {
            entry.push((format!("{} {}", adj, color), count))
        }
    }

    Some(BagRules { rules })
}

#[aoc(day7, part1)]
pub fn part1(inputs: &BagRules) -> usize {
    inputs
        .rules
        .iter()
        .filter(|&(color, _)| inputs.contains(color, "shiny gold"))
        .count()
}

#[aoc(day7, part2)]
pub fn part2(inputs: &BagRules) -> usize {
    inputs.count_contained("shiny gold") - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const SAMPLE2: &str = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    pub fn test_input() {
        let rules = [
            ("light red", &[("bright white", 1), ("muted yellow", 2)][..]),
            (
                "dark orange",
                &[("bright white", 3), ("muted yellow", 4)][..],
            ),
            ("bright white", &[("shiny gold", 1)][..]),
            ("muted yellow", &[("shiny gold", 2), ("faded blue", 9)][..]),
            ("shiny gold", &[("dark olive", 1), ("vibrant plum", 2)][..]),
            ("dark olive", &[("faded blue", 3), ("dotted black", 4)][..]),
            (
                "vibrant plum",
                &[("faded blue", 5), ("dotted black", 6)][..],
            ),
            ("faded blue", &[][..]),
            ("dotted black", &[][..]),
        ]
        .iter()
        .map(|&(key, v)| {
            (
                key.to_string(),
                v.iter().map(|&(k, n)| (k.to_string(), n)).collect(),
            )
        })
        .collect::<HashMap<_, _>>();

        assert_eq!(generator(SAMPLE1).unwrap(), BagRules { rules });
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE1).unwrap()), 4);
    }

    #[test]
    pub fn test2() {
        let bags = generator(SAMPLE1).unwrap();

        assert_eq!(bags.count_contained("faded blue"), 1);
        assert_eq!(bags.count_contained("vibrant plum"), 12);
        assert_eq!(bags.count_contained("dark olive"), 8);

        assert_eq!(part2(&generator(SAMPLE1).unwrap()), 32);
        assert_eq!(part2(&generator(SAMPLE2).unwrap()), 126);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day7.txt");
        const ANSWERS: (usize, usize) = (248, 57281);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
            assert_eq!(part1(&generator(input).unwrap()), ANSWERS.0);
            assert_eq!(part2(&generator(input).unwrap()), ANSWERS.1);
        }
    }
}

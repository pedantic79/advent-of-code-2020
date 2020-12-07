use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct ColoredBags {
    bags: HashMap<BagColor, Vec<(BagColor, usize)>>,
}

impl ColoredBags {
    fn contains_shiny_gold_bag(&self, color: &str) -> bool {
        self.bags[color]
            .iter()
            .any(|(color, _)| color == "shiny gold" || self.contains_shiny_gold_bag(color))
    }

    fn count_contained(&self, color: &str) -> usize {
        self.bags[color]
            .iter()
            .map(|(c, count)| count * self.count_contained(c))
            .sum::<usize>()
            + 1
    }
}

type BagColor = String;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Option<ColoredBags> {
    let mut bags = HashMap::new();

    for line in input.lines() {
        let mut parse_iter = line.split(" bags contain ");

        let key = parse_iter.next()?;
        for bag in parse_iter
            .next()?
            .split(&[',', '.'][..])
            .filter(|x| !x.is_empty())
            .map(|x| x.trim())
        {
            let entry = bags.entry(key.to_string()).or_insert(vec![]);
            if bag == "no other bags" {
                break;
            }

            let mut iter = bag.split(' ');
            let num = iter.next()?;
            let adjective = iter.next()?;
            let color = iter.next()?;

            entry.push((format!("{} {}", adjective, color), num.parse().unwrap()));
        }
    }

    Some(ColoredBags { bags })
}

#[aoc(day7, part1)]
pub fn part1(inputs: &ColoredBags) -> usize {
    inputs
        .bags
        .iter()
        .filter(|&(color, _)| inputs.contains_shiny_gold_bag(color))
        .count()
}

#[aoc(day7, part2)]
pub fn part2(inputs: &ColoredBags) -> usize {
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
        // assert_eq!(generator(SAMPLE), ColoredBags());
        generator(SAMPLE1);
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

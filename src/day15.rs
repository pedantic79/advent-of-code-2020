use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

pub fn solve(inputs: &[usize], limit: usize) -> usize {
    let mut seen: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut last = 0;

    for i in 0..limit {
        let val = if i < inputs.len() {
            inputs[i]
        } else if let Some((one_ago, two_ago)) = seen.get(&last) {
            one_ago - two_ago
        } else {
            0
        };

        last = val;
        seen.entry(val)
            .and_modify(|(o, t)| {
                *t = *o;
                *o = i;
            })
            .or_insert((i, i));
    }

    last
}

#[aoc(day15, part1)]
pub fn part1(inputs: &[usize]) -> usize {
    solve(inputs, 2020)
}

#[aoc(day15, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    solve(inputs, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"0,3,6";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        assert_eq!(generator(SAMPLE), vec![0, 3, 6]);
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 436);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 175594);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day15.txt");
        const ANSWERS: (usize, usize) = (1618, 548531);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

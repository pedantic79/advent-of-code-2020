fn bool2num(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .fold(0, |total, c| total * 2 + bool2num(c == b'B' || c == b'R'))
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(inputs: &[usize]) -> usize {
    inputs.iter().copied().max().unwrap()
}

#[aoc(day5 part2, vec)]
pub fn part2_vec(inputs: &[usize]) -> usize {
    let mut set = inputs.to_vec();
    set.sort_unstable();

    for seats in set.windows(2) {
        assert!(seats.len() == 2);
        if seats[0] + 1 != seats[1] {
            return seats[0] + 1;
        }
    }

    unreachable!()
}

#[aoc(day5 part2, array)]
pub fn part2(inputs: &[usize]) -> usize {
    let mut seats = [false; 1024];

    for id in inputs.iter().copied() {
        seats[id] = true
    }

    let mut iter = seats.iter().enumerate();

    while let Some((_, false)) = iter.next() {}
    while let Some((_, true)) = iter.next() {}
    iter.next().unwrap().0 - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    pub fn test_input() {
        assert_eq!(generator(SAMPLE), vec![357, 567, 119, 820]);
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 820);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 120);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day5.txt");
        const ANSWERS: (usize, usize) = (978, 727);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

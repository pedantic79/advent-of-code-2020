#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<usize> {
    let mut adaptors: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();

    adaptors.push(0);
    adaptors.sort_unstable();
    adaptors.push(adaptors.last().unwrap() + 3);

    adaptors
}

#[aoc(day10, part1)]
pub fn part1(inputs: &[usize]) -> usize {
    let mut ones = 0;
    let mut threes = 0;

    for w in inputs.windows(2) {
        match w[1] - w[0] {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    }

    ones * threes
}

#[aoc(day10, part1, iterator)]
pub fn part1_iterator(inputs: &[usize]) -> usize {
    let (ones, threes) =
        inputs
            .windows(2)
            .map(|w| w[1] - w[0])
            .fold((0, 0), |(ones, threes), x| match x {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes),
            });

    ones * threes
}

#[aoc(day10, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    let mut dp = vec![0; inputs.len()];
    dp[0] = 1;

    for (i, input) in inputs.iter().enumerate() {
        for (j, output) in inputs.iter().enumerate().skip(i + 1) {
            if output - input > 3 {
                break;
            }

            dp[j] += dp[i];
        }
    }

    dp.last().copied().unwrap()
}

#[aoc(day10, part2, alt)]
pub fn part2_alt(inputs: &[usize]) -> usize {
    inputs
        .windows(2)
        .fold((1, 0, 0), |(paths_0, paths_1, paths_2), w| {
            match w[1] - w[0] {
                3 => (paths_0, 0, 0),
                1 => (paths_0 + paths_1, paths_0 + paths_2, paths_0),
                _ => unreachable!(),
            }
        })
        .0
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const SAMPLE2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE2));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE1)), 35);
        assert_eq!(part1(&generator(SAMPLE2)), 220);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE1)), 8);
        assert_eq!(part2(&generator(SAMPLE2)), 19208);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day10.txt");
        const ANSWERS: (usize, usize) = (2310, 64_793_042_714_624);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

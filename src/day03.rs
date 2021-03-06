pub struct Forest {
    field: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Forest {
    const TREE: u8 = b'#';

    fn new(field: Vec<Vec<u8>>) -> Self {
        let width = field[0].len();
        let height = field.len();

        Self {
            field,
            width,
            height,
        }
    }

    fn get(&self, c: usize, r: usize) -> bool {
        self.field[r][c] == Self::TREE
    }

    fn count_trees(&self, c_inc: usize, r_inc: usize) -> usize {
        let mut c = 0;

        (0..self.height)
            .step_by(r_inc)
            .filter(|&r| {
                let c_orig = c;

                c += c_inc;
                if c >= self.width {
                    c -= self.width;
                }

                self.get(c_orig, r)
            })
            .count()
    }
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Forest {
    Forest::new(input.lines().map(|l| l.as_bytes().to_owned()).collect())
}

#[aoc(day3, part1)]
pub fn part1(inputs: &Forest) -> usize {
    inputs.count_trees(3, 1)
}

#[aoc(day3, part2)]
pub fn part2(inputs: &Forest) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(c, r)| inputs.count_trees(c, r))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    pub fn test_input() {
        assert_eq!(
            generator(SAMPLE)
                .field
                .iter()
                .map(|row| row
                    .iter()
                    .map(|&x| (x == Forest::TREE) as usize)
                    .fold(0, |acc, digit| acc * 2 + digit))
                .collect::<Vec<_>>(),
            vec![384, 1092, 530, 325, 562, 352, 673, 513, 1416, 1121, 581]
        )
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 7);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day3.txt");
        const ANSWERS: (usize, usize) = (292, 9_354_744_432);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

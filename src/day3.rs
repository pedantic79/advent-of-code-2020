pub struct Map {
    field: Vec<Vec<bool>>,
    width: usize,
}

impl Map {
    fn get(&self, r: usize, c: usize) -> bool {
        let c = c % self.width;

        self.field[r][c]
    }
}

#[aoc_generator(day3)]
pub fn day3_generator(input: &str) -> Map {
    let field: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    let width = field[0].len();

    Map { field, width }
}

pub fn count_trees(inputs: &Map, c_inc: usize, r_inc: usize) -> usize {
    let height = inputs.field.len();
    let mut c = 0;

    (0..height)
        .step_by(r_inc)
        .filter(|&r| {
            let c_orig = c;
            c += c_inc;
            inputs.get(r, c_orig)
        })
        .count()
}

#[aoc(day3, part1)]
pub fn part1(inputs: &Map) -> usize {
    count_trees(inputs, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(inputs: &Map) -> usize {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .iter()
        .map(|(c, r)| count_trees(inputs, *c, *r))
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
    pub fn test1() {
        assert_eq!(part1(&day3_generator(SAMPLE)), 7);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&day3_generator(SAMPLE)), 336);
    }
}

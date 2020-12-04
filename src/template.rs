pub struct Object {}

#[aoc_generator(dayN)]
pub fn generator(input: &str) -> Object {
    unimplemented!()
}

#[aoc(dayN, part1)]
pub fn part1(inputs: &Object) -> i32 {
    unimplemented!()
}

#[aoc(dayN, part2)]
pub fn part2(inputs: &Object) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"..##.......";

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 7);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 336);
    }
}

#[aoc_generator(dayN)]
pub fn dayN_generator(input: &str) -> Vec<i32> {
    unimplemented!()
}

#[aoc(dayN, part1)]
pub fn part1(inputs: &[i32]) -> i32 {
    unimplemented!()
}

#[aoc(dayN, part2)]
pub fn part2(inputs: &[i32]) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE), 1721 * 299)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&SAMPLE), 979 * 366 * 675)
    }
}

use crate::common::utils;

#[aoc_generator(day25)]
pub fn generator(input: &str) -> (u64, u64) {
    let mut iter = input.lines();

    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}

const MOD: u64 = 20_201_227;

fn loop_size(target1: u64, target2: u64) -> (u64, u64) {
    (
        target2,
        utils::baby_step_giant_step(MOD, 7, target1).unwrap(),
    )
}

fn encryption_key((public_key, loop_count): (u64, u64)) -> u64 {
    utils::mod_pow(public_key, loop_count, MOD)
}

#[aoc(day25, part1)]
pub fn part1((card, door): &(u64, u64)) -> u64 {
    encryption_key(loop_size(*card, *door))
}

// #[aoc(day25, part2)]
// pub fn part2((card, door): &(u64, u64)) -> u64 {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "5764801\n17807724";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        assert_eq!(generator(SAMPLE), (5_764_801, 17_807_724));
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 14_897_079);
    }

    // #[test]
    // pub fn test2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 336);
    // }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day25.txt");
        const ANSWERS: (u64, usize) = (1_478_097, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            // assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

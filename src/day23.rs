use std::{
    convert::{TryFrom, TryInto},
    fmt::{Debug, Write},
};

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Vec<u32> {
    input.chars().map(|x| x.to_digit(10).unwrap()).collect()
}

#[allow(dead_code)]
fn to_string<T: num::PrimInt + Debug>(start: T, ring: &[T]) -> String {
    let mut output = String::new();
    write!(output, "[({:?})", start).unwrap();
    let mut i = ring[start.to_usize().unwrap()];

    while i != start {
        write!(output, ", {:?}", i).unwrap();

        i = ring[i.to_usize().unwrap()];
    }

    write!(output, "]").unwrap();
    output
}

fn solve<T>(ring: &mut [T], mut current: T, iteratations: usize)
where
    T: num::PrimInt + Debug,
{
    let len = ring.len() - 1;

    for _mv in 0..iteratations {
        // println!("\n-- move {} --", _mv + 1);
        // println!("cups {}", to_string(current, &ring));

        let mut three_cups = [T::zero(); 3];
        three_cups[0] = ring[current.to_usize().unwrap()];
        three_cups[1] = ring[three_cups[0].to_usize().unwrap()];
        three_cups[2] = ring[three_cups[1].to_usize().unwrap()];
        // println!("picks up: {:?}", three_cups);

        let mut dest = current - T::one();
        if dest == T::zero() {
            dest = T::from(len).unwrap();
        }

        while three_cups.contains(&dest) {
            dest = dest - T::one();

            if dest == T::zero() {
                dest = T::from(len).unwrap();
            }
        }
        // println!("destination: {:?}", dest);

        let next_current = ring[three_cups[2].to_usize().unwrap()];
        let dest_next: T = ring[dest.to_usize().unwrap()];
        ring[current.to_usize().unwrap()] = next_current;
        ring[dest.to_usize().unwrap()] = three_cups[0];
        ring[three_cups[2].to_usize().unwrap()] = dest_next;

        current = next_current;
    }
}

#[aoc(day23, part1)]
pub fn part1(inputs: &[u32]) -> String {
    let mut ring = vec![0; inputs.len() + 1];

    for w in inputs.windows(2) {
        ring[usize::try_from(w[0]).unwrap()] = w[1];
    }
    ring[usize::try_from(inputs[inputs.len() - 1]).unwrap()] = inputs[0];

    solve(&mut ring, inputs[0], 100);

    let mut output = String::new();

    let mut current_pos = ring[1];
    while current_pos != 1 {
        output.push(std::char::from_digit(current_pos, 10).unwrap());
        current_pos = ring[usize::try_from(current_pos).unwrap()];
    }

    output
}

#[aoc(day23, part2)]
pub fn part2(inputs: &[u32]) -> u64 {
    const LEN: usize = 1_000_000;
    let mut ring = vec![0; LEN + 1];

    for w in inputs.windows(2) {
        ring[usize::try_from(w[0]).unwrap()] = w[1];
    }

    ring[usize::try_from(inputs[inputs.len() - 1]).unwrap()] = 10;
    for (i, r) in ring.iter_mut().enumerate().skip(10) {
        *r = (i + 1).try_into().unwrap();
    }
    ring[LEN] = inputs[0];

    solve(&mut ring, inputs[0], 10_000_000);

    let a = ring[1];
    let b = ring[usize::try_from(a).unwrap()];

    u64::from(a) * u64::from(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "389125467";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        assert_eq!(generator(SAMPLE), vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
    }

    #[test]
    // #[ignore]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), "67384529");
    }

    #[test]
    // #[ignore]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 149_245_887_792);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day23.txt");
        const ANSWERS: (&str, u64) = ("32658947", 683_486_010_900);

        #[test]
        // #[ignore]
        pub fn test_1() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
        }

        #[test]
        // #[ignore]
        pub fn test_2() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

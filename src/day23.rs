use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Cups {}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect()
}

fn print(start: usize, ring: &HashMap<usize, usize>) -> String {
    let mut output = String::new();
    write!(output, "[{},", start).unwrap();
    let mut i = ring[&start];

    while i != start {
        write!(output, " {},", i).unwrap();

        i = ring[&i];
    }

    write!(output, "]").unwrap();
    output
}

#[aoc(day23, part1)]
pub fn part1(inputs: &[usize]) -> String {
    let len = inputs.len();
    let mut ring = HashMap::new();

    for w in inputs.windows(2) {
        ring.insert(w[0], w[1]);
    }
    ring.insert(inputs[len - 1], inputs[0]);

    println!("{}", print(inputs[0], &ring));

    let mut current = inputs[0];

    for _mv in 0..100 {
        // println!("\n-- move {} --", _mv + 1);
        // println!("cups ({}) {}", current, print(current, &ring));

        let mut three_cups = [0; 3];
        three_cups[0] = ring[&current];
        three_cups[1] = ring[&three_cups[0]];
        three_cups[2] = ring[&three_cups[1]];
        // println!("picks up: {:?}", three_cups);

        let mut dest = current - 1;
        if dest == 0 {
            dest = len;
        }

        while three_cups.contains(&dest) {
            dest -= 1;

            if dest == 0 {
                dest = len;
            }
        }
        // println!("destination: {}", dest);

        let next_current = ring[&three_cups[2]];
        let dest_next = ring[&dest];

        ring.insert(current, next_current);
        ring.insert(dest, three_cups[0]);
        ring.insert(three_cups[2], dest_next);
        current = next_current;
    }

    let mut current_pos = ring[&1];
    let mut output = String::new();

    while current_pos != 1 {
        output.push(std::char::from_digit(current_pos as u32, 10).unwrap());
        current_pos = ring[&current_pos];
    }

    output
}

#[aoc(day23, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    const LEN: usize = 1_000_000;
    let mut ring = HashMap::new();

    for w in inputs.windows(2) {
        ring.insert(w[0], w[1]);
    }
    ring.insert(inputs[inputs.len() - 1], 10);
    ring.insert(LEN, inputs[0]);

    for i in 10..LEN {
        ring.insert(i, i + 1);
    }

    let mut current = inputs[0];

    for _mv in 0..10_000_000 {
        // println!("\n-- move {} --", _mv + 1);
        // println!("cups ({}) {}", current, print(current, &ring));

        let mut three_cups = [0; 3];
        three_cups[0] = ring[&current];
        three_cups[1] = ring[&three_cups[0]];
        three_cups[2] = ring[&three_cups[1]];
        // println!("picks up: {:?}", three_cups);

        let mut dest = current - 1;
        if dest == 0 {
            dest = LEN;
        }

        while three_cups.contains(&dest) {
            dest -= 1;

            if dest == 0 {
                dest = LEN;
            }
        }
        // println!("destination: {}", dest);

        let next_current = ring[&three_cups[2]];
        let dest_next = ring[&dest];

        ring.insert(current, next_current);
        ring.insert(dest, three_cups[0]);
        ring.insert(three_cups[2], dest_next);
        current = next_current;
    }

    let a = ring[&1];
    let b = ring[&a];

    a * b
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
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 149245887792);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day23.txt");
        const ANSWERS: (&str, usize) = ("32658947", 683486010900);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

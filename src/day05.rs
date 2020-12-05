use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub struct Seat(String, String);

impl Seat {
    fn get_row_col(&self) -> (usize, usize) {
        fn bool2num(b: bool) -> usize {
            b as usize // not a fan of this but it is valid, false => 0, true => 1
        }

        let row = self
            .0
            .bytes()
            .fold(0, |total, c| total * 2 + bool2num(c == b'B'));

        let col = self
            .1
            .bytes()
            .fold(0, |total, c| total * 2 + bool2num(c == b'R'));

        (row, col)
    }

    fn get_id(&self) -> usize {
        let (row, col) = self.get_row_col();
        row * 8 + col
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|line| {
            let (r, c) = line.split_at(7);
            Seat(r.into(), c.into())
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(inputs: &[Seat]) -> usize {
    inputs.iter().map(|seat| seat.get_id()).max().unwrap()
}

#[aoc(day5 part2)]
pub fn part2(inputs: &[Seat]) -> usize {
    let set = inputs
        .iter()
        .map(|seat| seat.get_id())
        .collect::<BTreeSet<_>>();

    let mut next_seat = None;
    for id in set {
        match next_seat {
            None => next_seat = Some(id + 1),
            Some(num) if id == num => next_seat = Some(id + 1),
            Some(num) => return num,
        }
    }

    unreachable!()
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
        assert_eq!(
            generator(SAMPLE)
                .into_iter()
                .map(|seat| seat.get_row_col())
                .collect::<Vec<_>>(),
            vec![(44, 5), (70, 7), (14, 7), (102, 4)]
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 820);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 120);
    }
}

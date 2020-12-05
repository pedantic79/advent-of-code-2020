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

#[aoc(day5 part2, vec)]
pub fn part2_vec(inputs: &[Seat]) -> usize {
    let mut set = inputs.iter().map(|seat| seat.get_id()).collect::<Vec<_>>();
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
pub fn part2(inputs: &[Seat]) -> usize {
    let mut seats = [false; 1024];

    for id in inputs.iter().map(|seat| seat.get_id()) {
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

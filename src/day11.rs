#[derive(PartialEq, Copy, Clone)]
enum SeatState {
    Empty,
    Occupied,
    Blank,
}

impl std::fmt::Debug for SeatState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SeatState::Empty => 'L',
                SeatState::Occupied => '#',
                SeatState::Blank => '.',
            }
        )
    }
}

impl SeatState {
    fn occupied(&self) -> usize {
        if let SeatState::Occupied = self {
            1
        } else {
            0
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Increase,
    Decrease,
    Steady,
}

impl Direction {
    fn next(&self, x: usize) -> usize {
        match self {
            Direction::Increase => x + 1,
            Direction::Decrease => x.wrapping_sub(1),
            Direction::Steady => x,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Floor {
    floor: Vec<Vec<SeatState>>,
}

impl std::fmt::Debug for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.floor {
            for seat in row.iter() {
                write!(f, "{:?}", seat)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Floor {
    fn queen_iterator(
        &self,
        row: usize,
        col: usize,
        delta_r: Direction,
        delta_c: Direction,
    ) -> QueenIterator {
        QueenIterator {
            row,
            col,
            delta_r,
            delta_c,
            floor: &self.floor,
        }
    }

    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        let row_min = row.saturating_sub(1);
        let row_max = (row + 1).min(self.floor.len() - 1);

        let col_min = col.saturating_sub(1);
        let col_max = (col + 1).min(self.floor[0].len() - 1);

        self.floor[row_min..=row_max]
            .iter()
            .map(|floor_row| {
                floor_row[col_min..=col_max]
                    .iter()
                    .map(|x| x.occupied())
                    .sum::<usize>()
            })
            .sum::<usize>()
            .saturating_sub(self.floor[row][col].occupied())
    }

    fn count_queen(&self, row: usize, col: usize) -> usize {
        [
            (Direction::Steady, Direction::Decrease),
            (Direction::Steady, Direction::Increase),
            (Direction::Decrease, Direction::Steady),
            (Direction::Increase, Direction::Steady),
            (Direction::Decrease, Direction::Decrease),
            (Direction::Decrease, Direction::Increase),
            (Direction::Increase, Direction::Increase),
            (Direction::Increase, Direction::Decrease),
        ]
        .iter()
        .map(|&(delta_r, delta_c)| {
            self.queen_iterator(row, col, delta_r, delta_c)
                .find_map(|x| Some(x.occupied()).filter(|_| x != &SeatState::Blank))
                .unwrap_or(0)
        })
        .sum()
    }

    fn rule<F>(&self, row: usize, col: usize, threshold: usize, count_fn: &F) -> SeatState
    where
        F: Fn(&Self, usize, usize) -> usize,
    {
        let seat = self.floor[row][col];
        if seat == SeatState::Blank {
            return seat;
        }

        let occ_count = count_fn(self, row, col);

        if seat == SeatState::Empty && occ_count == 0 {
            SeatState::Occupied
        } else if seat == SeatState::Occupied && occ_count >= threshold {
            SeatState::Empty
        } else {
            seat
        }
    }

    fn tick<F>(&self, threshold: usize, count_fn: &F) -> Self
    where
        F: Fn(&Self, usize, usize) -> usize,
    {
        let mut floor = vec![vec![SeatState::Blank; self.floor[0].len()]; self.floor.len()];

        for (r, row) in floor.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                *cell = self.rule(r, c, threshold, count_fn);
            }
        }

        Self { floor }
    }
}

struct QueenIterator<'a> {
    row: usize,
    col: usize,
    delta_r: Direction,
    delta_c: Direction,
    floor: &'a [Vec<SeatState>],
}

impl<'a> Iterator for QueenIterator<'a> {
    type Item = &'a SeatState;

    fn next(&mut self) -> Option<Self::Item> {
        self.row = self.delta_r.next(self.row);
        self.col = self.delta_c.next(self.col);

        self.floor.get(self.row).and_then(|row| row.get(self.col))
    }
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Floor {
    Floor {
        floor: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| match x {
                        'L' => SeatState::Empty,
                        '.' => SeatState::Blank,
                        '#' => SeatState::Occupied,
                        _ => panic!("unknown"),
                    })
                    .collect()
            })
            .collect(),
    }
}

fn solve<F>(mut current: Floor, threshold: usize, count_fn: F) -> usize
where
    F: Fn(&Floor, usize, usize) -> usize,
{
    loop {
        let next = current.tick(threshold, &count_fn);
        if next == current {
            break;
        }

        current = next;
    }

    current
        .floor
        .iter()
        .flat_map(|row| row.iter())
        .map(|s| s.occupied())
        .sum()
}

#[aoc(day11, part1)]
pub fn part1(inputs: &Floor) -> usize {
    solve(inputs.clone(), 4, Floor::count_neighbors)
}

#[aoc(day11, part2)]
pub fn part2(inputs: &Floor) -> usize {
    solve(inputs.clone(), 5, Floor::count_queen)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    pub fn test_input() {
        assert_eq!(
            format!("{:?}", generator(SAMPLE)).trim_end_matches('\n'),
            SAMPLE
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 37);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 26);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day11.txt");
        const ANSWERS: (usize, usize) = (2222, 2032);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

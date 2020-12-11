#[derive(PartialEq, Copy, Clone, Debug)]
enum GridState {
    Empty,
    Occupied,
    Blank,
}

impl std::fmt::Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            GridState::Empty => 'L',
            GridState::Occupied => '#',
            GridState::Blank => '.',
        };

        write!(f, "{}", c)
    }
}

impl GridState {
    fn occupied(&self) -> usize {
        if let GridState::Occupied = self {
            1
        } else {
            0
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Floor {
    floor: Vec<Vec<GridState>>,
}

impl std::fmt::Debug for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.floor {
            let r = row
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .concat();
            writeln!(f, "{}", r)?
        }

        Ok(())
    }
}

impl Floor {
    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        let mut count = 0;

        let row_min = row.saturating_sub(1);
        let row_max = if row + 1 < self.floor.len() {
            row + 1
        } else {
            row
        };

        let col_min = col.saturating_sub(1);
        let col_max = if col + 1 < self.floor[0].len() {
            col + 1
        } else {
            col
        };

        for floor_row in self.floor[row_min..=row_max].iter() {
            count += floor_row[col_min..=col_max]
                .iter()
                .map(|x| x.occupied())
                .sum::<usize>();
        }

        count.saturating_sub(self.floor[row][col].occupied())
    }

    fn count_queen(&self, row: usize, col: usize) -> usize {
        let mut count = 0;

        //left
        count += self.floor[row][..col]
            .iter()
            .rev()
            .find(|x| x != &&GridState::Blank)
            .unwrap_or(&GridState::Blank)
            .occupied();
        //right
        count += self.floor[row][(col + 1)..]
            .iter()
            .find(|x| x != &&GridState::Blank)
            .unwrap_or(&GridState::Blank)
            .occupied();

        //up
        count += self.floor[..row]
            .iter()
            .rev()
            .map(|r| r[col])
            .find(|x| x != &GridState::Blank)
            .unwrap_or(GridState::Blank)
            .occupied();
        //down
        count += self.floor[(row + 1)..]
            .iter()
            .map(|r| r[col])
            .find(|x| x != &GridState::Blank)
            .unwrap_or(GridState::Blank)
            .occupied();

        let mut i = 1;
        while let Some(seat) = self
            .floor
            .get(row.wrapping_sub(i))
            .and_then(|r| r.get(col.wrapping_sub(i)))
        {
            if seat == &GridState::Blank {
                i += 1;
                continue;
            }
            count += seat.occupied();
            break;
        }

        i = 1;
        while let Some(seat) = self
            .floor
            .get(row.wrapping_sub(i))
            .and_then(|r| r.get(col + i))
        {
            if seat == &GridState::Blank {
                i += 1;
                continue;
            }
            count += seat.occupied();
            break;
        }

        i = 1;
        while let Some(seat) = self.floor.get(row + i).and_then(|r| r.get(col + i)) {
            if seat == &GridState::Blank {
                i += 1;
                continue;
            }
            count += seat.occupied();
            break;
        }

        i = 1;
        while let Some(seat) = self
            .floor
            .get(row + i)
            .and_then(|r| r.get(col.wrapping_sub(i)))
        {
            if seat == &GridState::Blank {
                i += 1;
                continue;
            }
            count += seat.occupied();
            break;
        }

        count
    }

    fn rule(&self, row: usize, col: usize, threshold: usize, part1: bool) -> GridState {
        let seat = self.floor[row][col];
        if seat == GridState::Blank {
            return seat;
        }

        // let count = self.count_neighbors(row, col);

        let occ_count = if part1 {
            self.count_neighbors(row, col)
        } else {
            self.count_queen(row, col)
        };

        if seat == GridState::Empty && occ_count == 0 {
            GridState::Occupied
        } else if seat == GridState::Occupied && occ_count >= threshold {
            GridState::Empty
        } else {
            seat
        }
    }

    fn tick(&self, threshold: usize, part1: bool) -> Self {
        let mut floor = vec![vec![GridState::Blank; self.floor[0].len()]; self.floor.len()];

        for (r, row) in floor.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                *cell = self.rule(r, c, threshold, part1);
            }
        }

        Self { floor }
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
                        'L' => GridState::Empty,
                        '.' => GridState::Blank,
                        '#' => GridState::Occupied,
                        _ => panic!("unknown"),
                    })
                    .collect()
            })
            .collect(),
    }
}

#[aoc(day11, part1)]
pub fn part1(inputs: &Floor) -> usize {
    let mut current = inputs.clone();

    loop {
        let next = current.tick(4, true);
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

#[aoc(day11, part2)]
pub fn part2(inputs: &Floor) -> usize {
    let mut current = inputs.clone();

    loop {
        let next = current.tick(5, false);

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
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
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

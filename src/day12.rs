#[derive(Debug, PartialEq)]
pub enum Op {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(usize),
    Right(usize),
    Forward(i32),
}

impl From<(u8, &str)> for Op {
    fn from((b, s): (u8, &str)) -> Self {
        let n = s.parse().unwrap();

        match b {
            b'N' => Op::North(n),
            b'S' => Op::South(n),
            b'E' => Op::East(n),
            b'W' => Op::West(n),
            b'L' => Op::Left(s.parse().unwrap()),
            b'R' => Op::Right(s.parse().unwrap()),
            b'F' => Op::Forward(n),
            _ => panic!("invalid op"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    East,
    North,
    South,
    West,
}

impl Direction {
    fn left(&mut self) {
        *self = match *self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn right(&mut self) {
        *self = match *self {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ship {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Ship {
    fn move_forward(&mut self, amount: i32) {
        match self.direction {
            Direction::East => self.x += amount,
            Direction::North => self.y += amount,
            Direction::South => self.y -= amount,
            Direction::West => self.x -= amount,
        }
    }

    fn turn_left(&mut self, times: usize) {
        for _ in 0..times {
            self.direction.left()
        }
    }

    fn turn_right(&mut self, times: usize) {
        for _ in 0..times {
            self.direction.right()
        }
    }
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|x| (x.bytes().next().unwrap(), &x[1..]).into())
        .collect()
}

fn rotate_right((mut x, mut y): (i32, i32), times: usize) -> (i32, i32) {
    for _ in 0..times {
        std::mem::swap(&mut x, &mut y);
        y *= -1;
    }

    (x, y)
}

fn rotate_left((mut x, mut y): (i32, i32), times: usize) -> (i32, i32) {
    for _ in 0..times {
        std::mem::swap(&mut x, &mut y);
        x *= -1;
    }

    (x, y)
}

#[aoc(day12, part2)]
pub fn part2(inputs: &[Op]) -> i32 {
    let mut ship = Ship {
        x: 0,
        y: 0,
        direction: Direction::East,
    };

    let mut waypoint = (10, 1);

    for op in inputs {
        match op {
            Op::North(amount) => waypoint.1 += amount,
            Op::South(amount) => waypoint.1 -= amount,
            Op::East(amount) => waypoint.0 += amount,
            Op::West(amount) => waypoint.0 -= amount,
            Op::Left(amount) => waypoint = rotate_left(waypoint, *amount / 90),
            Op::Right(amount) => waypoint = rotate_right(waypoint, *amount / 90),
            Op::Forward(amount) => {
                ship.x += waypoint.0 * amount;
                ship.y += waypoint.1 * amount;
            }
        }
    }

    ship.x.abs() + ship.y.abs()
}

#[aoc(day12, part1)]
pub fn part1(inputs: &[Op]) -> i32 {
    let mut ship = Ship {
        x: 0,
        y: 0,
        direction: Direction::East,
    };

    for op in inputs {
        match op {
            Op::North(amount) => ship.y += amount,
            Op::South(amount) => ship.y -= amount,
            Op::East(amount) => ship.x += amount,
            Op::West(amount) => ship.x -= amount,
            Op::Left(amount) => ship.turn_left(*amount / 90),
            Op::Right(amount) => ship.turn_right(*amount / 90),
            Op::Forward(amount) => ship.move_forward(*amount),
        }
    }

    ship.x.abs() + ship.y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"F10
N3
F7
R90
F11";

    #[test]
    pub fn test_input() {
        use Op::*;

        assert_eq!(
            generator(SAMPLE),
            vec![Forward(10), North(3), Forward(7), Right(90), Forward(11),]
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 25);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 286);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day12.txt");
        const ANSWERS: (i32, i32) = (2458, 145_117);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

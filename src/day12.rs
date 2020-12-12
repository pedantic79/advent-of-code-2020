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
pub fn generator(input: &str) -> Vec<(u8, i32)> {
    input
        .lines()
        .map(|x| (x.bytes().next().unwrap(), x[1..].parse().unwrap()))
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
pub fn part2(inputs: &[(u8, i32)]) -> i32 {
    let mut ship = Ship {
        x: 0,
        y: 0,
        direction: Direction::East,
    };

    let mut waypoint = (10, 1);

    for (op, amount) in inputs {
        match op {
            b'N' => waypoint.1 += amount,
            b'S' => waypoint.1 -= amount,
            b'E' => waypoint.0 += amount,
            b'W' => waypoint.0 -= amount,
            b'L' => waypoint = rotate_left(waypoint, *amount as usize / 90),
            b'R' => waypoint = rotate_right(waypoint, *amount as usize / 90),
            b'F' => {
                ship.x += waypoint.0 * amount;
                ship.y += waypoint.1 * amount;
            }
            _ => panic!("invalid op"),
        }
    }

    ship.x.abs() + ship.y.abs()
}

#[aoc(day12, part1)]
pub fn part1(inputs: &[(u8, i32)]) -> i32 {
    let mut ship = Ship {
        x: 0,
        y: 0,
        direction: Direction::East,
    };

    for (op, amount) in inputs {
        match op {
            b'N' => ship.y += amount,
            b'S' => ship.y -= amount,
            b'E' => ship.x += amount,
            b'W' => ship.x -= amount,
            b'L' => ship.turn_left(*amount as usize / 90),
            b'R' => ship.turn_right(*amount as usize / 90),
            b'F' => ship.move_forward(*amount),
            _ => panic!("invalid op"),
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
        assert_eq!(
            generator(SAMPLE),
            vec![(b'F', 10), (b'N', 3), (b'F', 7), (b'R', 90), (b'F', 11),]
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
        const ANSWERS: (i32, i32) = (2458, 145117);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

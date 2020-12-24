use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

//       (-1, 1) (0, 1) (1, 1)
//   (-1, 0) (0, 0) (1, 0)
//       (0, -1) (1, -1)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord(i32, i32);

impl Coord {
    #[allow(dead_code)]
    fn neighbors(&self) -> [Coord; 6] {
        let mut ans = [*self; 6];

        for (i, [x, y]) in [[1, 0], [1, -1], [0, -1], [-1, 0], [-1, 1], [0, 1]]
            .iter()
            .enumerate()
        {
            ans[i].0 += x;
            ans[i].1 += y;
        }

        ans
    }

    fn neighbors2(&self) -> impl IntoIterator<Item = Self> + '_ {
        const NEIGHBORS: [[i32; 2]; 6] = [[1, 0], [1, -1], [0, -1], [-1, 0], [-1, 1], [0, 1]];

        NEIGHBORS.iter().map(move |[x, y]| {
            let mut ans = *self;
            ans.0 += x;
            ans.1 += y;
            ans
        })
    }
}

impl FromStr for Coord {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut current = Coord(0, 0);

        let mut it = line.bytes();
        while let Some(c) = it.next() {
            match c {
                b'e' => current.0 += 1,
                b'w' => current.0 -= 1,
                b'n' => match it.next().unwrap() {
                    b'e' => current.1 += 1,
                    b'w' => {
                        current.1 += 1;
                        current.0 -= 1;
                    }
                    _ => return Err("unknown symbol n_"),
                },
                b's' => match it.next().unwrap() {
                    b'e' => {
                        current.1 -= 1;
                        current.0 += 1;
                    }
                    b'w' => current.1 -= 1,
                    _ => return Err("unknown symbol s_"),
                },
                _ => return Err("unknown symbol _"),
            }
        }

        Ok(current)
    }
}

fn tick(black_tiles: HashSet<Coord>) -> HashSet<Coord> {
    let mut counts = HashMap::with_capacity(black_tiles.len() * 6);

    for coord in black_tiles.iter() {
        for neighbor in coord.neighbors2() {
            *counts.entry(neighbor).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .filter_map(|(current, count)| {
            if !black_tiles.contains(&current) && count == 2
                || black_tiles.contains(&current) && !(count == 0 || count > 2)
            {
                Some(current)
            } else {
                None
            }
        })
        .collect()
}

#[aoc_generator(day24)]
pub fn generator(input: &str) -> HashSet<Coord> {
    let len = input.lines().count();

    let counts = input
        .lines()
        .map(|line| Coord::from_str(line).unwrap())
        .fold(HashMap::with_capacity(len), |mut hm, coord| {
            *hm.entry(coord).or_insert(0) += 1;
            hm
        });

    counts
        .into_iter()
        .filter_map(
            |(coord, count)| {
                if count % 2 == 1 {
                    Some(coord)
                } else {
                    None
                }
            },
        )
        .collect()
}

#[aoc(day24, part1)]
pub fn part1(inputs: &HashSet<Coord>) -> usize {
    inputs.len()
}

#[aoc(day24, part2)]
pub fn part2(inputs: &HashSet<Coord>) -> usize {
    let mut state = inputs.clone();

    for _ in 0..100 {
        state = tick(state);
    }

    state.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 10);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 2208);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day24.txt");
        const ANSWERS: (usize, usize) = (386, 4214);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

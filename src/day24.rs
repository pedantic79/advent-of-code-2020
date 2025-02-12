use nom::{
    branch::alt, bytes::complete::tag, combinator::all_consuming, multi::fold_many1, IResult,
    Parser,
};

use std::collections::{HashMap, HashSet};

//       (-1, 1) (0, 1) (1, 1)
//   (-1, 0) (0, 0) (1, 0)
//       (0, -1) (1, -1)

//       (0, 1) (1, 1)
//   (-1, 0) (0, 0) (1, 0)
//      (-1, -1) (0, -1)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct HexCoord(i32, i32);

impl HexCoord {
    fn neighbors(self) -> [HexCoord; 6] {
        [
            self.nw(),
            self.ne(),
            self.e(),
            self.se(),
            self.sw(),
            self.w(),
        ]
    }

    fn ne(self) -> Self {
        Self(self.0 + 1, self.1 + 1)
    }

    fn nw(self) -> Self {
        Self(self.0, self.1 + 1)
    }

    fn se(self) -> Self {
        Self(self.0, self.1 - 1)
    }

    fn sw(self) -> Self {
        Self(self.0 - 1, self.1 - 1)
    }

    fn e(self) -> Self {
        Self(self.0 + 1, self.1)
    }

    fn w(self) -> Self {
        Self(self.0 - 1, self.1)
    }

    fn directions(input: &str) -> IResult<&str, Self> {
        all_consuming(fold_many1(
            alt((
                tag("ne"),
                tag("nw"),
                tag("se"),
                tag("sw"),
                tag("e"),
                tag("w"),
            )),
            || Self(0, 0),
            |current, s| match s {
                "ne" => current.ne(),
                "nw" => current.nw(),
                "se" => current.se(),
                "sw" => current.sw(),
                "e" => current.e(),
                "w" => current.w(),
                _ => unreachable!(),
            },
        ))
        .parse(input)
    }
}

impl From<&str> for HexCoord {
    fn from(s: &str) -> Self {
        Self::directions(s).unwrap().1
    }
}

fn tick(black_tiles: HashSet<HexCoord>) -> HashSet<HexCoord> {
    let mut counts = HashMap::with_capacity(black_tiles.len() * 6);

    for coord in black_tiles.iter() {
        for neighbor in coord.neighbors().iter().copied() {
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
pub fn generator(input: &str) -> HashSet<HexCoord> {
    let counts = input
        .lines()
        .map(HexCoord::from)
        .fold(HashMap::new(), |mut hm, coord| {
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
pub fn part1(inputs: &HashSet<HexCoord>) -> usize {
    inputs.len()
}

#[aoc(day24, part2)]
pub fn part2(inputs: &HashSet<HexCoord>) -> usize {
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

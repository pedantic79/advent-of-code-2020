#![warn(rust_2018_idioms)]

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Top,
    Right,
    Bottom,
    Left,
}

impl From<usize> for Dir {
    fn from(n: usize) -> Self {
        match n % 4 {
            0 => Self::Top,
            1 => Self::Right,
            2 => Self::Bottom,
            3 => Self::Left,
            _ => panic!("modulo 4 should be between 0 and 3 exclusively"),
        }
    }
}

impl Default for Dir {
    fn default() -> Self {
        Self::Top
    }
}

impl Dir {
    fn value(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug, PartialEq)]
pub struct Tile {
    id: usize,
    data: Vec<Vec<u16>>,
}

impl Tile {
    pub fn new(id: usize, data: Vec<Vec<u16>>) -> Self {
        Self { id, data }
    }

    fn edges(&self) -> ([u16; 4], [u16; 4]) {
        fn array2int(data: &[u16]) -> (u16, u16) {
            let reverse = data.iter().rev().fold(0, |acc, n| acc * 2 + n);
            let forward = data.iter().fold(0, |acc, n| acc * 2 + n);

            (forward, reverse)
        }

        // top
        let (a1, a2) = array2int(&self.data.first().unwrap());

        // bottom
        let (b1, b2) = array2int(&self.data.last().unwrap());

        // left
        let (c1, c2) = array2int(&self.data.iter().map(|row| row[0]).collect::<Vec<_>>());

        // right
        let (d1, d2) = array2int(
            &self
                .data
                .iter()
                .map(|row| row[row.len() - 1])
                .collect::<Vec<_>>(),
        );

        ([a1, d1, b2, c2], [a2, d2, b1, c1])
    }

    fn rotate(&self, edge: u16, direction: Dir, flipped: bool) -> ModifiedTile<'_> {
        let edges = {
            let e = self.edges();
            if flipped {
                e.1
            } else {
                e.0
            }
        };

        assert!(edges.contains(&edge));
        let mut d = 0;
        while edges[d % 4] != edge {
            d += 1;
        }

        ModifiedTile {
            flipped,
            direction: (direction.value() + d).into(),
            id: self.id,
            tile: Some(&self),
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
struct ModifiedTile<'a> {
    flipped: bool,
    direction: Dir,
    id: usize,
    tile: Option<&'a Tile>,
}

impl<'a> ModifiedTile<'a> {
    fn edges(&self) -> ([u16; 4], [u16; 4]) {
        let (mut a, mut b) = self.tile.unwrap().edges();
        a.rotate_right(self.direction.value());
        b.rotate_right(self.direction.value());

        if self.flipped {
            (b, a)
        } else {
            (a, b)
        }
    }
}

struct TileCache<'a> {
    id2tile: HashMap<usize, &'a Tile>,
    edge2tile: HashMap<u16, Vec<&'a Tile>>,
    edge2id: HashMap<u16, Vec<usize>>,
    tiles_one_edge: HashMap<usize, usize>,
}

impl<'a> TileCache<'a> {
    fn new(tiles: &'a [Tile]) -> Self {
        let id2tile = tiles
            .iter()
            .map(|tile| (tile.id, tile))
            .collect::<HashMap<_, _>>();

        let mut edge2tile = HashMap::new();
        let mut edge2id = HashMap::new();

        for tile in tiles.iter() {
            let id = tile.id;
            let (normal, flipped) = tile.edges();

            for edge in normal.iter().chain(flipped.iter()) {
                edge2tile.entry(*edge).or_insert_with(Vec::new).push(tile);
                edge2id.entry(*edge).or_insert_with(Vec::new).push(id);
            }
        }

        let tiles_one_edge = edge2id
            .iter()
            .fold(HashMap::new(), |mut hm, (_edge_id, tileids)| {
                if tileids.len() == 1 {
                    *hm.entry(tileids[0]).or_insert(0_usize) += 1;
                }
                hm
            });

        Self {
            id2tile,
            edge2tile,
            edge2id,
            tiles_one_edge,
        }
    }
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|section| {
            let mut line = section.lines();

            let tile_id = line
                .next()
                .unwrap()
                .split(&[' ', ':'][..])
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            let tile: Vec<Vec<u16>> = line
                .map(|l| l.chars().map(|x| if x == '#' { 1 } else { 0 }).collect())
                .collect();

            Tile::new(tile_id, tile)
        })
        .collect()
}

fn solve1(cache: &TileCache) -> (Vec<usize>, Vec<usize>) {
    let corners: Vec<usize> = cache
        .tiles_one_edge
        .iter()
        .filter_map(|(id, c)| if *c > 2 { Some(*id) } else { None })
        .collect();

    let sides = cache
        .tiles_one_edge
        .iter()
        .filter_map(|(id, c)| if *c <= 2 { Some(*id) } else { None })
        .collect();

    (corners, sides)
}

#[aoc(day20, part1)]
pub fn part1(inputs: &[Tile]) -> usize {
    let cache = TileCache::new(inputs);
    solve1(&cache).0.into_iter().product()
}

#[aoc(day20, part2)]
pub fn part2(tiles: &[Tile]) -> usize {
    let cache = TileCache::new(tiles);

    let (corners, sides) = solve1(&cache);
    let l = if tiles.len() == 144 { 12 } else { 3 }; // Cheating, you can use sqrt

    let mut mosiac = vec![vec![ModifiedTile::default(); l]; l];
    mosiac[0][0].id = corners[0];

    println!("Edges {:?}", cache.id2tile[&corners[0]].edges());

    for side in sides {
        println!("{}: {:?}", side, cache.id2tile[&side].edges());
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../input/2020/day20-sample.txt");

    #[test]
    #[ignore]
    pub fn test_input() {

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    #[ignore]
    pub fn test1() {
        let input = generator(SAMPLE);
        println!("{:?}", input[3079].edges());
        println!("{:?}", input[2473].rotate(116, Dir::Top, false).edges());

        assert_eq!(input.len(), 9);
        assert_eq!(part1(&input), 20899048083289);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 273);
    }

    #[test]
    fn test_rotate() {
        const TILEA: &str = "#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

        const TILE_B: &str = "...###.#..
.......#..
...###.#..
#.#####.##
.##.#...#.
.#..#.####
....######
.......#..
######..#.
.#####.#.#";

        let tilea = Tile::new(
            1234,
            TILEA
                .lines()
                .map(|l| l.chars().map(|x| if x == '#' { 1 } else { 0 }).collect())
                .collect(),
        );

        let tileb = Tile::new(
            1234,
            TILE_B
                .lines()
                .map(|l| l.chars().map(|x| if x == '#' { 1 } else { 0 }).collect())
                .collect(),
        );

        assert_eq!(tilea.edges(), tileb.rotate(702, Dir::Top, false).edges());

        for d in 0..4 {
            assert_eq!(tilea.rotate(702, d.into(), false).edges().0[d], 702);
            assert_eq!(tilea.rotate(184, d.into(), true).edges().0[d], 184);
        }
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day20.txt");
        const ANSWERS: (usize, usize) = (104831106565027, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            // assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

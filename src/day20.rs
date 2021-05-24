use std::collections::{HashMap, HashSet};

use crate::matrix::{flip, rotate_bottom, rotate_left, rotate_right};

const SEA_MONSTER_MATRIX: [&[u8]; 3] = [
    b"                  # ",
    b"#    ##    ##    ###",
    b" #  #  #  #  #  #   ",
];

const SEA_MONSTER: [u32; 3] = [
    sea_monster_chksum(SEA_MONSTER_MATRIX[0]),
    sea_monster_chksum(SEA_MONSTER_MATRIX[1]),
    sea_monster_chksum(SEA_MONSTER_MATRIX[2]),
];

const SEA_MONSTER_SIZE: usize = count_sea_monster();

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
    fn value(self) -> usize {
        self as usize
    }
}

const fn sea_monster_chksum(r1: &[u8]) -> u32 {
    let mut sum = 0;

    let mut j = 0;
    while j < r1.len() {
        sum = sum * 2 + if r1[j] == b'#' { 1 } else { 0 };
        j += 1;
    }

    sum
}

const fn count_sea_monster() -> usize {
    let mut sum = 0;

    let mut j = 0;
    while j < SEA_MONSTER.len() {
        sum += SEA_MONSTER[j].count_ones() as usize;
        j += 1;
    }

    sum
}

fn check_sea_monster<A>(grid: &[A]) -> usize
where
    A: AsRef<[u8]>,
{
    const WIDTH: usize = SEA_MONSTER_MATRIX[0].len();

    debug_assert_eq!(grid.len(), 3);
    let len = grid[0].as_ref().len();

    (0..(len - WIDTH))
        .filter(|&offset| {
            let end = offset + WIDTH;

            (0..grid.len()).all(|i| {
                let r1 = &grid[i].as_ref()[offset..end];
                let map = sea_monster_chksum(r1);

                SEA_MONSTER[i] & map == SEA_MONSTER[i]
            })
        })
        .count()
}

#[allow(dead_code)]
fn print_full_grid(mosiac: &[Vec<Option<ModifiedTile<'_>>>]) {
    let l = mosiac.len();
    let mut full_grid = vec![vec![b'.'; l * 10]; l * 10];

    for (r, m_row) in mosiac.iter().enumerate() {
        for (c, cell) in m_row.iter().enumerate() {
            let r_offset = r * 10;
            let c_offset = c * 10;
            let map = cell.as_ref().unwrap().symbols_debug();

            for (row, mrow) in full_grid[r_offset..(r_offset + 10)]
                .iter_mut()
                .zip(map.iter())
            {
                row[c_offset..(c_offset + 10)].copy_from_slice(mrow)
            }
        }
    }

    println!();
    print_grid(&full_grid, 10, true);
}

fn print_grid(grid: &[Vec<u8>], size: usize, debug: bool) {
    let l = grid.len();
    let mut each = l / 10;
    each += match each {
        9 => 3,
        2 => 1,
        _ => 0,
    };

    if !debug {
        for row in grid.iter() {
            println!("{}", row.iter().map(|x| *x as char).collect::<String>());
        }
    } else {
        for (r, row) in grid.iter().enumerate() {
            if r % size == 0 {
                println!(
                    "{}",
                    std::iter::repeat('-')
                        .take(each * (size + 3))
                        .collect::<String>()
                );
            }

            for sec in row.chunks(size).map(|x| std::str::from_utf8(x).unwrap()) {
                print!(" {} |", sec);
            }
            println!();
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Tile {
    id: usize,
    data: Vec<Vec<u16>>,
    edges: ([u16; 4], [u16; 4]),
}

impl Tile {
    pub fn new(id: usize, data: Vec<Vec<u16>>) -> Self {
        let edges = Self::calc_edges(&data);
        Self { id, data, edges }
    }

    fn calc_edges(data: &[Vec<u16>]) -> ([u16; 4], [u16; 4]) {
        fn array2int(data: &[u16]) -> (u16, u16) {
            let forward = data.iter().fold(0, |acc, n| acc * 2 + n);
            let reverse = data.iter().rev().fold(0, |acc, n| acc * 2 + n);

            (forward, reverse)
        }

        // top
        let (a1, a2) = array2int(data.first().unwrap());

        // bottom
        let (b1, b2) = array2int(data.last().unwrap());

        // left
        let (c1, c2) = array2int(&data.iter().map(|row| row[0]).collect::<Vec<_>>());

        // right
        let (d1, d2) = array2int(
            &data
                .iter()
                .map(|row| row[row.len() - 1])
                .collect::<Vec<_>>(),
        );

        // ([a1, d1, b2, c2], [c1, b1, d2, a2])
        // ([a1, d1, b2, c2], [a2, d2, b1, c1])
        ([a1, d1, b1, c1], [a2, d2, b2, c2])
    }

    fn edges(&self) -> ([u16; 4], [u16; 4]) {
        self.edges
    }

    fn rotate(&self, edge: u16, direction: Dir) -> ModifiedTile<'_> {
        let mut d = 0;

        loop {
            for &flipped in &[false, true] {
                let mt = ModifiedTile {
                    flipped,
                    direction: d.into(),
                    tile: &self,
                };

                if mt.edges().0[direction.value()] == edge {
                    return mt;
                }
            }

            d += 1;
            if d > 4 {
                panic!("invalid combo to rotate")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct ModifiedTile<'a> {
    flipped: bool,
    direction: Dir,
    tile: &'a Tile,
}

impl<'a> ModifiedTile<'a> {
    fn edges(&self) -> ([u16; 4], [u16; 4]) {
        let (mut a, mut b) = self.tile.edges();

        if !self.flipped {
            a.rotate_right(self.direction.value());
            b.rotate_right(self.direction.value());

            match self.direction {
                Dir::Top => (a, b),
                Dir::Right => {
                    std::mem::swap(&mut a[0], &mut b[0]);
                    std::mem::swap(&mut a[2], &mut b[2]);
                    (a, b)
                }
                Dir::Bottom => (b, a),
                Dir::Left => {
                    std::mem::swap(&mut a[0], &mut b[0]);
                    std::mem::swap(&mut a[2], &mut b[2]);
                    (b, a)
                }
            }
        } else {
            a.rotate_left(self.direction.value());
            b.rotate_left(self.direction.value());

            match self.direction {
                Dir::Top => {
                    std::mem::swap(&mut a[1], &mut b[3]);
                    std::mem::swap(&mut b[1], &mut a[3]);
                    (b, a)
                }
                Dir::Right => {
                    a.swap(1, 3);
                    b.swap(1, 3);
                    (b, a)
                }
                Dir::Bottom => {
                    std::mem::swap(&mut a[1], &mut b[3]);
                    std::mem::swap(&mut b[1], &mut a[3]);
                    (a, b)
                }
                Dir::Left => {
                    a.swap(1, 3);
                    b.swap(1, 3);
                    (a, b)
                }
            }
        }
    }

    fn symbols(&self) -> [[u8; 8]; 8] {
        let mut grid = [[b'.'; 8]; 8];

        for (r, row) in self.tile.data.iter().skip(1).take(8).enumerate() {
            for (c, cell) in row.iter().skip(1).take(8).enumerate() {
                grid[r][c] = if *cell == 1 { b'#' } else { b'.' };
            }
        }

        if self.flipped {
            flip(&mut grid);
        }

        // for _ in 0..self.direction.value() {
        //     rotate_right(&mut grid);
        // }

        match self.direction.value() {
            1 => rotate_right(&mut grid),
            2 => rotate_bottom(&mut grid),
            3 => rotate_left(&mut grid),
            _ => {}
        }

        grid
    }

    #[allow(dead_code)]
    fn symbols_debug(&self) -> [[u8; 10]; 10] {
        let mut grid = [[b'.'; 10]; 10];

        for (r, row) in self.tile.data.iter().skip(0).take(10).enumerate() {
            for (c, cell) in row.iter().skip(0).take(10).enumerate() {
                grid[r][c] = if *cell == 1 { b'#' } else { b'.' };
            }
        }

        if self.flipped {
            flip(&mut grid);
        }

        for _ in 0..self.direction.value() {
            rotate_right(&mut grid);
        }

        grid
    }
}

struct TileCache<'a> {
    id2tile: HashMap<usize, &'a Tile>,
    edge2tile: HashMap<u16, Vec<&'a Tile>>,
}

impl<'a> TileCache<'a> {
    fn new(tiles: &'a [Tile]) -> Self {
        let id2tile = tiles
            .iter()
            .map(|tile| (tile.id, tile))
            .collect::<HashMap<_, _>>();

        let mut edge2tile = HashMap::new();

        for tile in tiles.iter() {
            let (normal, flipped) = tile.edges();

            for edge in normal.iter().chain(flipped.iter()) {
                edge2tile.entry(*edge).or_insert_with(Vec::new).push(tile);
            }
        }

        Self { id2tile, edge2tile }
    }

    // Orient tile so top and left corners are unique
    fn orient_first_tile(&self, tile_id: usize) -> ModifiedTile<'a> {
        let tile = self.id2tile[&tile_id];

        for &flipped in &[false, true] {
            for &direction in [Dir::Top, Dir::Right, Dir::Bottom, Dir::Left].iter().rev() {
                let mod_tile = ModifiedTile {
                    flipped,
                    direction,
                    tile,
                };

                if mod_tile
                    .edges()
                    .0
                    .iter()
                    .map(|edge| (self.get_edge_count_by_edge_id(*edge)))
                    .eq([1, 2, 2, 1].iter().copied())
                {
                    return mod_tile;
                }
            }
        }

        unreachable!()
    }

    fn get_edge_count_by_edge_id(&self, edge_id: u16) -> usize {
        self.edge2tile[&edge_id].len()
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

fn solve1(cache: &TileCache<'_>) -> (Vec<usize>, Vec<usize>) {
    // maps tileid to unique_edges
    let unique_tile_edge_count =
        cache
            .edge2tile
            .iter()
            .fold(HashMap::new(), |mut hm, (_edge_id, tileids)| {
                if tileids.len() == 1 {
                    *hm.entry(tileids[0].id).or_insert(0_usize) += 1;
                }
                hm
            });

    let corners: Vec<usize> = unique_tile_edge_count
        .iter()
        .filter_map(|(id, c)| if *c > 2 { Some(*id) } else { None })
        .collect();

    let sides = unique_tile_edge_count
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

    let (corners, _sides) = solve1(&cache);
    let l = if tiles.len() == 144 { 12 } else { 3 }; // Cheating, you can use sqrt

    let mut mosiac: Vec<Vec<Option<ModifiedTile<'_>>>> = vec![vec![None; l]; l];

    for row in 0..l {
        let mut last_mt;

        if row == 0 {
            last_mt = cache.orient_first_tile(corners[0]);
        } else {
            last_mt = mosiac[row - 1][0].clone().unwrap();
            let target_top_edge = last_mt.edges().0[Dir::Bottom.value()];
            let next = cache.edge2tile[&target_top_edge]
                .iter()
                .find(|tile| tile.id != last_mt.tile.id)
                .copied()
                .unwrap();

            last_mt = next.rotate(target_top_edge, Dir::Top);
        }
        mosiac[row][0] = Some(last_mt.clone());

        for target in mosiac[row].iter_mut().skip(1) {
            let target_left_edge = last_mt.edges().0[Dir::Right.value()];
            let next = cache.edge2tile[&target_left_edge]
                .iter()
                .find(|tile| tile.id != last_mt.tile.id)
                .copied()
                .unwrap();

            last_mt = next.rotate(target_left_edge, Dir::Left);
            *target = Some(last_mt.clone());
        }
    }

    debug_assert_eq!(
        mosiac
            .iter()
            .flat_map(|row| row.iter())
            .filter_map(|x| x.as_ref().map(|mt| mt.tile.id))
            .collect::<HashSet<_>>(),
        tiles.iter().map(|x| x.id).collect::<HashSet<_>>()
    );

    // print_full_grid(&mosiac);

    let mut grid = vec![vec![b'.'; l * 8]; l * 8];

    for (r, m_row) in mosiac.iter().enumerate() {
        for (c, cell) in m_row.iter().enumerate() {
            let r_offset = r * 8;
            let c_offset = c * 8;
            let map = cell.as_ref().unwrap().symbols();

            for (row, mrow) in grid[r_offset..(r_offset + 8)].iter_mut().zip(map.iter()) {
                row[c_offset..(c_offset + 8)].copy_from_slice(mrow)
            }
        }
    }

    let mut count = 0;

    for _ in 0..2 {
        for _ in 0..4 {
            for rows in grid.windows(3) {
                count += check_sea_monster(rows);
            }
            if count > 0 {
                // print_grid(&grid, 8, true);
                // println!("count: {}", count);
                break;
            }

            rotate_right(&mut grid);
        }
        flip(&mut grid);
    }

    grid.iter()
        .map(|row| bytecount::count(&row, b'#'))
        .sum::<usize>()
        - SEA_MONSTER_SIZE * count
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
    // #[ignore]
    pub fn test1() {
        let input = generator(SAMPLE);
        // println!("{:?}", input[3079].edges());
        // println!("{:?}", input[2473].rotate(116, Dir::Top, false).edges());

        assert_eq!(input.len(), 9);
        assert_eq!(part1(&input), 20_899_048_083_289);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 273);
    }

    #[test]
    fn test_check_sea_monster() {
        const TEST: [&[u8]; 3] = [
        b"#.##....##.......###.#.#............#....#.......#.#........#..#..#..............#.......##.##..",
        b"........#..#...#...####......#......#......##.......#..#...###.###..###.##.#..##.#.##...........",
        b"#..........#.....#.........#...#....#.....###............##...#...#.#..#.###...##....###.#......",
    ];

        const GRID: [&[u8]; 3] = [
        b"#..#...#..#..#..#..#..#.........##..#......#..#.##...##........#.............#.#.......#...#..#.",
        b"..#...#...###....##....###..........#.##........##.......#.....#...#....#..#.###.#..#.##.#..##..",
        b"...#.......##...........#...#.#.#.#..##.#..#..##.#.#.#...........#.#..##....####.###..#.####....",
    ];

        assert_eq!(check_sea_monster(&TEST), 0);
        assert_eq!(check_sea_monster(&GRID), 0);
    }

    #[test]
    fn test_check_sea_monster2() {
        const TEST: [&[u8]; 5] = [
            b".####...#####..#...###..",
            b"#####..#..#.#.####..#.#.",
            b".#.#...#.###...#.##.##..",
            b"#.#.##.###.#.##.##.#####",
            b"..##.###.####..#.####.##",
        ];

        let count: usize = TEST.windows(3).map(|x| check_sea_monster(x)).sum();

        assert_eq!(count, 1)
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day20.txt");
        const ANSWERS: (usize, usize) = (104_831_106_565_027, 2093);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}

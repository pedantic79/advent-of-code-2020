use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Tile(Vec<Vec<u16>>);

impl Tile {
    fn edges(&self) -> ([u16; 4], [u16; 4]) {
        let (a1, a2) = array2int(&self.0.first().unwrap());
        let (b1, b2) = array2int(&self.0.last().unwrap());
        let (c1, c2) = array2int(&self.0.iter().map(|row| row[0]).collect::<Vec<_>>());
        let (d1, d2) = array2int(
            &self
                .0
                .iter()
                .map(|row| row[row.len() - 1])
                .collect::<Vec<_>>(),
        );

        ([a1, b1, c1, d1], [a2, b2, c2, d2])
    }
}

fn array2int(data: &[u16]) -> (u16, u16) {
    let forward = data.iter().rev().fold(0, |acc, n| acc * 2 + n);
    let reverse = data.iter().fold(0, |acc, n| acc * 2 + n);

    (forward, reverse)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Rotated {
    N,
    E,
    S,
    W,
}

impl Default for Rotated {
    fn default() -> Self {
        Self::N
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct ModifiedTile {
    flipped: bool,
    rotated: Rotated,
    id: usize,
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> HashMap<usize, Tile> {
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

            (tile_id, Tile(tile))
        })
        .collect()
}

fn solve1(inputs: &HashMap<usize, Tile>) -> (Vec<usize>, Vec<usize>) {
    let mut all_edges = HashMap::new();

    for (id, (normal, flipped)) in inputs.iter().map(|(id, x)| (id, x.edges())) {
        for edge in normal.iter().chain(flipped.iter()) {
            all_edges.entry(*edge).or_insert_with(Vec::new).push(id)
        }
    }

    let border_count = all_edges.iter().fold(HashMap::new(), |mut hm, (_, v)| {
        if v.len() < 2 {
            *hm.entry(v[0]).or_insert(0) += 1;
        }

        hm
    });

    let corners = border_count
        .iter()
        .filter_map(|(id, c)| if *c > 2 { Some(**id as usize) } else { None })
        .collect();

    let edges = border_count
        .iter()
        .filter_map(|(id, c)| if *c <= 2 { Some(**id as usize) } else { None })
        .collect();

    (corners, edges)
}

#[aoc(day20, part1)]
pub fn part1(inputs: &HashMap<usize, Tile>) -> usize {
    solve1(inputs).0.into_iter().product()
}

// #[aoc(day20, part2)]
pub fn part2(inputs: &HashMap<usize, Tile>) -> usize {
    let (corners, edges) = solve1(inputs);
    let l = if inputs.len() == 144 { 12 } else { 3 }; // Cheating, you can use sqrt

    let mut mosiac = vec![vec![ModifiedTile::default(); l]; l];

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../input/2020/day20-sample.txt");

    #[test]
    pub fn test_input() {

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        let input = generator(SAMPLE);
        assert_eq!(input.len(), 9);
        assert_eq!(part1(&input), 20899048083289);
    }

    #[test]
    pub fn test2() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
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

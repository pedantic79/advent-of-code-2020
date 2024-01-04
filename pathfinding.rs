use std::collections::VecDeque;

use bit_set::BitSet;

/// `bfs_count_bitset` returns the number of elements and using a bitset to keep track of seen nodes.
/// Based on [pathfinding](https://github.com/samueltardieu/pathfinding/blob/v4.0.0/src/directed/bfs.rs#L78)
/// `bfs` algorithm.
///
/// - `mapper`, must map to a usize. This is useful to map coordinates, but not hashing. Bits should be
///    tightly packed, otherwise the BitSet may grow too large.
pub fn bfs_count_bitset<N, FN, IN, FS, FM>(
    start: &N,
    mut successors: FN,
    mut success: FS,
    mut mapper: FM,
) -> Option<usize>
where
    N: Eq + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
    FM: FnMut(&N) -> usize,
{
    if success(start) {
        return Some(1);
    }
    let mut queue = VecDeque::new();
    let mut seen = BitSet::new();

    queue.push_back((start.clone(), 0));
    seen.insert(mapper(start));

    while let Some((node, depth)) = queue.pop_front() {
        let depth = depth + 1;
        for successor in successors(&node) {
            if success(&successor) {
                return Some(depth);
            }
            let v = mapper(&successor);
            if !seen.contains(v) {
                queue.push_back((successor, depth));
                seen.insert(v);
            }
        }
    }

    None
}

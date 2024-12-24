use fxhash::FxHashSet;
use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    unsafe { inner_part1(input.as_bytes()) }
}
// 1046

pub fn part2(input: &str) -> String {
    unsafe { inner_part2(input.as_bytes()) }
}
// de,id,ke,ls,po,sn,tf,tl,tm,uj,un,xw,yz

// 6426
const SIZE: usize = get_index("zz".as_bytes()) as usize + 1;

pub unsafe fn inner_part1(input: &[u8]) -> usize {
    let input = &input[..input.len() - 1];

    let mut map: [Vec<u16>; SIZE] = [const { vec![] }; SIZE];
    let mut t_list = vec![];

    // Parse and store mappings in `map`. Save list of computer names that start with "t".
    for line in input.split(|&b| b == b'\n') {
        let a = get_index(&line[..2]);
        let b = get_index(&line[3..]);

        let m = &mut map[a as usize];
        if line[0] == b't' && m.is_empty() {
            t_list.push(a);
        }
        m.push(b);

        let m = &mut map[b as usize];
        if line[3] == b't' && m.is_empty() {
            t_list.push(b);
        }
        m.push(a);
    }

    // Results is a set containing all 3 length cliques with a "t" computer
    let mut results: std::collections::HashSet<
        [u16; 3],
        std::hash::BuildHasherDefault<fxhash::FxHasher>,
    > = FxHashSet::default();

    for t in t_list {
        let t_vec = &map[t as usize];
        for &e0 in t_vec {
            for e1 in &map[e0 as usize] {
                if t_vec.contains(e1) {
                    let mut set = [t, e0, *e1];

                    // Must be sorted so that we don't add duplicate sets
                    sort3(&mut set);
                    results.insert(set);
                }
            }
        }
    }
    results.len()
}
// 1046

unsafe fn inner_part2(input: &[u8]) -> String {
    let input = &input[..input.len() - 1];

    let mut map: [Vec<u16>; SIZE] = [const { vec![] }; SIZE];
    let mut keys = vec![];
    
    // Parse mappings. Since `map` is a sparse vec, we track separate list of unique computers in `keys`
    for line in input.split(|&b| b == b'\n') {
        let a = get_index(&line[..2]);
        let b = get_index(&line[3..]);

        let m = &mut map[a as usize];
        if m.is_empty() {
            keys.push(a);
        }
        m.push(b);
        let m = &mut map[b as usize];
        if m.is_empty() {
            keys.push(b);
        }
        m.push(a);
    }
    // Track largest clique found so far
    let mut max_index = 0;
    let mut max_len = 0;

    let mut sets: Vec<Vec<u16>> = vec![];

    for k in keys.iter() {
        if sets.iter().any(|s| s.contains(k)) {
            // We have already calculated the size of the clique that contains `k`
            continue;
        }
        // `set` will contain the nodes that form the maximal complete graph with this `k`
        let mut set = vec![];
        rec(*k, &mut set, &map);

        if set.len() > max_len {
            max_index = sets.len();
            max_len = set.len();
        }
        sets.push(set);
    }
    let mut result = sets[max_index].iter().collect::<Vec<_>>();
    result.sort_unstable();

    // Convert back to computer names from indices
    let result = result.iter().map(|&&r| get_computer_name(r)).join(",");

    result
}

/// Convert computer name to mapping index
const fn get_index(c: &[u8]) -> u16 {
    (((c[0] - b'a') as u16) << 8) | (c[1] - b'a') as u16
}

/// Convert from mapping index back to compter name
unsafe fn get_computer_name(i: u16) -> String {
    const MASK: u16 = 0b1111_1111;
    core::str::from_utf8_unchecked([((i >> 8) & MASK) as u8 + b'a', (i & MASK) as u8 + b'a'].as_slice())
        .to_string()
}

/// Recursive function to fill `set` with maximal clique that contains `s`
fn rec(
    s: u16,
    set: &mut Vec<u16>,
    map: &[Vec<u16>; SIZE],
) {
    if set.contains(&s) {
        return;
    }
    set.push(s);
    for &e in &map[s as usize] {
        // All the items in the set so far must be found among the nodes that are connected to
        // the nodes that are connected to `s`
        let k = &map[e as usize];
        if set.iter().all(|l| k.contains(l)) {
            // Still complete with `e`, keep searching
            rec(e, set, map);
            return;
        }
    }
}

/// Sort array of 3 u16s
unsafe fn sort3(list: &mut [u16; 3]) {
    if list.get_unchecked(0) > list.get_unchecked(1) {
        list.swap(0, 1);
    }
    if list.get_unchecked(1) > list.get_unchecked(2) {
        list.swap(1, 2);
    }
    if list.get_unchecked(0) > list.get_unchecked(1) {
        list.swap(0, 1);
    }
}

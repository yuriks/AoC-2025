use std::{array, io};
use std::cmp::Reverse;
use std::collections::BTreeMap;

fn sqr_distance(a: &[i64; 3], b: &[i64; 3]) -> i64 {
    fn sqr(x: i64) -> i64 { x * x }
    sqr(a[0] - b[0]) + sqr(a[1] - b[1]) + sqr(a[2] - b[2])
}

fn minmax<T: Ord>(a: T, b: T) -> [T; 2] {
    if b < a { [b, a] } else { [a, b] }
}

struct UnionFind {
    memberships: Vec<usize>,
}

impl UnionFind {
    fn new(total_items: usize) -> UnionFind {
        UnionFind { memberships: (0..total_items).collect() }
    }

    /// Returns the id of the set item_id is a member of.
    fn resolve_set(&self, item_id: usize) -> usize {
        let mut set_id = item_id;
        while self.memberships[set_id] != set_id {
            set_id = self.memberships[set_id];
        }
        set_id
    }

    /// Merges the sets of both items together. Returns the id of the resulting set.
    fn merge_items(&mut self, item1: usize, item2: usize) -> usize {
        let set1 = self.resolve_set(item1);
        let set2 = self.resolve_set(item2);
        let [dst_set, src_set] = minmax(set1, set2);
        self.memberships[src_set] = dst_set;
        dst_set
    }
}

fn main() -> io::Result<()> {
    let mut points = Vec::new();

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut it = line.splitn(3, ',').map(|s| s.parse().unwrap());
        let point: [i64; 3] = array::from_fn(|_| it.next().unwrap());
        points.push(point);
    }

    // Compute n^2 distances between all points
    let mut distances = Vec::with_capacity(points.len());
    for (i, pi) in points.iter().enumerate() {
        for (j, pj) in points.iter().enumerate().skip(i + 1) {
            assert!(i < j);
            let d = sqr_distance(pi, pj);
            distances.push((d, (i, j)));
        }
    }

    // Find 1000 pairs with the shortest distances
    let head_len = distances.len().min(1000);
    distances.select_nth_unstable_by_key(head_len - 1, |(d, _)| *d);
    let head = &mut distances[..head_len];
    head.sort_unstable_by_key(|(_, ij)| *ij);

    // Union-find merge all the shortest pairs
    let mut sets = UnionFind::new(points.len());
    for &(_, (i, j)) in &*head {
        sets.merge_items(i, j);
    }
    println!("membership: {:?}", sets.memberships.iter().enumerate().collect::<BTreeMap<_, _>>());

    let mut set_counts = vec![0; points.len()];
    for i in 0..points.len() {
        set_counts[sets.resolve_set(i)] += 1;
    }
    println!("counts: {:?}", set_counts.iter().enumerate().collect::<BTreeMap<_, _>>());

    // Find k=3 largest sets
    set_counts.select_nth_unstable_by_key(3 - 1, |&x| Reverse(x));
    let k_largest = &set_counts[..3];
    println!("k_largest: {k_largest:?}");

    let size_product: usize = k_largest.iter().product();
    println!("Result: {size_product}");

    Ok(())
}

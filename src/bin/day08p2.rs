use std::{array, io};

fn sqr_distance(a: &[i64; 3], b: &[i64; 3]) -> i64 {
    fn sqr(x: i64) -> i64 { x * x }
    sqr(a[0] - b[0]) + sqr(a[1] - b[1]) + sqr(a[2] - b[2])
}

fn minmax<T: Ord>(a: T, b: T) -> [T; 2] {
    if b < a { [b, a] } else { [a, b] }
}

struct UnionFind {
    memberships: Vec<usize>,
    set_sizes: Vec<usize>
}

impl UnionFind {
    fn new(total_items: usize) -> UnionFind {
        UnionFind {
            memberships: (0..total_items).collect(),
            set_sizes: vec![1; total_items],
        }
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
        if src_set != dst_set {
            self.memberships[src_set] = dst_set;
            self.set_sizes[dst_set] += self.set_sizes[src_set];
            self.set_sizes[src_set] = 0;
        }
        dst_set
    }

    fn set_size(&self, item_id: usize) -> usize {
        self.set_sizes[self.resolve_set(item_id)]
    }

    fn check_integrity(&self) {
        // Integrity check
        let mut recounts = vec![0; self.memberships.len()];
        for i in 0..self.memberships.len() {
            if self.memberships[i] == i {
                assert_ne!(self.set_sizes[i], 0);
            } else {
                assert_eq!(self.set_sizes[i], 0);
            }
            recounts[self.resolve_set(i)] += 1;
        }
        assert_eq!(recounts, self.set_sizes);
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
    let mut distances = Vec::with_capacity(points.len().pow(2) / 2);
    for (i, pi) in points.iter().enumerate() {
        for (j, pj) in points.iter().enumerate().skip(i + 1) {
            assert!(i < j);
            let d = sqr_distance(pi, pj);
            distances.push((d, (i, j)));
        }
    }

    // Union-find merge all the shortest pairs
    distances.sort_unstable_by_key(|(d, _)| *d);
    let mut sets = UnionFind::new(points.len());
    let mut last_pair = None;
    for &(_, (i, j)) in &distances {
        let result_set = sets.merge_items(i, j);
        if sets.set_size(result_set) == points.len() {
            last_pair = Some((i, j));
            break;
        }
    }
    sets.check_integrity();

    if let Some((i, j)) = last_pair {
        println!("Result: {}", points[i][0] * points[j][0]);
    }

    Ok(())
}

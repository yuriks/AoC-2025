use std::cell::OnceCell;
use std::collections::BTreeMap;
use std::io;

fn main() -> io::Result<()> {
    let mut columns: Vec<BTreeMap<usize, OnceCell<u64>>> = Vec::new();

    let mut starting_x = 0;

    for (y, line) in io::stdin().lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        assert!(line.is_ascii());
        columns.resize_with(line.len(), BTreeMap::new);
        for (x, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'S' => { starting_x = x; },
                b'^' => { columns[x].insert(y, OnceCell::new()); },
                _ => {},
            }
        }
    }

    fn trace_beam(x: usize, y: usize, columns: &[BTreeMap<usize, OnceCell<u64>>]) -> u64 {
        if let Some((&y, num_beams)) = columns[x].range(y..).next() {
            *num_beams.get_or_init(|| {
                trace_beam(x - 1, y, columns) + trace_beam(x + 1, y, columns)
            })
        } else {
            1
        }
    }

    let num_beams = trace_beam(starting_x, 0, &columns);
    println!("Number of splits: {num_beams}");

    Ok(())
}

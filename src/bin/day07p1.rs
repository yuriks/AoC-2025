use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        assert!(line.is_ascii());
        grid.push(line.into_bytes());
    }

    let mut beam_queue = VecDeque::new();
    let mut num_splits = 0;

    // Find starting position
    let starting_x = grid[0].iter().position(|&c| c == b'S').unwrap();
    beam_queue.push_back((starting_x, 0));

    while let Some((x, y)) = beam_queue.pop_front() {
        for y in y..grid.len() {
            match grid[y][x] {
                b'^' => {
                    num_splits += 1;
                    beam_queue.push_back((x - 1, y));
                    beam_queue.push_back((x + 1, y));
                    break;
                }
                b'|' => break,
                _ => {},
            }
            grid[y][x] = b'|';
        }
    }

    println!("Number of splits: {num_splits}");

    Ok(())
}

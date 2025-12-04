use std::{io, iter};

fn main() -> io::Result<()> {
    let mut width = 0;
    let mut height = 2;
    let mut grid: Vec<u8> = Vec::new();

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        if width == 0 {
            width = line.len() + 2;
            println!("width: {width}");
            grid.extend(iter::repeat_n(0, width));
        }
        assert_eq!(width, line.len() + 2);
        grid.push(0);
        grid.extend(line.chars().map(|c| if c == '@' { 1 } else { 0 }));
        grid.push(0);
        height += 1;
    }
    grid.extend(iter::repeat_n(0, width));

    let mut total_reachable = 0;
    let mut reachable_first_iteration = 0;
    let mut iterations = 0;
    loop {
        let mut num_reachable: u32 = 0;
        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                if grid[y * width + x] == 0 {
                    print!(".");
                    continue;
                }
                let neighbors: u8 = (-1..=1)
                    .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
                    .filter(|&(dx, dy)| dx != 0 || dy != 0)
                    .map(|(dx, dy)| grid[y.strict_add_signed(dy) * width + x.strict_add_signed(dx)])
                    .sum();
                if neighbors < 4 {
                    num_reachable += 1;
                    grid[y * width + x] = 0;
                    print!("x");
                } else {
                    print!("@");
                }
            }
            println!();
        }

        if iterations == 0 {
            reachable_first_iteration = num_reachable;
        }
        iterations += 1;
        if num_reachable == 0 {
            break;
        }
        total_reachable += num_reachable;
    }

    println!("Reachable first iter: {reachable_first_iteration}");
    println!("Reachable total: {total_reachable}");
    println!("Iterations: {iterations}");
    Ok(())
}

use std::{array, io};

fn calculate_area(a: &[u64; 2], b: &[u64; 2]) -> u64 {
    let width = a[0].abs_diff(b[0]) + 1;
    let height = a[1].abs_diff(b[1]) + 1;
    width * height
}

fn main() -> io::Result<()> {
    let mut points = Vec::new();

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut it = line.splitn(2, ',').map(|s| s.parse().unwrap());
        let point: [u64; 2] = array::from_fn(|_| it.next().unwrap());
        points.push(point);
    }

    // Compute all areas
    let mut max_area = 0;
    for (i, pi) in points.iter().enumerate() {
        for (_j, pj) in points.iter().enumerate().skip(i + 1) {
            let area = calculate_area(pi, pj);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Result: {max_area}");
    Ok(())
}

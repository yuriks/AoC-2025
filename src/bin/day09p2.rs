use std::{array, io};
use std::fs::File;
use std::io::Write;

fn calculate_area(a: &[u64; 2], b: &[u64; 2]) -> u64 {
    let width = a[0].abs_diff(b[0]) + 1;
    let height = a[1].abs_diff(b[1]) + 1;
    width * height
}

fn line_intersects_rect(&[left, top]: &[u64; 2], &[right, bottom]: &[u64; 2], la: &[u64; 2], lb: &[u64; 2]) -> bool {
    if la[0] != lb[0] && la[1] == lb[1] {
        // Horizontal edge
        let [xa, y] = *la;
        let [xb, _] = *lb;
        if xb > xa {
            // Top edge (going right)
            !(y <= top || y > bottom || xb <= left || xa >= right)
        } else {
            // Bottom edge (going left)
            !(y < top || y >= bottom || xb >= right || xb <= left)
        }
    } else if la[0] == lb[0] && la[1] != lb[1] {
        // Vertical edge
        let [x, ya] = *la;
        let [_, yb] = *lb;
        if yb > ya {
            // Right edge (going down)
            !(x < left || x >= right || yb <= top || ya >= bottom)
        } else {
            // Left edge (going up)
            !(x <= left || x > right || yb >= bottom || ya <= top)
        }
    } else {
        panic!("Non-orthogonal or degenerate edge");
    }
}

fn minmax<T: Ord>(a: T, b: T) -> [T; 2] {
    if b < a { [b, a] } else { [a, b] }
}

fn _rasterize(points: &[[usize; 2]]) {
    let min_x = points.iter().map(|p| p[0]).min().unwrap() - 1;
    let min_y = points.iter().map(|p| p[1]).min().unwrap() - 1;
    let max_x = points.iter().map(|p| p[0]).max().unwrap() + 1;
    let max_y = points.iter().map(|p| p[1]).max().unwrap() + 1;

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    println!("w:{width} ({min_x}-{max_x}), h:{height} ({min_y}-{max_y})");
    //return;

    let mut fb = vec![vec![b'.'; width as usize]; height as usize];
    let mut prev_x = points.last().unwrap()[0];
    let mut prev_y = points.last().unwrap()[1];

    for [px, py] in points.iter().copied() {
        if py == prev_y {
            let y = py - min_y;
            // Horizontal line
            let [l_edge, r_edge] = minmax(prev_x - min_x, px - min_x);
            fb[y][l_edge] = b'#';
            if l_edge < r_edge {
                fb[y][r_edge] = b'#';
                fb[y][l_edge + 1..r_edge].fill(b'-');
            }
        } else if px == prev_x {
            let x = px - min_x;
            // Vertical line
            let [t_edge, b_edge] = minmax(prev_y - min_y, py - min_y);
            fb[t_edge][x] = b'#';
            if t_edge < b_edge {
                fb[b_edge][x] = b'#';
                for y in t_edge + 1..b_edge {
                    fb[y][x] = b'|';
                }
            }
        } else {
            panic!("Non-orthogonal lines");
        }
        prev_x = px;
        prev_y = py;
    }

    let mut outf = File::create("raster.txt").unwrap();
    for line in fb {
        outf.write_all(&line).unwrap();
        outf.write_all(b"\n").unwrap();
    }
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

    // for p in &mut points {
    //     p[0] /= 500;
    //     p[1] /= 500;
    // }
    // rasterize(&points);

    // Compute all areas
    let mut max_area = 0;
    for (i, pi) in points.iter().copied().enumerate() {
        'j_loop: for j in i+1..points.len() {
            let pj = points[j];

            let area = calculate_area(&pi, &pj);
            if area <= max_area {
                continue;
            }

            let [min_x, max_x] = minmax(pi[0], pj[0]);
            let [min_y, max_y] = minmax(pi[1], pj[1]);
            let rect_a = [min_x, min_y];
            let rect_b = [max_x, max_y];

            let mut prev_p = points.last().unwrap();
            for p in points.iter() {
                if line_intersects_rect(&rect_a, &rect_b, prev_p, p) {
                    continue 'j_loop;
                }
                prev_p = p;
            }

            println!("New max: {pi:?}-{pj:?} (area: {area})");
            max_area = area;
        }
    }

    println!("Result: {max_area}");
    Ok(())
}

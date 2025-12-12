use std::{array, io};
use std::fmt::{Debug, Formatter, Write};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Shape([[bool; 3]; 3]);

impl Shape {
    fn at(&self, x: usize, y: usize) -> bool {
        self.0[y][x]
    }
    fn rotate_cw(&self) -> Shape {
        let s = self;
        Shape([
            [s.at(0, 2), s.at(0, 1), s.at(0, 0)],
            [s.at(1, 2), s.at(1, 1), s.at(1, 0)],
            [s.at(2, 2), s.at(2, 1), s.at(2, 0)],
        ])
    }

    fn flipped(&self) -> Shape {
        let s = self;
        Shape([
            [s.at(2, 0), s.at(1, 0), s.at(0, 0)],
            [s.at(2, 1), s.at(1, 1), s.at(0, 1)],
            [s.at(2, 2), s.at(1, 2), s.at(0, 2)],
        ])
    }
}

impl Debug for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Shape(")?;
        for (y, row) in self.0.iter().enumerate() {
            for c in row.iter() {
                f.write_char(if *c { '#' } else { '.' })?;
            }
            if y != 2 {
                f.write_str(", ")?;
            }
        }
        f.write_str(")")?;
        Ok(())
    }
}

#[derive(Debug)]
struct ShapeInfo {
    rotations: [Option<Shape>; 4],
    flipped_rots: [Option<Shape>; 4],
    area: u32,
}

#[derive(Debug)]
struct Problem {
    dim: [u32; 2],
    shape_counts: Vec<u32>,
}

fn create_rotations(mut shape: Shape) -> ([Option<Shape>; 4], [Option<Shape>; 4]) {
    let mut next_rotation = |i| {
        let cur = shape;
        shape = shape.rotate_cw();
        if i == 3 {
            shape = shape.flipped();
        }
        Some(cur)
    };

    let mut rotations: [Option<Shape>; 4] = array::from_fn(&mut next_rotation);
    let mut flipped_rots: [Option<Shape>; 4] = array::from_fn(&mut next_rotation);

    for i in 0..rotations.len() {
        if rotations[..i].contains(&rotations[i]) {
            rotations[i] = None;
        }
    }
    for i in 0..flipped_rots.len() {
        if rotations.contains(&rotations[i]) || flipped_rots[..i].contains(&rotations[i]) {
            flipped_rots[i] = None;
        }
    }

    (rotations, flipped_rots)
}

fn read_input() -> io::Result<(Vec<ShapeInfo>, Vec<Problem>)> {
    let mut shapes: Vec<ShapeInfo> = Vec::new();
    let mut problems: Vec<Problem> = Vec::new();

    let mut lines = io::stdin().lines();
    while let Some(line) = lines.next() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut it = line.split_ascii_whitespace();
        let descriptor = it.next().unwrap().strip_suffix(':').unwrap();
        let shape_counts = it.map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();

        if shape_counts.is_empty() {
            let shape_id = descriptor.parse::<usize>().unwrap();
            assert_eq!(shape_id, shapes.len());

            let cells = Shape(array::from_fn(|_| {
                let line_s = lines.next().unwrap().unwrap();
                let mut line_chars = line_s.chars().map(|c| c == '#');
                let shape_line = array::from_fn(|_| line_chars.next().unwrap());
                assert_eq!(line_chars.next(), None);
                shape_line
            }));

            let (rotations, flipped_rots) = create_rotations(cells);
            let area = cells.0.iter().flatten().filter(|c| **c).count() as u32;

            shapes.push(ShapeInfo {rotations, flipped_rots, area });
            //println!("{:?}", shapes.last().unwrap());

            let empty_line = lines.next().unwrap()?;
            assert!(empty_line.is_empty());
        } else {
            let (w, h) = descriptor.split_once('x').unwrap();
            let dim = [w.parse().unwrap(), h.parse().unwrap()];

            problems.push(Problem { dim, shape_counts });
        }
    }

    Ok((shapes, problems))
}

fn main() -> io::Result<()> {
    let (shapes, problems) = read_input()?;

    let mut counter = 0u32;
    for p in &problems {
        let problem_area = p.dim[0] * p.dim[1];
        let total_shape_area: u32 = p.shape_counts.iter().zip(&shapes).map(|(count, info)| count * info.area).sum();
        if total_shape_area <= problem_area {
            println!("delta {:6}; {}x{} area {problem_area}, shape area {total_shape_area}", problem_area - total_shape_area, p.dim[0], p.dim[1]);
            counter += 1;
        }
    }
    println!("Feasible cases: {counter}");

    Ok(())
}

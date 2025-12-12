use std::{io, iter};
use microlp::{ComparisonOp, OptimizationDirection, Problem};

trait BitSetOps {
    fn bit(&self, i: usize) -> bool;
    fn set_bit(&mut self, i: usize);
    fn clear_bit(&mut self, i: usize);

    fn iter_bits(&self) -> impl Iterator<Item=usize>;
}

impl BitSetOps for u32 {
    fn bit(&self, i: usize) -> bool {
        *self >> i & 1 != 0
    }

    fn set_bit(&mut self, i: usize) {
        *self |= 1 << i;
    }

    fn clear_bit(&mut self, i: usize) {
        *self &= !(1 << i);
    }

    fn iter_bits(&self) -> impl Iterator<Item=usize> {
        let mut x = *self;
        iter::from_fn(move || {
            if x == 0 {
                return None;
            }
            let next = x.trailing_zeros();
            // Clear lowest bit
            x &= x - 1;
            Some(next as usize)
        })
    }
}

#[derive(Debug)]
struct Machine {
    _num_lights: u32,
    // Lights are represented as bitfields
    desired_lights: u32,
    buttons: Vec<u32>,
    joltages: Vec<u32>,
}

fn read_input_line(line: String) -> Option<Machine> {
    let mut it = line.split(' ').peekable();

    let lights_str = it.next()?.strip_prefix('[')?.strip_suffix(']')?;
    let num_lights = u32::try_from(lights_str.chars().count()).ok()?;
    let desired_lights = lights_str.chars()
        .enumerate()
        .filter_map(|(i, c)| (c == '#').then_some(i))
        .fold(0u32, |val, bit_i| val | 1 << bit_i);

    let mut buttons = Vec::new();
    while it.peek()?.starts_with('(') {
        let button_str = it.next()?.strip_prefix('(')?.strip_suffix(')')?;
        let button = button_str.split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .inspect(|i| assert!(*i < num_lights))
            .fold(0u32, |val, bit_i| val | 1 << bit_i);
        buttons.push(button);
    }

    let joltage_str = it.next()?.strip_prefix('{')?.strip_suffix('}')?;
    let joltages = joltage_str.split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    Some(Machine {
        _num_lights: num_lights,
        desired_lights,
        buttons,
        joltages,
    })
}

fn main() -> io::Result<()> {
    let mut machines = Vec::new();

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        machines.push(read_input_line(line).unwrap());
    }

    let mut result_accum = 0;

    for m in machines {
        let mut p = Problem::new(OptimizationDirection::Minimize);
        let vars = (0..m.buttons.len())
            .map(|_| p.add_integer_var(1.0, (0, i32::MAX)))
            .collect::<Vec<_>>();
        for (jolt_i, joltage) in m.joltages.iter().copied().enumerate() {
            let c = m.buttons.iter().zip(&vars)
                .filter_map(|(b, var)| b.bit(jolt_i).then_some((*var, 1.0f64)));
            p.add_constraint(c, ComparisonOp::Eq, f64::from(joltage));
        }

        let solution = p.solve().unwrap();
        let result = solution.objective();
        println!("Best sum: {}", result);
        assert!((result - result.round()).abs() <= 0.0001);
        result_accum += result.round() as u32;
    }

    println!("Result sum: {result_accum}");
    Ok(())
}

use std::{io, iter};

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
    buttons: Vec<u32>,
    joltages: Vec<u32>,
}

fn read_input_line(line: String) -> Option<Machine> {
    let mut it = line.split(' ').peekable();

    let lights_str = it.next()?.strip_prefix('[')?.strip_suffix(']')?;
    let num_lights = u32::try_from(lights_str.chars().count()).ok()?;

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
        buttons,
        joltages,
    })
}

struct BacktrackingState {
    mtx: Vec<Vec<f64>>,
    mtx_free_vars: u32, // bitset
    mtx_dependency_vars: Vec<u32>, // bitset

    /// Indexed by variable, which equations it affects
    columns: Vec<u32>, // bitset
    /// Indexed by equation, set of which variables affect it
    lines: Vec<u32>, // bitset

    remaining_constant: Vec<u32>,
    free_vars: u32, // bitset
    fixed_values: Vec<u32>,

    current_sum: u32,
    best_sum: u32,
    indent: u32,
}

const ZERO_EPSILON: f64 = 0.000001;

fn _print_matrix(mtx: &[Vec<f64>]) {
    for r in mtx {
        print!("[");
        for x in r[..r.len() - 1].iter() {
            print!(" {x:4.1}");
        }
        println!(" |{:5.1}]", r.last().unwrap());
    }
    println!();
}
fn gauss_jordan_reduction(mtx: &mut [Vec<f64>]) -> u32 {
    let num_cols = mtx[0].len();
    for r in &*mtx {
        assert_eq!(r.len(), num_cols);
    }
    let mut free_variables = (1 << num_cols - 1) - 1;

    let mut pivot = 0;
    for c in 0..mtx[0].len() - 1 {
        let new_pivot = mtx.iter()
            .map(|r| r[c])
            .enumerate()
            .skip(pivot)
            .filter(|(_, x)| x.abs() >= ZERO_EPSILON)
            .min_by(|(_, x), (_, y)| (x.abs() - 1.0).abs().total_cmp(&(y.abs() - 1.0).abs()));
        if let Some((new_pivot, scale)) = new_pivot {
            mtx.swap(new_pivot, pivot);
            mtx[pivot].iter_mut().for_each(|x| *x /= scale);

            for r in 0..mtx.len() {
                let Ok([row, p]) = mtx.get_disjoint_mut([r, pivot]) else { continue };
                let scale = row[c];
                if scale.abs() < ZERO_EPSILON {
                    continue;
                }
                row.iter_mut().zip(p).for_each(|(x, p)| *x -= *p * scale);
            }
            free_variables.clear_bit(c);
            pivot += 1;
        }
    }

    free_variables
}

fn expand_bitmatrix(bitlines: &[u32], width: usize, constants: &[u32]) -> Vec<Vec<f64>> {
    bitlines.iter().zip(constants).map(|(bits, constant)| {
        (0..width).map(|i| if bits.bit(i) { 1.0 } else { 0.0 })
            .chain(iter::once(f64::from(*constant))).collect()
    }).collect()
}

macro_rules! dbg_println {
    ($self:expr, $($arg:tt)*) => (#[cfg(debug_assertions)] { $self.indent(); println!($($arg)*) });
}

impl BacktrackingState {
    fn with_constrained_var<T>(&mut self, var_i: usize, value: u32, f: impl FnOnce(&mut Self) -> T) -> Option<T> {
        assert!(self.free_vars.bit(var_i));
        dbg_println!(self, "Attempt fix of var {var_i} to {value}");
        self.indent += 1;

        if self.current_sum + value >= self.best_sum {
            dbg_println!(self, "< {} worse than best sum ({})", self.current_sum + value, self.best_sum);
            self.indent -= 1;
            return None;
        }

        // Reduce constants in affected equations
        for equ_i in self.columns[var_i].iter_bits() {
            if value > self.remaining_constant[equ_i] {
                dbg_println!(self, "< Value ({value}) exceeds constant {equ_i} ({})", self.remaining_constant[equ_i]);
                // Impossible state, revert changes and backtrack
                let traversed_set = self.columns[var_i] & (1 << equ_i) - 1;
                for equ_j in traversed_set.iter_bits() {
                    self.remaining_constant[equ_j] += value;
                }
                self.indent -= 1;
                return None;
            }
            self.remaining_constant[equ_i] -= value;
        }
        self.current_sum += value;
        self.free_vars.clear_bit(var_i);
        self.fixed_values[var_i] = value;

        let result = f(self);

        self.indent -= 1;
        dbg_println!(self, "Backtracking fixing var {var_i} to {value}");
        self.fixed_values[var_i] = u32::MAX;
        self.free_vars.set_bit(var_i);
        self.current_sum -= value;
        for equ_i in self.columns[var_i].iter_bits() {
            self.remaining_constant[equ_i] += value;
        }
        Some(result)
    }

    #[cfg(debug_assertions)]
    fn indent(&self) {
        for _ in 0..self.indent {
            print!("    ");
        }
    }
    fn log_state(&self) {
        let _vars_s = (0..self.columns.len())
            .map(|i| if self.free_vars.bit(i) { format!("{i}: _") } else { format!("{i}: {}", self.fixed_values[i]) })
            .collect::<Vec<_>>().join(", ");
        let _remaining_s = self.remaining_constant.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
        dbg_println!(self, "vars: ({_vars_s}); remaining: ({_remaining_s})");
    }

    fn calculate_any_constrained_var(&mut self) {
        if let Some((eqn_i, dep_vars)) = self.mtx_dependency_vars.iter().enumerate().find(|(_, r)| (*r & self.free_vars).count_ones() == 1) {
            let target_var = (dep_vars & self.free_vars).iter_bits().next().unwrap();
            let eqn = &self.mtx[eqn_i];
            let mut value = *eqn.last().unwrap();
            for var_i in (dep_vars & !self.free_vars).iter_bits() {
                value -= f64::from(self.fixed_values[var_i]) * eqn[var_i];
            }
            if value.round() < 0.0 {
                dbg_println!(self, "< Calculated x_{target_var} less than 0: {value}");
                return;
            }
            if (value - value.round()).abs() > ZERO_EPSILON {
                dbg_println!(self, "< Calculated x_{target_var} has non-integer solution: {value}");
                return;
            }
            self.with_constrained_var(target_var, value.round() as u32, Self::calculate_any_constrained_var);
        } else {
            self.search_solution();
        }
    }

    fn search_solution(&mut self) {
        self.log_state();

        if self.free_vars == 0 {
            // Backtracking shouldn't have allowed getting here if the sum was worse
            assert!(self.current_sum < self.best_sum);
            if self.remaining_constant.iter().all(|x| *x == 0) {
                dbg_println!(self, "-- New best: {} -> {}", self.best_sum, self.current_sum);
                self.best_sum = self.current_sum;
            } else {
                dbg_println!(self, "-- Bad solution");
            }
            return;
        }

        let var_i = (self.free_vars & self.mtx_free_vars).iter_bits().next().unwrap();
        let max_value = self.columns[var_i].iter_bits().map(|x| self.remaining_constant[x]).min().unwrap();
        for value in 0..=max_value {
            if value != 0 {
                self.log_state();
            }
            self.with_constrained_var(var_i, value, Self::calculate_any_constrained_var);
        }
    }
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
        // buttons == variables
        // joltages == constraints
        //assert!(m.buttons.len() <= MAX_VARS);

        // Transpose bit matrix
        let affecting_buttons = (0..m.joltages.len()).map(|jolt_i| {
            m.buttons.iter()
                .enumerate()
                .filter_map(|(var_i, b)| b.bit(jolt_i).then_some(var_i))
                .fold(0u32, |val, bit_i| val | 1 << bit_i)
        }).collect::<Vec<_>>();

        // Pre-solve matrix to guide integer solution search
        let mut mtx = expand_bitmatrix(&affecting_buttons, m.buttons.len(), &m.joltages);
        let mtx_free_vars = gauss_jordan_reduction(&mut mtx);
        // Remove useless matrix lines
        while let Some(_) = mtx.pop_if(|r| r.iter().all(|x| *x < ZERO_EPSILON)) {}
        // Calculate constrained variable dependencies
        let mtx_dependency_vars = mtx.iter().map(|r| {
            r[..r.len() - 1].iter().enumerate()
                .filter(|(_, x)| x.abs() >= ZERO_EPSILON)
                .fold(0u32, |vars, (c, _)| vars | (1 << c))
        }).collect::<Vec<_>>();

        let mut state = BacktrackingState {
            mtx,
            mtx_free_vars,
            mtx_dependency_vars,
            columns: m.buttons.clone(),
            lines: affecting_buttons,
            remaining_constant: m.joltages.clone(),
            free_vars: (1u32 << m.buttons.len()) - 1,
            fixed_values: vec![u32::MAX; m.buttons.len()],
            current_sum: 0,
            best_sum: u32::MAX,
            indent: 0,
        };

        println!("Matrix representation:");
        for (line, constant) in state.lines.iter().zip(&state.remaining_constant) {
            let line_s = (0..state.columns.len()).map(|i| if line.bit(i) { "1" } else { "0" }).collect::<Vec<_>>().join(" ");
            println!("[{line_s} | {constant}]");
        }

        state.calculate_any_constrained_var();
        println!("Best sum: {}", state.best_sum);
        result_accum += state.best_sum;
    }

    println!("Result sum: {result_accum}");
    Ok(())
}

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

const MAX_VARS: usize = 16;

struct BacktrackingState {
    /// Indexe by variable, which equations it affects
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

impl BacktrackingState {
    fn find_most_constrained_equation(&self) -> (usize, u32) {
        let (best_equ, equ_free_vars) = self.lines.iter()
            .map(|vars| vars & self.free_vars)
            .enumerate()
            .filter(|(_, vars)| *vars != 0)
            .min_by_key(|(_, b)| b.count_ones())
            .unwrap();
        (best_equ, equ_free_vars)
    }

    fn with_constrained_var<T>(&mut self, var_i: usize, value: u32, f: impl FnOnce(&mut Self) -> T) -> Option<T> {
        assert!(self.free_vars.bit(var_i));
        self.indent();
        println!("Attempt fix of var {var_i} to {value}");
        self.indent += 1;

        if self.current_sum + value >= self.best_sum {
            self.indent();
            println!("< {} worse than best sum ({})", self.current_sum + value, self.best_sum);
            self.indent -= 1;
            return None;
        }

        // Reduce constants in affected equations
        for equ_i in self.columns[var_i].iter_bits() {
            if value > self.remaining_constant[equ_i] {
                self.indent();
                println!("< Value ({value}) exceeds constant {equ_i} ({})", self.remaining_constant[equ_i]);
                // Impossible state, revert changes and backtrack
                let traversed_set = self.columns[var_i] & (1 << equ_i) - 1;
                for equ_j in traversed_set.iter_bits() {
                    //println!("@ Rollback {equ_i} by {value}");
                    self.remaining_constant[equ_j] += value;
                }
                self.indent -= 1;
                return None;
            }
            //println!("@ Reduce {equ_i} by {value}");
            self.remaining_constant[equ_i] -= value;
        }
        self.current_sum += value;
        self.free_vars.clear_bit(var_i);
        self.fixed_values[var_i] = value;

        let result = f(self);

        self.indent -= 1;
        self.indent();
        println!("Backtracking fixing var {var_i} to {value}");
        self.fixed_values[var_i] = u32::MAX;
        self.free_vars.set_bit(var_i);
        self.current_sum -= value;
        for equ_i in self.columns[var_i].iter_bits() {
            //println!("@ Restore {equ_i} by {value}");
            self.remaining_constant[equ_i] += value;
        }
        Some(result)
    }

    fn indent(&self) {
        for _ in 0..self.indent {
            print!("    ");
        }
    }
    fn log_state(&self) {
        let vars_s = (0..self.columns.len())
            .map(|i| if self.free_vars.bit(i) { format!("{i}: _") } else { format!("{i}: {}", self.fixed_values[i]) })
            .collect::<Vec<_>>().join(", ");
        let remaining_s = self.remaining_constant.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
        self.indent();
        println!("vars: ({vars_s}); remaining: ({remaining_s})");
    }

    fn search_solution(&mut self) {
        self.log_state();

        if self.free_vars == 0 {
            //assert_eq!(self.remaining_constant.iter().sum::<u32>(), 0);
            // Backtracking shouldn't have allowed getting here if the sum was worse
            assert!(self.current_sum < self.best_sum);
            if self.remaining_constant.iter().all(|x| *x == 0) {
                self.indent();
                println!("-- New best: {} -> {}", self.best_sum, self.current_sum);
                self.best_sum = self.current_sum;
            } else {
                self.indent();
                println!("-- Bad solution");
            }
            return;
        }

        let (best_equ, equ_free_vars) = self.find_most_constrained_equation();
        let num_vars = equ_free_vars.count_ones();

        /*loop {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();
            let in_nums = user_input.trim_ascii_end().split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
            if in_nums.is_empty() {
                break;
            }
            self.with_constrained_var(in_nums[0] as usize, in_nums[1], Self::search_solution);
            self.log_state();
        }*/

        if num_vars == 1 {
            let var_i = equ_free_vars.iter_bits().next().unwrap();
            self.with_constrained_var(var_i, self.remaining_constant[best_equ], Self::search_solution);
        } else {
            let var_i = equ_free_vars.iter_bits().next().unwrap();
            let max_value = self.columns[var_i].iter_bits().map(|x| self.remaining_constant[x]).min().unwrap();
            for value in 0..=max_value {
                if value != 0 {
                    self.log_state();
                }
                self.with_constrained_var(var_i, value, Self::search_solution);
            }
        }

    }
}

fn main() -> io::Result<()> {
    let mut machines = Vec::new();

    /*let mut pivot_line = None;
    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            pivot_line = None;
            continue;
        }
        let in_nums = line.trim_ascii_end().split_ascii_whitespace().flat_map(|s| s.parse::<i32>().ok()).collect::<Vec<_>>();
        if let Some(pivot_line) = &pivot_line {
            let res = in_nums.iter().zip(pivot_line)
                .map(|(x, piv_x)| x - piv_x)
                .map(|x: i32| x.to_string())
                .collect::<Vec<_>>().join(" ");
            println!("{res}");
        } else {
            pivot_line = Some(in_nums);
        }
    }*/

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

        let mut state = BacktrackingState {
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

        state.search_solution();
        println!("Best sum: {}", state.best_sum);
        result_accum += state.best_sum;
    }

    println!("Result sum: {result_accum}");
    Ok(())
}

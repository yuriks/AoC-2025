use std::io;

#[derive(Debug)]
struct Machine {
    _num_lights: u32,
    // Lights are represented as bitfields
    desired_lights: u32,
    buttons: Vec<u32>,
    _joltage: Vec<u32>,
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
    let _joltage = joltage_str.split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    Some(Machine {
        _num_lights: num_lights,
        desired_lights,
        buttons,
        _joltage,
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

    for machine in machines {
        //println!("Problem: {machine:#?}");
        let mut best = u32::MAX;
        for setting in 0..(1u32 << machine.buttons.len()) {
            let mut state = 0u32;
            for (i, toggled) in machine.buttons.iter().copied().enumerate() {
                if setting & 1 << i != 0 {
                    state ^= toggled;
                }
            }
            //println!("attempt: {state:0w$b} (setting {setting:0sw$b}", w = machine._num_lights as usize, sw = machine.buttons.len());
            if state == machine.desired_lights {
                //println!("match in {}", setting.count_ones());
                best = best.min(setting.count_ones());
            }
        }
        assert_ne!(best, u32::MAX);
        println!("Machine result: {best}");
        result_accum += best;
    }

    println!("Result sum: {result_accum}");
    Ok(())
}

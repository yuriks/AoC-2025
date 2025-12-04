use std::io;

fn main() -> io::Result<()> {
    let mut total_joltage = 0u32;

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let bank: Vec<_> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let msd = bank[..bank.len() - 1]
            .iter()
            .copied()
            .enumerate()
            .fold((None, 0),
                  |max, (cur_i, cur_d)| if cur_d > max.1 { (Some(cur_i), cur_d) } else { max });
        let lsd = bank
            .iter()
            .copied()
            .enumerate()
            .skip(msd.0.map_or(0, |i| i + 1))
            .fold((None, 0),
                  |max, (cur_i, cur_d)| if cur_d > max.1 { (Some(cur_i), cur_d) } else { max });
        let value = u32::from(msd.1) * 10 + u32::from(lsd.1);
        println!("Max: {value}");
        total_joltage += value;
    }

    println!("Total joltage: {total_joltage}");

    Ok(())
}

use std::io;

fn main() -> io::Result<()> {
    let mut total_joltage: u64 = 0;

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let bank: Vec<_> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        const NUM_BATTERIES: usize = 12;

        let mut value = 0;
        let mut start_i = 0;
        assert!(bank.len() >= NUM_BATTERIES);

        for iteration in 1..=NUM_BATTERIES {
            let mut max_d = 0;
            for i in start_i..(bank.len() - NUM_BATTERIES + iteration) {
                if bank[i] > max_d {
                    max_d = bank[i];
                    start_i = i + 1;
                }
            }
            value = 10 * value + u64::from(max_d);
        }

        println!("Max: {value}");
        total_joltage += value;
    }

    println!("Total joltage: {total_joltage}");

    Ok(())
}

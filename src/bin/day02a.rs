use std::io;

// Does not advance if `val` is already a double
fn advance_to_next_double(mut val: u64) -> u64 {
    let mut val_digits = val.ilog10() + 1;
    if val_digits % 2 != 0 {
        // Odd number of digits, expand to next valid even-digited number
        val_digits += 1;
        val = 10u64.pow(val_digits - 1);
    }

    let factor = 10u64.pow(val_digits / 2);
    let singlet_val = if val / factor >= val % factor {
        val / factor
    } else {
        (val / factor) + 1
    };
    singlet_val * factor + singlet_val
}

fn main() -> io::Result<()> {
    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut id_sum = 0u64;

        for range in line.split(',') {
            let (range_begin, range_end) = range.split_once('-').unwrap();

            let mut range_begin = range_begin.parse::<u64>().unwrap();
            let range_end = range_end.parse::<u64>().unwrap();

            loop {
                range_begin = advance_to_next_double(range_begin);
                if range_begin > range_end {
                    break;
                }
                id_sum += range_begin;
                print!("{range_begin},");
                range_begin += 1;
            }
            println!();
        }

        println!("Sum: {id_sum}");
    }
    Ok(())
}

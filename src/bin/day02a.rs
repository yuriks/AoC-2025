use std::collections::HashSet;
use std::io;

// Does not advance if `val` is already a repeating pattern
fn advance_to_next_double(mut val: u64, n_groups: u32) -> u64 {
    let mut val_digits = val.ilog10() + 1;
    if val_digits % n_groups != 0 {
        // Odd number of digits, expand to next valid even-digited number
        val_digits = val_digits.next_multiple_of(n_groups);
        val = 10u64.pow(val_digits - 1);
    }

    let factor = 10u64.pow(val_digits / n_groups);
    let mut best_group = 0;
    for _ in 0..n_groups {
        let next_group = val % factor;
        best_group = if next_group >= best_group {
            next_group
        } else {
            next_group + 1
        };
        val /= factor;
    }
    let mut result = 0;
    for _ in 0..n_groups {
        result = result * factor + best_group;
    }
    result
}

fn main() -> io::Result<()> {
    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut id_sum = 0u64;
        let mut id_2group_sum = 0u64;

        for range in line.split(',') {
            let (range_begin, range_end) = range.split_once('-').unwrap();

            let range_begin = range_begin.parse::<u64>().unwrap();
            let range_end = range_end.parse::<u64>().unwrap();

            let mut already_seen = HashSet::new();

            let max_digits = range_end.ilog10() + 1;
            for n_groups in 2..=max_digits {
                let mut n_groups_sum = 0;
                print!("{n_groups}-groups: ");
                let mut current = range_begin;
                loop {
                    current = advance_to_next_double(current, n_groups);
                    if current > range_end {
                        break;
                    }
                    if already_seen.insert(current) {
                        n_groups_sum += current;
                        print!("{current},");
                    } else {
                        print!("[{current}],");
                    }
                    current += 1;
                }
                println!();

                id_sum += n_groups_sum;
                if n_groups == 2 {
                    id_2group_sum += n_groups_sum;
                }
            }
        }

        println!("Sum: {id_sum}");
        println!("Sum (2-groups only): {id_2group_sum}");
    }
    Ok(())
}

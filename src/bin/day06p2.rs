use std::io;

fn main() -> io::Result<()> {
    let mut lines: Vec<Vec<u8>> = Vec::new();

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        assert!(line.is_ascii());
        lines.push(line.into_bytes());
    }

    // Pad all lines to length
    let max_length = lines.iter().map(|line| line.len()).max().unwrap();
    for l in &mut lines {
        l.resize(max_length, b' ');
    }

    let mut result_accumulator: u64 = 0;

    let ops = lines.pop().unwrap();
    let mut sum: u64 = 0;
    let mut product: u64 = 1;
    for i in (0..max_length).rev() {
        let mut term: u64 = 0;
        for l in &lines {
            match l[i] {
                b' ' => {}
                d @ b'0'..=b'9' => { term = 10 * term + u64::from(d - b'0'); }
                _ => unreachable!(),
            }
        }

        if term == 0 {
            continue;
        }
        sum += term;
        product *= term;

        match ops[i] {
            b' ' => continue,
            b'+' => result_accumulator += sum,
            b'*' => result_accumulator += product,
            _ => unreachable!(),
        };
        sum = 0;
        product = 1;
    }

    println!("Results sum: {result_accumulator}");

    Ok(())
}

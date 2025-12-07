use std::io;

fn main() -> io::Result<()> {
    let mut lines: Vec<Vec<i64>> = Vec::new();

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        if let Ok(terms) = line.split_ascii_whitespace().map(|s| s.parse::<i64>()).collect() {
            lines.push(terms);
            continue;
        }

        let mut accum: i64 = 0;
        for (i, op) in line.split_ascii_whitespace().enumerate() {
            let it = lines.iter().map(|v| v[i]);
            let result: i64 = match op {
                "+" => it.sum(),
                "*" => it.product(),
                _ => panic!("Unknown op '{op}'"),
            };

            accum += result;
        }
        println!("Results sum: {accum}");
    }

    Ok(())
}

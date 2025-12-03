use std::io;

fn main() -> io::Result<()> {
    let mut dial_pos = 50;
    let mut zero_count = 0;

    for line in io::stdin().lines() {
        let line = line?;
        let mut line_chars = line.chars();
        let movement = match line_chars.next() {
            Some('L') => -line_chars.as_str().parse::<i32>().unwrap(),
            Some('R') => line_chars.as_str().parse::<i32>().unwrap(),
            c => panic!("Unexpected line prefix: {c:?}"),
        };

        dial_pos = (dial_pos + movement).rem_euclid(100);
        if dial_pos == 0 {
            zero_count += 1;
        }
    }

    println!("{zero_count}");
    Ok(())
}

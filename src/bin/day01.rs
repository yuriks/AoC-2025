use std::io;

fn main() -> io::Result<()> {
    const FULL_TURN: i32 = 100;

    let mut dial_pos = 50i32;
    let mut zero_park_count = 0;
    let mut zero_crossings = 0;

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut line_chars = line.chars();
        let movement = match line_chars.next() {
            Some('L') => -line_chars.as_str().parse::<i32>().unwrap(),
            Some('R') => line_chars.as_str().parse::<i32>().unwrap(),
            c => panic!("Unexpected line prefix: {c:?}"),
        };

        let adjusted_dial = if movement < 0 {
            (dial_pos - 1).rem_euclid(FULL_TURN)
        } else {
            dial_pos
        };
        zero_crossings += (adjusted_dial + movement).div_euclid(FULL_TURN).abs();
        dial_pos = (dial_pos + movement).rem_euclid(FULL_TURN);
        if dial_pos == 0 {
            zero_park_count += 1;
        }
    }

    println!("Zero parks: {zero_park_count}");
    println!("Zero crossings: {zero_crossings}");
    Ok(())
}

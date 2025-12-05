use std::io;

fn main() -> io::Result<()> {
    let mut ranges = Vec::new();

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let (range_begin, range_end) = line.split_once('-').unwrap();
        let range_begin = range_begin.parse::<u64>().unwrap();
        let range_end = range_end.parse::<u64>().unwrap();
        ranges.push((range_begin, range_end + 1));
    }
    ranges.sort_unstable();
    ranges
        .iter()
        .for_each(|(begin, end)| println!("{}-{}", begin, end - 1));

    // Merge overlapping/adjacent ranges
    ranges.dedup_by(|right, left| {
        assert!(left.0 <= right.0);
        if right.0 <= left.1 {
            if left.1 < right.1 {
                left.1 = right.1;
            }
            true // discards `right`
        } else {
            false
        }
    });

    let fresh_ingredients: u64 = ranges.iter().map(|(begin, end)| end - begin).sum();
    println!();
    ranges
        .iter()
        .for_each(|(begin, end)| println!("{}-{}", begin, end - 1));
    println!("Fresh ingredients: {fresh_ingredients}");

    Ok(())
}

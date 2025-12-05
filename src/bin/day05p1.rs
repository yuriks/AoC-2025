use std::io;
use std::collections::BTreeSet;

fn main() -> io::Result<()> {
    let mut ranges = Vec::new();
    let mut ingredients = BTreeSet::new();

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let (range_begin, range_end) = line.split_once('-').unwrap();
        let range_begin = range_begin.parse::<u64>().unwrap();
        let range_end = range_end.parse::<u64>().unwrap();
        ranges.push(range_begin..=range_end);
    }

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let ingredient_id = line.parse::<u64>().unwrap();
        ingredients.insert(ingredient_id);
    }

    let mut fresh_ingredients = 0;
    for range in ranges {
        // `.count()` consumes the iterator to ensure all elements are removed
        fresh_ingredients += ingredients.extract_if(range, |_| true).count();
    }

    println!("Fresh ingredients: {}", fresh_ingredients);

    Ok(())
}

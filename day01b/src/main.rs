use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Cannot load file");

    let mut counts = content
        .split("\n\n")
        .map(|chunk| -> usize { chunk.split("\n").map(|row| row.parse().unwrap_or(0)).sum() })
        .collect::<Vec<_>>();

    counts.sort();

    // let test = counts
    println!("The highest counts are!");
    let mut total: usize = 0;

    for i in 1..4 {
        let count = counts[counts.len() - i];

        total = total + count;
        println!("[{}]: {}", i, count);
    }

    println!("Total: {total}");
}

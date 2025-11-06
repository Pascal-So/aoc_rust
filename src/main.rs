use advent::{io::file_str, y2021};

fn main() {
    println!(
        "{:?}",
        y2021::d22::solve(&file_str("data/y2021/d22_full.txt").unwrap()).unwrap()
    );
}

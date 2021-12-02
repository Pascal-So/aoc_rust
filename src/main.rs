use advent::{io::file, y2021};

fn main() {
    println!(
        "{:?}",
        y2021::d02::solve(file("data/y2021/d02_full.txt").unwrap()).unwrap()
    );
}

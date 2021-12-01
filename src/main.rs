use advent::{io::file, y2021};

fn main() {
    println!(
        "{}",
        y2021::d01::solve(file("data/2021/01.txt").unwrap()).unwrap()
    );
}

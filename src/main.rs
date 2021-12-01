use advent::{io::file, y2019};

fn main() {
    println!(
        "{:#}",
        y2019::d01::solve(file("data/2019/01.txt").unwrap()).unwrap()
    );
}

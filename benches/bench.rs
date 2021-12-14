use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent::{io::file, y2021};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve", |b| {
        b.iter(|| y2021::d09::solve(file(black_box("data/y2021/d09_full.txt")).unwrap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

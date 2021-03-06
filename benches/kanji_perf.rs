use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("group");
    group
        .sample_size(50)
        .measurement_time(Duration::from_millis(52000));
    group.bench_function("10kanji", |b| {
        b.iter(|| kanji_practice_sheet::create_pages(black_box("鬱知春項垂暗殺空想缶")))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

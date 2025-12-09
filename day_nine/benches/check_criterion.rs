use criterion::{Criterion, criterion_group, criterion_main};

fn bench_trivial(c: &mut Criterion) {
    c.bench_function("trivial_busywork", |b| {
        b.iter(|| {
            let mut s: u64 = 0;
            for i in 0..1000u64 {
                s = s.wrapping_add(i);
            }
            s
        })
    });
}

criterion_group!(benches, bench_trivial);
criterion_main!(benches);

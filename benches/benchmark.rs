use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

    puffin::set_scopes_on(true);
    c.bench_function("puffin_on", |b| {
        b.iter(|| {
            puffin::profile_function!();
        })
    });

    puffin::set_scopes_on(false);
    c.bench_function("puffin_off", |b| {
        b.iter(|| {
            puffin::profile_function!();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

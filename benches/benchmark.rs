use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn real_sine(phase: f32) -> f32 {
    phase.sin()
}

fn fake_sine(phase: f32) -> f32 {
    let x = (phase / std::f32::consts::PI) - 1.0;
    4.0 * x * (1.0 - x.abs())
}

fn bench_sine_functions(c: &mut Criterion) {
    let mut phase = 0.0;
    let increment = 0.01;

    c.bench_function("real sine", |b| {
        b.iter(|| {
            phase += increment;
            black_box(real_sine(black_box(phase)));
        })
    });

    c.bench_function("fake sine", |b| {
        b.iter(|| {
            phase += increment;
            black_box(fake_sine(black_box(phase)));
        })
    });
}

criterion_group!(benches, bench_sine_functions);
criterion_main!(benches);

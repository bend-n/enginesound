use criterion::{black_box, criterion_group, criterion_main, Criterion};
use enginesound::gen::*;
const RATE: u32 = 42000;

fn gen() -> Generator {
    let engine = Engine::new(RATE);
    let mut generator = Generator::new(RATE, engine, LowPassFilter::new(0.5, RATE));
    generator.volume = 1.0;
    generator
}

fn bench_gen(c: &mut Criterion) {
    let mut g = gen();
    c.bench_function("generation", |b| {
        b.iter(|| {
            for _ in 0..30000 {
                black_box(g.frame());
            }
        });
    });
}

criterion_group!(benches, bench_gen);
criterion_main!(benches);

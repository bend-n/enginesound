use enginesound::gen::*;
const RATE: u32 = 42000;

fn gen() -> Generator {
    let engine = Engine::new(RATE);
    let mut generator = Generator::new(RATE, engine, LowPassFilter::new(0.5, RATE));
    generator.volume = 1.0;
    generator
}

fn bench_gen() {
    let mut g = gen();
    for _ in 0..30000 {
        iai::black_box(g.frame());
    }
}

iai::main!(bench_gen);

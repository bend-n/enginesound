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
    // 30 seconds worth of sound.
    // this currently runs in 5510364722 cycles, so you need at least 200MHz to generate 30 seconds of high frequency engine sound.
    for _ in 0..RATE * 30 {
        iai::black_box(g.frame());
    }
}

iai::main!(bench_gen);

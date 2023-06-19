use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use unkr::fuzz_next_string_ruled;

fn enigma_bench(c: &mut Criterion) {
    let args = unkr::enigma_init();
    let strs = vec![String::from("HELLO")];

    c.bench_function("enigma-bench", |b| {
        b.iter(|| {
            let args_next = unkr::enigma_next(args.clone()).unwrap();
            unkr::enigma_encrypt(strs.clone(), args_next);
        })
    });
}

pub fn fuzz_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("fuzzer-group");
    group.sample_size(10);

    group.bench_function("fuzzer", |b| {
        b.iter(|| fuzz_next_string_ruled(&"KRYPTOR".to_string(), 7, 27, true, true, true))
    });
    group.finish()
}

criterion_group! {
  name = benches;
  config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
  targets = fuzz_bench, enigma_bench
}

criterion_main!(benches);

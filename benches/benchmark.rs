use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use unkr::fuzz_next_string_ruled;

fn encrypt_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("encrypt");
    group.bench_function("enigma", |b| {
        b.iter(|| {
            let args_next = unkr::enigma_next(unkr::enigma_init()).unwrap();
            unkr::enigma_encrypt(vec![String::from("HELLO")], args_next);
        })
    });

    group.bench_function("transpose", |b| {
        b.iter(|| {
            let strs = vec![String::from("HELLO")];
            let args_next = unkr::transpose_next(strs.clone(), unkr::transpose_init()).unwrap();
            unkr::transpose_decrypt(strs, args_next);
        })
    });

    group.bench_function("permute", |b| {
        b.iter(|| {
            let strs = vec![String::from("HELLO")];
            let args_next = unkr::permute_next(unkr::models::PermuteBruteForceState {
                brute_force_args: unkr::models::BruteForcePermuteArgs {
                    max_permutations: 26,
                },
                args: unkr::permute_init(),
            })
            .unwrap();
            unkr::permute_decrypt(strs, args_next);
        })
    });

    group.finish()
}

pub fn fuzz_next_bench(c: &mut Criterion) {
    c.bench_function("fuzz_next", |b| {
        b.iter(|| {
            unkr::fuzz_next(&vec![1, 0], 2, 4);
        })
    });
}

pub fn fuzz_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("fuzzer");
    group.sample_size(10);

    group.bench_function("fuzzer", |b| {
        b.iter(|| fuzz_next_string_ruled(&"KRYPTOR".to_string(), 7, 27, true, true, true))
    });
    group.finish()
}

pub fn char_position(c: &mut Criterion) {
    c.bench_function("char_pos", |b| {
        b.iter(|| {
            let _ = unkr::char_position('R');
        })
    });
}

pub fn char_position_native(c: &mut Criterion) {
    c.bench_function("char_pos_native", |b| {
        b.iter(|| {
            let _ = 'R' as u32 - 63;
        })
    });
}

criterion_group! {
  name = benches;
  config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
  targets =
    //fuzz_bench,
    encrypt_bench,
     fuzz_next_bench,
     char_position,
     char_position_native
}

criterion_main!(benches);

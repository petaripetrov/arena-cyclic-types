use criterion::{BenchmarkId, Criterion};
use rand::{seq::SliceRandom, rngs::StdRng, SeedableRng};
use thesis::{cpp_arena::ffi as cpp_bench, rust_bench};

const RUNS: usize = 11;
const STEP: usize = 1000;

fn benchmark_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("linked-list");

    for n in 1..RUNS {
        let n: usize = n * STEP;

        group.throughput(criterion::Throughput::Elements(n as u64));
        group.bench_with_input(
            BenchmarkId::new("linked-list-cpp-arena", n),
            &n,
            |b, &n| b.iter(|| cpp_bench::benchmark_linked_list_arena(n)),
        );

        group.bench_with_input(
            BenchmarkId::new("linked-list-cpp-manual", n),
            &n,
            |b, &n| b.iter(|| cpp_bench::benchmark_linked_list_manual(n)),
        );

        group.bench_with_input(
            BenchmarkId::new("linked-list-rust-arena", n),
            &n,
            |b, &n| b.iter(|| rust_bench::benchmark_linked_list_arena(n)),
        );
    }
}

fn benchmark_tree(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(2023);

    let mut group = c.benchmark_group("tree");

    for n in 1..RUNS {
        let n: usize = n * STEP;
        let mut rand_arr: Vec<u32> = Vec::with_capacity(n);

        for x in 0..n {
            rand_arr.push(x as u32);
        }

        rand_arr.shuffle(&mut rng);

        group.throughput(criterion::Throughput::Elements(n as u64));
        group.bench_with_input(
            BenchmarkId::new("tree-cpp-arena", n),
            &n,
            |b, &n| b.iter(|| cpp_bench::benchmark_tree_arena(n, rand_arr.clone())),
        );

        group.bench_with_input(
            BenchmarkId::new("tree-cpp-manual", n),
            &n,
            |b, &_n| b.iter(|| cpp_bench::benchmark_tree_manual(rand_arr.clone())),
        );

        group.bench_with_input(
            BenchmarkId::new("tree-rust-arena", n),
            &n,
            |b, &n| b.iter(|| rust_bench::benchmark_tree_arena(n, rand_arr.clone())),
        );
    }
}

criterion::criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = benchmark_list, benchmark_tree
}
criterion::criterion_main!(benches);

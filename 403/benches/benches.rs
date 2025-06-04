use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use frog_jump::solve::{
    dfs_solve, iterative_solve, par_dfs_solve, par_solve, recursive_solve,
};

pub fn bench(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(3)
        .build_global()
        .unwrap();

    const SIZE: usize = 1_000_000;

    let callees: [(_, fn(_) -> _); 5] = [
        ("dfs", dfs_solve),
        ("iterative", iterative_solve),
        ("par", par_solve),
        ("dfs (par)", par_dfs_solve),
        ("recursive", recursive_solve),
    ];

    let mut group = c.benchmark_group("full of stones");
    let input = vec![true; SIZE];

    for (name, callee) in callees.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            callee,
            |b, callee| b.iter(|| callee(&input)),
        );
    }

    group.finish();

    let mut group = c.benchmark_group("no stones");
    let input = vec![false; SIZE];

    for (name, callee) in callees.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            callee,
            |b, callee| b.iter(|| callee(&input)),
        );
    }

    group.finish();

    let mut group = c.benchmark_group("alternative jumps");
    let input = [true, false, true, false, true];
    assert_eq!(SIZE % input.len(), 0);
    let input = input.into_iter().cycle().take(SIZE).collect::<Vec<_>>();

    for (name, callee) in callees.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            callee,
            |b, callee| b.iter(|| callee(&input)),
        );
    }

    group.finish();

    let mut group = c.benchmark_group("bad alternative jumps");
    let input = [true, false, true, false, true];
    assert_eq!(SIZE % input.len(), 0);

    let input = input
        .into_iter()
        .cycle()
        .take(SIZE)
        .chain([false, false, false, true])
        .collect::<Vec<_>>();

    for (name, callee) in callees.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            callee,
            |b, callee| b.iter(|| callee(&input)),
        );
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

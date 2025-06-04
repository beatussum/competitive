use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use frog_jump::{
    Input,
    solve::{
        dfs_solve, iterative_solve, par_dfs_solve, par_solve, recursive_solve,
    },
};

pub fn bench(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
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
    let has_stone = vec![true; SIZE];
    let input = Input::new(&has_stone, (0, 1));

    for (name, callee) in callees.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            callee,
            |b, callee| b.iter(|| callee(input)),
        );
    }

    group.finish();

    let mut group = c.benchmark_group("no stones");
    let has_stone = vec![false; SIZE];
    let input = Input::new(&has_stone, (0, 1));

    for (name, callee) in callees.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            callee,
            |b, callee| b.iter(|| callee(input)),
        );
    }

    group.finish();

    let mut group = c.benchmark_group("alternative jumps");
    let has_stone = [true, false, true, false, true];

    assert_eq!(SIZE % has_stone.len(), 0);

    let has_stone =
        has_stone.into_iter().cycle().take(SIZE).collect::<Vec<_>>();

    let input = Input::new(&has_stone, (0, 1));

    for (name, callee) in callees.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            callee,
            |b, callee| b.iter(|| callee(input)),
        );
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

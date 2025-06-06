use std::{error::Error, fs::File, path::Path};

use criterion::{
    BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
};

use frog_jump::{
    Input, parse_input,
    solve::{
        dfs_solve, iterative_solve, par_dfs_solve, par_solve, recursive_solve,
        walk_tree_solve,
    },
};

use serde_json::{Map, Value};

fn parse_file(filename: &str) -> Result<Vec<bool>, Box<dyn Error>> {
    let path = format!(
        "{}/inputs/{}",
        Path::new(file!())
            .parent()
            .ok_or(format!("`{}` has no parent", file!()))?
            .to_str()
            .ok_or("The file path cannot be converted to `str`")?,
        filename
    );

    let file = File::open(path)?;

    let input =
        &serde_json::from_reader::<_, Map<String, Value>>(file)?["input"];

    let input = input
        .as_array()
        .ok_or("`input` is not an array")?
        .into_iter()
        .filter_map(|index| index.as_u64())
        .map(|index| index as usize);

    Ok(parse_input(input))
}

pub fn bench(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    const SIZE: usize = 10_000;

    let callees: [(_, fn(_) -> _); 6] = [
        ("dfs", dfs_solve),
        ("iterative", iterative_solve),
        ("par", par_solve),
        ("dfs (par)", par_dfs_solve),
        ("recursive", recursive_solve),
        ("walk tree", walk_tree_solve),
    ];

    let mut group = c.benchmark_group("full of stones");
    let has_stone = vec![true; SIZE];
    let input = Input::new(&has_stone, (0, 1));

    group.throughput(Throughput::Elements(SIZE as u64));

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

    group.throughput(Throughput::Elements(SIZE as u64));

    for (name, callee) in callees.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            callee,
            |b, callee| b.iter(|| callee(input)),
        );
    }

    group.finish();

    let mut group = c.benchmark_group("trap");

    let has_stone = parse_file("trap.json").unwrap();
    let input = Input::new(&has_stone, (0, 1));

    group.throughput(Throughput::Elements(has_stone.len() as u64));

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

    group.throughput(Throughput::Elements(SIZE as u64));

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

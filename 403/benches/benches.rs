use criterion::{Criterion, criterion_group, criterion_main};
use frog_jump::{iterative_solve, par_solve, recursive_solve};

pub fn bench(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new().build_global().unwrap();

    const SIZE: usize = 1_024;

    let callees: [(_, fn(_) -> _); 3] = [
        ("iterative", iterative_solve),
        ("par", par_solve),
        ("recursive", recursive_solve),
    ];

    let input = [true; SIZE];

    for (name, callee) in callees.iter() {
        c.bench_function(format!("full of stones ({})", name).as_str(), |b| {
            b.iter(|| callee(&input))
        });
    }

    let input = [false; SIZE];

    for (name, callee) in callees.iter() {
        c.bench_function(format!("no stones ({})", name).as_str(), |b| {
            b.iter(|| callee(&input))
        });
    }

    let input = [true, false, true, false, true];

    let input = input
        .into_iter()
        .cycle()
        .take(input.len() * SIZE)
        .collect::<Vec<_>>();

    for (name, callee) in callees.iter() {
        c.bench_function(
            format!("alternative jump ({})", name).as_str(),
            |b| b.iter(|| callee(&input)),
        );
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);

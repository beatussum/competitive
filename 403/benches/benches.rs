use criterion::{
    BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
};

use frog_jump::{
    Input,
    solve::{
        dfs_solve, iterative_solve, par_dfs_solve, par_solve, recursive_solve,
        walk_tree_solve,
    },
};

pub fn bench(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    const SIZE: usize = 1_000_000;

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

    let index = vec![
        0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153,
        171, 190, 210, 231, 253, 276, 300, 325, 350, 351, 375, 377, 400, 403,
        425, 429, 450, 455, 475, 481, 500, 507, 525, 533, 550, 558, 559, 575,
        583, 585, 600, 608, 611, 625, 633, 637, 650, 658, 663, 675, 683, 689,
        700, 708, 715, 725, 733, 741, 750, 758, 766, 767, 775, 783, 783, 791,
        791, 799, 800, 801, 808, 816, 823, 824, 825, 826, 827, 833, 841, 847,
        848, 849, 850, 851, 852, 853, 858, 866, 873, 874, 875, 876, 877, 883,
        891, 899, 900, 901, 908, 916, 925, 933, 941, 958, 966, 983, 983, 991,
        1008, 1015, 1016, 1017, 1033, 1039, 1040, 1041, 1042, 1043, 1058, 1063,
        1064, 1065, 1066, 1067, 1068, 1069, 1083, 1089, 1090, 1091, 1092, 1093,
        1108, 1115, 1116, 1117, 1133, 1141, 1158, 1183, 1207, 1208, 1209, 1231,
        1232, 1233, 1234, 1235, 1255, 1256, 1257, 1258, 1259, 1260, 1261, 1281,
        1282, 1283, 1284, 1285, 1307, 1308, 1309, 1333,
    ];

    let size = index.last().copied().unwrap() + 1;
    let mut has_stone = vec![false; size];

    for i in index {
        has_stone[i] = true;
    }

    let input = Input::new(&has_stone, (0, 1));

    group.throughput(Throughput::Elements(size as u64));

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

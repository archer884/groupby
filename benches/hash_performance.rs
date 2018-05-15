#[macro_use]
extern crate criterion;

extern crate fnv;
extern crate fxhash;
extern crate groupby;
extern crate metrohash;

use criterion::Criterion;
use groupby::GroupBy;
use std::collections::{hash_map::RandomState, HashMap};

type Grouping<T> = HashMap<i32, Vec<i32>, T>;

fn group_default(seq: &[i32]) -> Grouping<RandomState> {
    seq.iter().cloned().group_by(|&x| x)
}

fn group_fnv(seq: &[i32]) -> Grouping<fnv::FnvBuildHasher> {
    seq.iter()
        .cloned()
        .group_by_with_hasher(|&x| x, fnv::FnvBuildHasher::default())
}

fn group_fx(seq: &[i32]) -> Grouping<fxhash::FxBuildHasher> {
    seq.iter()
        .cloned()
        .group_by_with_hasher(|&x| x, fxhash::FxBuildHasher::default())
}

fn group_metro(seq: &[i32]) -> Grouping<metrohash::MetroBuildHasher> {
    seq.iter()
        .cloned()
        .group_by_with_hasher(|&x| x, metrohash::MetroBuildHasher::default())
}

fn group_default_criterion(c: &mut Criterion) {
    let seq: Vec<i32> = (1..=10).cycle().take(1000).collect();
    c.bench_function("GroupBy Default Hash", move |b| b.iter(|| group_default(&seq)));
}

fn group_fnv_criterion(c: &mut Criterion) {
    let seq: Vec<i32> = (1..=10).cycle().take(1000).collect();
    c.bench_function("GroupBy FNV Hash", move |b| b.iter(|| group_fnv(&seq)));
}

fn group_fx_criterion(c: &mut Criterion) {
    let seq: Vec<i32> = (1..=10).cycle().take(1000).collect();
    c.bench_function("GroupBy FX Hash", move |b| b.iter(|| group_fx(&seq)));
}

fn group_metro_criterion(c: &mut Criterion) {
    let seq: Vec<i32> = (1..=10).cycle().take(1000).collect();
    c.bench_function("GroupBy Metro Hash", move |b| b.iter(|| group_metro(&seq)));
}

criterion_group!(
    benches,
    group_default_criterion,
    group_fnv_criterion,
    group_fx_criterion,
    group_metro_criterion
);

criterion_main!(benches);

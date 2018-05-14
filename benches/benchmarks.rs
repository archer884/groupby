#![feature(test)]

extern crate fnv;
extern crate fxhash;
extern crate groupby;
extern crate metrohash;
extern crate test;

use groupby::GroupBy;
use test::Bencher;

#[bench]
fn default(b: &mut Bencher) {
    let seq: Vec<i32> = (1..=10).cycle().take(1000).collect();
    b.iter(|| test::black_box(seq.iter().cloned().group_by(|&x| x)));
}

#[bench]
fn fnv_hash(b: &mut Bencher) {
    let seq: Vec<i32> = (1..=10).cycle().take(1000).collect();
    b.iter(|| {
        test::black_box(
            seq.iter()
                .cloned()
                .group_by_with_hasher(|&x| x, fnv::FnvBuildHasher::default()),
        )
    });
}

#[bench]
fn fx_hash(b: &mut Bencher) {
    let seq: Vec<i32> = (1..=10).cycle().take(1000).collect();
    b.iter(|| {
        test::black_box(
            seq.iter()
                .cloned()
                .group_by_with_hasher(|&x| x, fxhash::FxBuildHasher::default()),
        )
    });
}

#[bench]
fn metro_hash(b: &mut Bencher) {
    let seq: Vec<i32> = (1..=10).cycle().take(1000).collect();
    b.iter(|| {
        test::black_box(
            seq.iter()
                .cloned()
                .group_by_with_hasher(|&x| x, metrohash::MetroBuildHasher::default()),
        )
    });
}

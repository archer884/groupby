#![feature(test)]

extern crate groupby;
extern crate test;

use groupby::GroupBy;
use test::Bencher;

#[bench]
fn measure_1_10_1000(b: &mut Bencher) {
    let seq: Vec<i32> = (1..=10).cycle().take(1000).collect();
    b.iter(|| {
        test::black_box(seq.iter().cloned().group_by(|&x| x))
    });
}

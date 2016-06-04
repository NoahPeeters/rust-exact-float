#![feature(test)]
extern crate test;
extern crate exact_float;

use test::Bencher;
use exact_float::ExactFloat;

#[bench]
fn bench_add(b: &mut Bencher) {
    b.iter(|| {
        let f = ExactFloat::new(10);
        let n = f.clone() + 1;
        let p = f * n;
        p.calculate().unwrap();
    });
}

#[bench]
fn test_mul(b: &mut Bencher) {
    b.iter(|| {
        let f = ExactFloat::new(10);
        let n = f.clone() * 2;
        let p = f * n;
        p.calculate().unwrap();
    });
}

#[bench]
fn test_div(b: &mut Bencher) {
    b.iter(|| {
        let f = ExactFloat::new(10);
        let mut n = f.clone() / 3;
        n *= 33;

        n.pre_calculate();
        n.calculate().unwrap();
    });
}

#[bench]
fn test_neg(b: &mut Bencher) {
    b.iter(|| {
        let f = ExactFloat::new(-10);
        let f = f.clone() * 2;
        let n = f.clone() * -3;
        let n = -n;
        n.abs().calculate().unwrap();
    });
}

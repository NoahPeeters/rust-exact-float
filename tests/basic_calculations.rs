extern crate exact_float;

use exact_float::ExactFloat;

#[test]
fn test_add() {
    let f = ExactFloat::new(10);
    let n = f.clone() + 1;
    let p = f * n;
    assert_eq!(p.calculate().unwrap(), 110.0);
}

#[test]
fn test_mul() {
    let f = ExactFloat::new(10);
    let n = f.clone() * f.clone() / 2;
    let p = f * n;
    assert_eq!(p.calculate().unwrap(), 500.0);
}

#[test]
fn test_div() {
    let f = ExactFloat::new(10);
    let mut n = f.clone() / 3;
    n *= 33;
    assert_eq!(n.calculate().unwrap(), 110.0);

    n.pre_calculate();
    assert_eq!(n.calculate().unwrap(), 110.0);
}

#[test]
fn test_neg() {
    let f = ExactFloat::new(-10);
    let f = f.clone() * 2;
    let n = f.clone() * -3;
    assert_eq!(n.calculate().unwrap(), 60.0);
    let n = -n;
    assert_eq!(n.calculate().unwrap(), -60.0);
    assert_eq!(n.abs().calculate().unwrap(), 60.0);
}

#[test]
fn test_float() {
    let mut f = ExactFloat::new(2.0f64);
    println!("{:?}", f);
    f.pre_calculate();
    println!("{:?}", f);
    assert_eq!(f.calculate().unwrap(), 2.0);
}

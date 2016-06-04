extern crate num;
use num::integer::Integer;
use num::{BigInt, One};
use num::cast::ToPrimitive;
use num::bigint::ToBigInt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

fn reduce(a: &BigInt, b: &BigInt) -> (BigInt, BigInt) {
    let gcd = a.gcd(&b);
    (a / gcd.clone(), b / gcd)
}

pub trait ExactFloatNumber: std::marker::Sized {
    fn from_integer(self) -> ExactFloat;
}

impl ExactFloatNumber for ExactFloat {
    fn from_integer(self) -> ExactFloat {self}
}

impl ExactFloatNumber for u8 {
    fn from_integer(self) -> ExactFloat {ExactFloat::from_bigint(self.to_bigint().unwrap())}
}

impl ExactFloatNumber for u16 {
    fn from_integer(self) -> ExactFloat {ExactFloat::from_bigint(self.to_bigint().unwrap())}
}

impl ExactFloatNumber for u32 {
    fn from_integer(self) -> ExactFloat {ExactFloat::from_bigint(self.to_bigint().unwrap())}
}

impl ExactFloatNumber for u64 {
    fn from_integer(self) -> ExactFloat {ExactFloat::from_bigint(self.to_bigint().unwrap())}
}

impl ExactFloatNumber for i8 {
    fn from_integer(self) -> ExactFloat {ExactFloat::from_bigint(self.to_bigint().unwrap())}
}

impl ExactFloatNumber for i16 {
    fn from_integer(self) -> ExactFloat {ExactFloat::from_bigint(self.to_bigint().unwrap())}
}

impl ExactFloatNumber for i32 {
    fn from_integer(self) -> ExactFloat {ExactFloat::from_bigint(self.to_bigint().unwrap())}
}

impl ExactFloatNumber for i64 {
    fn from_integer(self) -> ExactFloat {ExactFloat::from_bigint(self.to_bigint().unwrap())}
}


#[derive(Clone, Debug)]
pub struct ExactFloat {
    numerator: BigInt,
    denominator: BigInt
}

impl ExactFloat {
    pub fn new<T: ExactFloatNumber>(x: T) -> ExactFloat {
        x.from_integer()
    }
    fn from_bigint(a: BigInt) -> ExactFloat {
        ExactFloat {
            numerator: a,
            denominator: One::one()
        }
    }
    // fn get_sign(&self) -> f64 {
    //     let sign_bit = (self.numerator < Zero::zero()) ^ (self.denominator < Zero::zero());
    //     if sign_bit {-1.0} else {1.0}
    // }
    pub fn pre_calculate(&mut self) {
        let (numerator, denominator) = reduce(&self.numerator, &self.denominator);
        self.numerator = numerator;
        self.denominator = denominator;
    }
    pub fn calculate(&self) -> Option<f64> {
        let (numerator, denominator) = reduce(&self.numerator, &self.denominator);
        match (numerator.to_f64(), denominator.to_f64()) {
            (Some(numerator), Some(denominator)) => Some(numerator / denominator),
            _ => None
        }
    }
}

impl<T: ExactFloatNumber> Add<T> for ExactFloat {
    type Output = ExactFloat;

    fn add(self, other: T) -> ExactFloat {
        let as_exact_float = other.from_integer();
        ExactFloat {
            numerator: as_exact_float.denominator.clone() * self.numerator + self.denominator.clone() * as_exact_float.numerator,
            denominator: self.denominator * as_exact_float.denominator
        }
    }
}

impl<T: ExactFloatNumber> AddAssign<T> for ExactFloat {
    fn add_assign(&mut self, other: T) {
        let as_exact_float = other.from_integer();
        self.numerator = as_exact_float.denominator.clone() * self.numerator.clone() + self.denominator.clone() * as_exact_float.numerator;
        self.denominator = self.denominator.clone() * as_exact_float.denominator;
    }
}

impl<T: ExactFloatNumber> Sub<T> for ExactFloat {
    type Output = ExactFloat;

    fn sub(self, other: T) -> ExactFloat {
        let as_exact_float = other.from_integer();
        ExactFloat {
            numerator: as_exact_float.denominator.clone() * self.numerator - self.denominator.clone() * as_exact_float.numerator,
            denominator: self.denominator * as_exact_float.denominator
        }
    }
}

impl<T: ExactFloatNumber> SubAssign<T> for ExactFloat {
    fn sub_assign(&mut self, other: T) {
        let as_exact_float = other.from_integer();
        self.numerator = as_exact_float.denominator.clone() * self.numerator.clone() - self.denominator.clone() * as_exact_float.numerator;
        self.denominator = self.denominator.clone() * as_exact_float.denominator;
    }
}

impl<T: ExactFloatNumber> Mul<T> for ExactFloat {
    type Output = ExactFloat;

    fn mul(self, other: T) -> ExactFloat {
        let as_exact_float = other.from_integer();
        ExactFloat {
            numerator: self.numerator * as_exact_float.numerator,
            denominator: self.denominator * as_exact_float.denominator
        }
    }
}

impl<T: ExactFloatNumber> MulAssign<T> for ExactFloat {
    fn mul_assign(&mut self, other: T) {
        let as_exact_float = other.from_integer();
        self.numerator = self.numerator.clone() * as_exact_float.numerator;
        self.denominator = self.denominator.clone() * as_exact_float.denominator;
    }
}

impl<T: ExactFloatNumber> Div<T> for ExactFloat {
    type Output = ExactFloat;

    fn div(self, other: T) -> ExactFloat {
        let as_exact_float = other.from_integer();
        ExactFloat {
            numerator: self.numerator * as_exact_float.denominator,
            denominator: self.denominator * as_exact_float.numerator
        }
    }
}

impl<T: ExactFloatNumber> DivAssign<T> for ExactFloat {
    fn div_assign(&mut self, other: T) {
        let as_exact_float = other.from_integer();
        self.numerator = self.numerator.clone() * as_exact_float.denominator;
        self.denominator = self.denominator.clone() * as_exact_float.numerator;
    }
}

impl Neg for ExactFloat {
    type Output = ExactFloat;

    fn neg(self) -> ExactFloat {
        ExactFloat {
            numerator: -self.numerator,
            denominator: self.denominator
        }
    }
}


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
    let n = f.clone() * 2;
    let p = f * n;
    assert_eq!(p.calculate().unwrap(), 200.0);
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

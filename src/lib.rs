#![feature(float_extras)]
extern crate num;

use num::integer::Integer;
use num::{BigInt, One, abs};
use num::cast::ToPrimitive;
use num::bigint::ToBigInt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

fn reduce(a: &BigInt, b: &BigInt) -> (BigInt, BigInt) {
    let gcd = a.gcd(&b);
    (a / gcd.clone(), b / gcd)
}

pub trait ExactFloatNumber {
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

impl ExactFloatNumber for f32 {
    fn from_integer(self) -> ExactFloat {
        let (mantissa, exponent, sign) = self.integer_decode();
        let (numerator, denominator) = if exponent > 0 {
            (mantissa * 2u64.pow(exponent as u32), 1)
        }
        else {
            (mantissa, 2u64.pow((-exponent) as u32))
        };
        ExactFloat {
            numerator: (numerator * sign as u64).to_bigint().unwrap(),
            denominator: denominator.to_bigint().unwrap()
        }
    }
}

impl ExactFloatNumber for f64 {
    fn from_integer(self) -> ExactFloat {
        let (mantissa, exponent, sign) = self.integer_decode();
        println!("{:?}, {:?}", mantissa, exponent);
        let (numerator, denominator) = if exponent > 0 {
            (mantissa * 2u64.pow(exponent as u32), 1)
        }
        else {
            (mantissa, 2u64.pow((-exponent) as u32))
        };
        ExactFloat {
            numerator: (numerator * sign as u64).to_bigint().unwrap(),
            denominator: denominator.to_bigint().unwrap()
        }
    }
}


#[derive(Clone, Debug)]
pub struct ExactFloat {
    pub numerator: BigInt,
    pub denominator: BigInt
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
    pub fn abs(&self) -> ExactFloat {
        ExactFloat {
            numerator: abs(self.numerator.clone()),
            denominator: abs(self.denominator.clone())
        }
    }
}

impl<T: ExactFloatNumber> Add<T> for ExactFloat {
    type Output = ExactFloat;

    fn add(self, other: T) -> ExactFloat {
        let as_exact_float = other.from_integer();
        ExactFloat {
            numerator: &as_exact_float.denominator * self.numerator + &self.denominator * as_exact_float.numerator,
            denominator: self.denominator * as_exact_float.denominator
        }
    }
}

impl<T: ExactFloatNumber> AddAssign<T> for ExactFloat {
    fn add_assign(&mut self, other: T) {
        let as_exact_float = other.from_integer();
        self.numerator = &as_exact_float.denominator * &self.numerator + &self.denominator * as_exact_float.numerator;
        self.denominator = &self.denominator * as_exact_float.denominator;
    }
}

impl<T: ExactFloatNumber> Sub<T> for ExactFloat {
    type Output = ExactFloat;

    fn sub(self, other: T) -> ExactFloat {
        let as_exact_float = other.from_integer();
        ExactFloat {
            numerator: &as_exact_float.denominator * self.numerator - &self.denominator * as_exact_float.numerator,
            denominator: &self.denominator * &as_exact_float.denominator
        }
    }
}

impl<T: ExactFloatNumber> SubAssign<T> for ExactFloat {
    fn sub_assign(&mut self, other: T) {
        let as_exact_float = other.from_integer();
        self.numerator = &as_exact_float.denominator * &self.numerator - &self.denominator * as_exact_float.numerator;
        self.denominator = &self.denominator * &as_exact_float.denominator;
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
        self.numerator = &self.numerator * as_exact_float.numerator;
        self.denominator = &self.denominator * as_exact_float.denominator;
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
        self.numerator = &self.numerator * as_exact_float.denominator;
        self.denominator = &self.denominator * as_exact_float.numerator;
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

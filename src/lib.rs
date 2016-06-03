extern crate num;
use num::integer::Integer;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

fn reduce(a: u64, b: u64) -> (u64, u64) {
    let gcd = a.gcd(&b);
    (a / gcd, b / gcd)
}

pub struct ExactFloat {
    sign: bool,
    numerator: u64,
    denominator: u64
}

impl ExactFloat {
    pub fn from_integer(x: i64) -> ExactFloat {
        ExactFloat {
            sign: x < 0,
            numerator: x.abs() as u64,
            denominator: 1
        }
    }
    fn get_sign(&self) -> f64 {
        if self.sign {-1.0} else {1.0}
    }
    // pub fn pre_calculate(&mut self) {
    //     (self.numerator, self.denominator) = reduce(self.numerator, self.denominator);
    // }
    pub fn calculate(&self) -> f64 {
        let (numerator, denominator) = reduce(self.numerator, self.denominator);
        numerator as f64 / denominator as f64 * self.get_sign()
    }
}

// impl Add for ExactFloat {
//     type Output = ExactFloat;
//
//     fn add(self, other: ExactFloat) -> ExactFloat {
//         ExactFloat {
//             sign: self.sign ^ other.sign,
//             numerator: self.numerator * other.denominator + other.numerator * self.denominator,
//             denominator: self.denominator * other.denominator
//         }
//     }
// }
//
// impl Add<i64> for ExactFloat {
//     type Output = ExactFloat;
//
//     fn add(self, other: i64) -> ExactFloat {
//         let extended_other = self.denominator * other.abs() as u64;
//         ExactFloat {
//             sign: self.sign ^ (extended_other > ),
//             numerator: self.numerator + self.denominator * other.abs() as u64,
//             denominator: self.denominator
//         }
//     }
// }

impl AddAssign<i64> for ExactFloat {
    fn add_assign(&mut self, other: i64) {
        self.sign ^= other < 0;
        self.numerator = self.numerator + self.denominator * other.abs() as u64;
    }
}

impl Div<i64> for ExactFloat {
    type Output = ExactFloat;

    fn div(self, other: i64) -> ExactFloat {
        ExactFloat {
            sign: self.sign ^ (other < 0),
            numerator: self.numerator,
            denominator: self.denominator * other.abs() as u64
        }
    }
}


#[test]
fn it_works() {
    let f = ExactFloat::from_integer(10);

    assert_eq!((f/2+1).calculate(), 6.0);
}

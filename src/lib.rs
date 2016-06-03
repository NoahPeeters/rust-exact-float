
#![allow(dead_code)]
struct ExactFloat {
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
    pub fn calculate(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

#[test]
fn it_works() {
     assert_eq!(ExactFloat::from_integer(10).calculate(), 10.0);
}

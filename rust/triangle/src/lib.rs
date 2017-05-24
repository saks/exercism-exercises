extern crate num;

use num::{Zero, Num};

pub struct Triangle<T>([T; 3]);

fn is_valid<T: Copy + Num + PartialOrd>(sides: &[T; 3]) -> bool {
    let a = sides[0];
    let b = sides[1];
    let c = sides[2];

    if Zero::is_zero(&a) || Zero::is_zero(&b) || Zero::is_zero(&c) {
        return false;
    }

    if a > (b + c) || b > (a + c) || c > (a + b) {
        return false;
    }

    true
}

impl<T: Copy + Num + PartialOrd> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Result<Self, ()> {
        if !is_valid(&sides) {
            return Err(());
        }
        Ok(Triangle(sides))
    }

    pub fn is_scalene(&self) -> bool {
        let &Triangle(ref sides) = self;

        let a = &sides[0];
        let b = &sides[1];
        let c = &sides[2];

        (a != b) && (b != c) && (c != a)
    }

    pub fn is_isosceles(&self) -> bool {
        let &Triangle(ref sides) = self;

        let a = &sides[0];
        let b = &sides[1];
        let c = &sides[2];

        (a == b) || (b == c) || (c == a)
    }

    pub fn is_equilateral(&self) -> bool {
        let &Triangle(ref sides) = self;

        let a = &sides[0];
        let b = &sides[1];
        let c = &sides[2];

        (a == b) && (b == c) && (c == a)
    }
}

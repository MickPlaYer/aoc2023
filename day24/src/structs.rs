use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt,
    pub z: BigInt,
}
impl From<Vec<BigInt>> for Point {
    fn from(value: Vec<BigInt>) -> Self {
        Self {
            x: value[0].clone(),
            y: value[1].clone(),
            z: value[2].clone(),
        }
    }
}

#[derive(Debug)]
pub struct Hailstone {
    pub position: Point,
    pub velocity: Point,
}
impl Hailstone {
    pub(crate) fn new(position: Vec<BigInt>, velocity: Vec<BigInt>) -> Self {
        Self {
            position: position.into(),
            velocity: velocity.into(),
        }
    }

    pub(crate) fn get_line(&self) -> Line {
        let position = self.position.clone();
        let velocity = self.velocity.clone();
        Line {
            x: position.x,
            y: position.y,
            delta_x: velocity.x,
            delta_y: velocity.y,
        }
    }

    pub(crate) fn get_cross_xy(&self, other: &Self) -> (Rational, Rational) {
        self.get_line().get_cross_xy(other.get_line())
    }

    pub(crate) fn will_through(&self, point: (&Rational, &Rational)) -> bool {
        let a = point.0.eq_by(&self.position.x) && self.velocity.x == Zero::zero()
            || point.0.gte_by(&self.position.x) && self.velocity.x > Zero::zero()
            || point.0.lte_by(&self.position.x) && self.velocity.x < Zero::zero();
        let b = point.1.eq_by(&self.position.y) && self.velocity.y == Zero::zero()
            || point.1.gte_by(&self.position.y) && self.velocity.y > Zero::zero()
            || point.1.lte_by(&self.position.y) && self.velocity.y < Zero::zero();
        a && b
    }
}

#[derive(Debug)]
pub struct Line {
    x: BigInt,
    y: BigInt,
    delta_x: BigInt,
    delta_y: BigInt,
}

impl Line {
    pub(crate) fn get_cross_xy(&self, other: Self) -> (Rational, Rational) {
        let delta_a = Rational(self.delta_y.clone(), self.delta_x.clone()).simplify();
        let const_a = Rational(self.y.clone(), One::one())
            .sub(&delta_a.mult(&Rational(self.x.clone(), One::one())))
            .simplify();
        let delta_b = Rational(other.delta_y, other.delta_x).simplify();
        let const_b = Rational(other.y, One::one())
            .sub(&delta_b.mult(&Rational(other.x, One::one())))
            .simplify();
        let x = const_b.sub(&const_a).div(&delta_a.sub(&delta_b));
        let y = delta_a.mult(&x).add(&const_a);
        (x, y)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rational(BigInt, BigInt);
impl Rational {
    fn div(&self, other: &Self) -> Self {
        self.mult(&Rational(other.1.clone(), other.0.clone()))
    }

    fn mult(&self, other: &Self) -> Self {
        let a = Self(self.0.clone(), other.1.clone()).simplify();
        let b = Self(other.0.clone(), self.1.clone()).simplify();
        Self(a.0 * b.0, a.1 * b.1)
    }

    fn sub(&self, other: &Self) -> Self {
        let left = self.0.clone() * other.1.clone();
        let right = other.0.clone() * self.1.clone();
        let new = Self(left - right, self.1.clone() * other.1.clone());
        new.simplify()
    }

    fn add(&self, other: &Self) -> Self {
        let left = self.0.clone() * other.1.clone();
        let right = other.0.clone() * self.1.clone();
        let new = Self(left + right, self.1.clone() * other.1.clone());
        new.simplify()
    }

    fn simplify(self) -> Self {
        if self.0 == Zero::zero() || self.1 == Zero::zero() {
            return self;
        }
        let mut gcd = num::integer::gcd(self.0.abs(), self.1.abs());
        if self.1 < Zero::zero() {
            gcd = gcd * -1;
        }
        Self(self.0 / &gcd, self.1 / &gcd)
    }

    pub(crate) fn gte_by(&self, n: &BigInt) -> bool {
        if self.1 == Zero::zero() {
            return false;
        }
        self.0 >= n * self.1.clone()
    }

    pub(crate) fn lte_by(&self, n: &BigInt) -> bool {
        if self.1 == Zero::zero() {
            return false;
        }
        self.0 <= n * self.1.clone()
    }

    pub(crate) fn eq_by(&self, n: &BigInt) -> bool {
        if self.1 == Zero::zero() {
            return false;
        }
        self.0 == n * self.1.clone()
    }
}

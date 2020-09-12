use std::ops::Sub;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Div;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
const ERR: f64 = 0.0001;
const EPSILON: Tuple = Tuple {
    x: ERR,
    y: ERR,
    z: ERR,
    w: ERR,
};
const ZERO: Tuple = Tuple {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};

impl Tuple {
    fn point(x: f64, y: f64, z: f64) -> Tuple {
        let w: f64 = 1.0;
        Tuple::new(x, y, z, w)
    }
    fn vector(x: f64, y: f64, z: f64) -> Tuple {
        let w: f64 = 0.0;
        Tuple::new(x, y, z, w)
    }
    fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }
    #[allow(clippy::float_cmp)]
    fn is_point(&self) -> bool {
        self.w == 1.0
    }
    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
    fn eq(self, other: Self) -> bool {
        (self - other) < EPSILON
    }
}

fn magnitude(t: Tuple) -> f64 {
    let xs = t.x * t.x;
    let ys = t.y * t.y;
    let zs = t.z * t.z;
    let sum = xs + ys + zs;
    sum.sqrt()
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Tuple {
        Tuple{
                x: self.x * -1.0,
                y: self.y * -1.0,
                z: self.z * -1.0,
                w: self.w * -1.0,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // initial tests
    #[test]
    fn test_page30() {
        assert_eq!(
            Tuple {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 4.0
            },
            Tuple {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 4.0
            }
        );

        let p = Tuple::new(4.30, -4.2, 3.1, 1.0);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p.w, 1.0);
        assert_eq!(p.is_point(), true);
        assert_eq!(p.is_vector(), false);

        let p = Tuple::new(4.30, -4.2, 3.1, 0.0);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p.w, 0.0);
        assert_eq!(p.is_point(), false);
        assert_eq!(p.is_vector(), true);
    }

    #[test]
    fn test_point() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0))
    }

    #[test]
    fn test_comparison() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let b = Tuple::new(1.0, 2.0, 3.0, 4.0 + (ERR / 2.0));
        assert!(Tuple::eq(a, b.clone()));
        let c = Tuple::new(4.0, 3.0, 2.0, 1.0);
        assert!(Tuple::eq(b, c));
    }

    // adding two tuples
    #[test]
    fn test_add() {
        let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a + b, Tuple::new(1.0,1.0,6.0,1.0));
    }

    // subtracing two points
    #[test]
    fn test_sub() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0,6.0,7.0);
        assert_eq!(p1 - p2, Tuple::vector(-2.0,-4.0,-6.0));
    }

    // subtracting a vector from a point
    #[test]
    fn test_sub_vec() {
        let p = Tuple::point(3.0,2.0,1.0);
        let v = Tuple::vector(5.0,6.0,7.0);
        assert_eq!(p - v, Tuple::point(-2.0,-4.0,-6.0))
    }

    // subtracting two vectors
    #[test]
    fn test_sub_vecs() {
        let v1 = Tuple::vector(3.0,2.0,1.0);
        let v2 = Tuple::vector(5.0,6.0,7.0);
        assert_eq!(v1-v2, Tuple::vector(-2.0,-4.0,-6.0));
    }

    // subtracting a vector from the zero vector
    #[test]
    fn test_sub_a_vec_from_zero() {
        let z = ZERO.clone();
        let v = Tuple::vector(1.0,-2.0,-3.0);
        assert_eq!(z - v, Tuple::vector(-1.0,2.0,3.0));
    }

    // negating a tuple
    #[test]
    fn test_neg() {
        let a = Tuple::new(1.0,-2.0,3.0,-4.0);
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    // multiplying a tuple by a scalar
    #[test]
    fn test_scalar_mult() {
        let a = Tuple::new(1.0,-2.0,3.0,-4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5,-7.0,10.5,-14.0))
    }

    #[test]
    fn test_fraction_mult() {
        let a = Tuple::new(1.0,-2.0,3.0,-4.0);
        assert_eq!(a * 0.5, Tuple::new(0.5,-1.0,1.5,-2.0))
    }

    #[test]
    fn test_dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0,-2.0,3.0,-4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5,-1.0,1.5,-2.0))
    }

    #[test]
    fn test_magnitude() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(magnitude(v), 1.0)
    }

    #[test]
    fn test_magnitude2() {
        let v = Tuple::vector(0.0,1.0,0.0);
        assert_eq!(magnitude(v), 1.0)
    }

    #[test]
    fn test_magnitude3() {
        let v = Tuple::vector(0.0,0.0,1.0);
        assert_eq!(magnitude(v), 1.0)
    }
    #[test]
    fn test_magnitude4() {
        let v = Tuple::vector(1.0,2.0,3.0);
        assert_eq!(magnitude(v), 14.0_f64.sqrt())
    }
    #[test]
    fn test_magnitude5() {
        let v = Tuple::vector(-1.0,-2.0,-3.0);
        assert_eq!(magnitude(v), 14.0_f64.sqrt())
    }
}

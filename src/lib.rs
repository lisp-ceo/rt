use std::ops::Sub;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
const ERR: f64 = 0.0001;
const EPSILON: Tuple = Tuple {
    x: ERR,
    y: ERR,
    z: ERR,
    w: ERR,
};


impl Tuple {
    pub const ZERO: Tuple = Tuple {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        let w: f64 = 1.0;
        Tuple::new(x, y, z, w)
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
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

pub fn normalize(t: Tuple) -> Tuple {
    let magt = magnitude(t);
    Tuple{
        x: t.x / magt,
        y: t.y / magt,
        z: t.z / magt,
        w: t.w / magt
    }
}

fn dot(a: Tuple, b: Tuple) -> f64 {
    a.x * b.x +
        a.y * b.y +
        a.z * b.z +
        a.w * b.w
}

fn cross(a: Tuple, b: Tuple) -> Tuple {
    Tuple::vector(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    return Color{
        red,
        green,
        blue,
    };
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
        let z = Tuple::ZERO.clone();
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
    #[test]
    fn test_normalizing1() {
        let v = Tuple::vector(4.0,0.0,0.0);
        assert_eq!(normalize(v), Tuple::vector(1.0,0.0,0.0))
    }
    #[test]
    fn test_normalizing2() {
        let v = Tuple::vector(1.0,2.0,3.0);
        assert!(Tuple::eq(normalize(v), Tuple::vector(0.26726,0.53452,0.80178)))
    }
    #[test]
    fn test_normalizing3() {
        let v = Tuple::vector(1.0,2.0,3.0);
        let norm = normalize(v);
        assert_eq!(magnitude(norm), 1.0)
    }
    #[test]
    fn test_dot_product() {
        let a = Tuple::vector(1.0,2.0,3.0);
        let b = Tuple::vector(2.0,3.0,4.0);
        assert_eq!(dot(a,b), 20.0)
    }
    #[test]
    fn test_cross_product() {
        let a = Tuple::vector(1.0,2.0,3.0);
        let b = Tuple::vector(2.0,3.0,4.0);
        assert_eq!(cross(a,b), Tuple::vector(-1.0,2.0,-1.0));
        assert_eq!(cross(b,a), Tuple::vector(1.0,-2.0,1.0));
    }
    #[test]
    fn test_colors() {
        let c = color(-0.5,0.4,1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }
    #[test]
    fn test_colors_add() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, color(1.6,0.7,1.0))
    }
    #[test]
    fn test_mult_scalar() {
        let c = color(0.2, 0.3, 0.4);
        assert_eq!(c * 2, color(0.4,0.6,0.8))
    }
    #[test]
    fn test_mult_colors() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, color(0.9,0.2,0.04))
    }
}

#![feature(clamp)]

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

const ERR: f64 = 0.001;
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

#[derive(Debug, Copy, Clone, PartialOrd)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let rd = self.red - other.red;
        if rd > ERR {
            return false;
        }
        let gd = self.green - other.green;
        if gd > ERR {
            return false;
        }
        let bd = self.blue - other.blue;
        if bd > ERR {
            return false;
        }
        true
    }
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
    Color{
        red,
        green,
        blue,
    }
}

pub struct Canvas {
    pub pixels: Vec<Vec<Color>>,
    pub width: i64,
    pub height: i64
}

pub fn canvas(width: i64, height: i64) -> Canvas {
    let mut pixels: Vec<Vec<Color>> = Vec::new();
    // colunn-major order
    for _ in 0..width {
        let mut col: Vec<Color> = Vec::new();
        for _ in 0..height {
            col.push(color(0.0, 0.0, 0.0));
        }
        pixels.push(col)
    }
    Canvas{
        pixels,
        width,
        height,
    }
}

pub fn write_pixel(canvas: &mut Canvas, x: i64, y: i64, color: Color) {
    canvas.pixels[x as usize][y as usize] = color
}

pub fn pixel_at(canvas: &mut Canvas, x: i64, y: i64) -> Color {
    canvas.pixels[x as usize][y as usize]
}

impl PartialEq for Canvas {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..self.pixels.len() {
            for y in 0..self.pixels[x].len() {
                if self.pixels[x as usize][y as usize] != other.pixels[x as usize][y as usize] {
                    return false;
                }
            }
        }
        true
    }
}

pub fn canvas_to_ppm(c: Canvas) -> String {
    let mut ppm = String::new();
    let version = "P3";
    let max_color = "255";
    ppm.push_str(version);
    ppm.push('\n');
    ppm.push_str(&format!("{} {}", c.width, c.height));
    ppm.push('\n');
    ppm.push_str(max_color);
    ppm.push('\n');

    let mut tmp = String::new();
    let mut leading = true;
    // row-major traversal of
    // column-major matrix
    for x in 0..c.height {
        let mut row: Vec<Color> = Vec::new();
        for col in c.pixels.iter() {
            row.push(col[x as usize]);
        }
        for pixel in row.iter() {
            let components = [pixel.red, pixel.green, pixel.blue];
            for b in components.iter() {
                let clamped = byte_clamp(*b);
                let mut formatted = format!(" {}", clamped);
                if leading {
                    formatted = format!("{}", clamped);
                    leading = false;
                }
                if tmp.len() + formatted.len() > 70 {
                    tmp.push('\n');
                    ppm.push_str(tmp.as_str());
                    tmp = String::new();
                    formatted = format!("{}", clamped);
                }
                tmp.push_str(formatted.as_str());
            }
        }
        tmp.push('\n');
        leading = true;
        ppm.push_str(tmp.as_str());
        tmp = String::new();
    }
    ppm
}

pub fn byte_clamp(x: f64) -> i64 {
    (x * 255.0).clamp(0.0, 255.0).round() as i64
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
        assert_eq!(c * 2.0, color(0.4,0.6,0.8))
    }
    #[test]
    fn test_mult_colors() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, color(0.9,0.2,0.04))
    }
    #[test]
    fn test_canvas() {
        let c = canvas(10,20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for x in 0..c.pixels.len() {
            for y in 0..c.pixels[x].len() {
                assert_eq!(c.pixels[x as usize][y as usize], color(0.0, 0.0, 0.0));
            }

        }
    }
    #[test]
    fn test_writing_pixels() {
        let mut c = canvas(10, 20);
        let red = color(1.0, 0.0, 0.0);
        write_pixel(&mut c, 2, 3, red);
        assert_eq!(pixel_at(&mut c, 2, 3), red);
    }

    #[test]
    fn test_costructing_the_ppm_header() {
        let c = canvas(5, 3);
        let ppm = canvas_to_ppm(c);
        let want = r#"P3
5 3
255
"#;
        let mut first_four = String::new();
        for line in ppm.lines().take(3) {
            first_four.push_str(line);
            first_four.push('\n');
        }

        assert_eq!(first_four, want)
    }

    #[test]
    fn test_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = color(1.5, 0.0, 0.0);
        let c2 = color(0.0, 0.5, 0.0);
        let c3 = color(-0.5, 0.0, 1.0);
        write_pixel(&mut c, 0, 0, c1);
        write_pixel(&mut c, 2, 1, c2);
        write_pixel(&mut c, 4, 2, c3);
        let ppm = canvas_to_ppm(c);
        let mut four_to_six = String::new();
        let want = r#"255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"#;
        for line in ppm.lines().skip(3).take(4) {
            four_to_six.push_str(line);
            four_to_six.push('\n');
        }
        assert_eq!(four_to_six, want)
    }

     #[test]
     fn test_splitting_long_lines_in_ppm() {
         let mut c = canvas(10, 2);
         let col = color(1.0, 0.8, 0.6);
         for x in 0..c.pixels.len() {
             for y in 0..c.pixels[x].len() {
                 write_pixel(&mut c, x as i64, y as i64, col);
             }
         }
         let want = r#"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
"#;
         let ppm = canvas_to_ppm(c);
         let mut four_to_seven = String::new();
         for line in ppm.lines().skip(3).take(4) {
             four_to_seven.push_str(line);
             four_to_seven.push('\n');
         }
         assert_eq!(four_to_seven, want)
     }
}

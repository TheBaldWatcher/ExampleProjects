use crate::common::{color::Color, utils::clamp};

use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use std::iter::Sum;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Default, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn new_min(a: &Self, b: &Self) -> Self {
        Self {
            x: a.x.min(b.x),
            y: a.y.min(b.y),
            z: a.z.min(b.z),
        }
    }

    pub fn new_max(a: &Self, b: &Self) -> Self {
        Self {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
            z: a.z.max(b.z),
        }
    }

    pub fn random_in_unit_box() -> Self {
        Self::new(
            thread_rng().gen_range(-1.0, 1.0),
            thread_rng().gen_range(-1.0, 1.0),
            thread_rng().gen_range(-1.0, 1.0),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in_unit_box();
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    // hongfendong must_use?
    #[must_use]
    pub fn random_unit() -> Self {
        let a: f64 = thread_rng().gen_range(0.0, 2 as f64 * PI);
        let z: f64 = thread_rng().gen_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Self::new(r * a.cos(), r * a.sin(), z)
    }

    #[must_use]
    pub fn random_unit_dir(dir: &Self) -> Self {
        let u = Self::random_unit();
        if u.dot(dir) > 0.0 {
            u
        } else {
            -u
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        // i    j   k
        // x1   y1  z1
        // x2   y2  z3
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn unit(&self) -> Self {
        self / self.length()
    }

    // colorize a unit-length-vector
    pub fn into_color(mut self, samples: usize, gamma: bool) -> Color {
        if 1 != samples {
            self /= samples as f64;
        }
        if gamma {
            self.x = self.x.sqrt();
            self.y = self.y.sqrt();
            self.z = self.z.sqrt();
        }
        Color::newf(
            clamp(self.x, 0.0..=1.0),
            clamp(self.y, 0.0..=1.0),
            clamp(self.z, 0.0..=1.0),
        )
    }

    // 反向
    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
}

///////// Neg //////////
impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}
////////// Index //////////
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 can only index by 0~2, {} provided", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 can only index by 0~2, {} provided", index),
        }
    }
}

////////// Add //////////
impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        &self + rhs
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        &self + &rhs
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        // an implementation of `std::ops::AddAssign` might be missing for `&mut Vec
        // self += &rhs;
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

////////// Sub //////////
impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        &self - rhs
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        &self - &rhs
    }
}

impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        // an implementation of `std::ops::SubAssign` might be missing for `&mut Vec
        // self += &rhs;
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

////////// Mul //////////
impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        &rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

////////// Div //////////
impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, val| acc + val)
    }
}

// impl Sum for Vec3 {
//     fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
//         iter.fold(Self::default(), |acc, val| acc + val)
//     }
// }

////////// UT //////////
#[test]
fn test_neg() {
    {
        let vec3 = Vec3::new(3.0, 5.0, 7.0);
        assert_eq!(-vec3, Vec3::new(-3.0, -5.0, -7.0));
    }
    {
        let vec3 = Vec3::new(11.0, 13.0, 17.0);
        assert_eq!(-&vec3, Vec3::new(-11.0, -13.0, -17.0));
    }
}

#[test]
fn test_index() {
    {
        let vec3 = Vec3::new(3.0, 5.0, 7.0);
        assert_eq!(vec3[0], 3.0);
        assert_eq!(vec3[1], 5.0);
        assert_eq!(vec3[2], 7.0);
        let result = std::panic::catch_unwind(|| vec3[5]);
        assert!(result.is_err())
    }
    {
        let mut vec3 = Vec3::new(3.0, 5.0, 7.0);
        vec3[0] = 11.0;
        vec3[1] = 13.0;
        vec3[2] = 17.0;
        assert_eq!(vec3[0], 11.0);
        assert_eq!(vec3[1], 13.0);
        assert_eq!(vec3[2], 17.0);
        let result = std::panic::catch_unwind(move || vec3[5] = 9.0);
        assert!(result.is_err())
    }
}

#[test]
fn test_add() {
    {
        let vec3_1 = Vec3::new(3.0, 5.0, 7.0);
        let vec3_11 = Vec3::new(3.0, 5.0, 7.0);
        let vec3_2 = Vec3::new(11.0, 13.0, 17.0);
        let vec3_22 = Vec3::new(11.0, 13.0, 17.0);
        // & + &
        assert_eq!(&vec3_1 + &vec3_2, Vec3::new(14.0, 18.0, 24.0));
        // & + _
        assert_eq!(&vec3_1 + vec3_2, Vec3::new(14.0, 18.0, 24.0));
        // _ + &
        assert_eq!(vec3_11 + &vec3_22, Vec3::new(14.0, 18.0, 24.0));
        // _ + _
        assert_eq!(vec3_1 + vec3_22, Vec3::new(14.0, 18.0, 24.0));
    }
    {
        let mut vec_lhs = Vec3::new(3.0, 5.0, 7.0);
        let vec_rhs = Vec3::new(11.0, 13.0, 17.0);
        vec_lhs += &vec_rhs;
        assert_eq!(vec_lhs, Vec3::new(14.0, 18.0, 24.0));
        vec_lhs += vec_rhs;
        assert_eq!(vec_lhs, Vec3::new(25.0, 31.0, 41.0));
    }
}

#[test]
fn test_sub() {
    {
        let vec3_1 = Vec3::new(3.0, 5.0, 7.0);
        let vec3_11 = Vec3::new(3.0, 5.0, 7.0);
        let vec3_2 = Vec3::new(11.0, 13.0, 17.0);
        let vec3_22 = Vec3::new(11.0, 13.0, 17.0);
        // & + &
        assert_eq!(&vec3_1 - &vec3_2, Vec3::new(-8.0, -8.0, -10.0));
        // & + _
        assert_eq!(&vec3_1 - vec3_2, Vec3::new(-8.0, -8.0, -10.0));
        // _ + &
        assert_eq!(vec3_11 - &vec3_22, Vec3::new(-8.0, -8.0, -10.0));
        // _ + _
        assert_eq!(vec3_1 - vec3_22, Vec3::new(-8.0, -8.0, -10.0));
    }
    {
        let mut vec_lhs = Vec3::new(3.0, 5.0, 7.0);
        let vec_rhs = Vec3::new(11.0, 13.0, 17.0);
        vec_lhs -= &vec_rhs;
        assert_eq!(vec_lhs, Vec3::new(-8.0, -8.0, -10.0));
        vec_lhs -= vec_rhs;
        assert_eq!(vec_lhs, Vec3::new(-19.0, -21.0, -27.0));
    }
}

#[test]
fn test_mul_between_f64_and_vec3() {
    {
        let vec3_1 = Vec3::new(3.0, 5.0, 7.0);
        let vec3_2 = Vec3::new(11.0, 13.0, 17.0);
        assert_eq!(&vec3_1 * 10.0, Vec3::new(30.0, 50.0, 70.0));
        assert_eq!(vec3_1 * 10.0, Vec3::new(30.0, 50.0, 70.0));

        assert_eq!(10.0 * &vec3_2, Vec3::new(110.0, 130.0, 170.0));
        assert_eq!(10.0 * vec3_2, Vec3::new(110.0, 130.0, 170.0));
    }
    {
        let mut vec3 = Vec3::new(3.0, 5.0, 7.0);
        vec3 *= 10.0;
        assert_eq!(vec3, Vec3::new(30.0, 50.0, 70.0));
    }
}

#[test]
fn test_div() {
    {
        let vec3 = Vec3::new(30.0, 50.0, 70.0);

        assert_eq!(&vec3 / 10.0, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(vec3 / 10.0, Vec3::new(3.0, 5.0, 7.0));
    }
    {
        let mut vec3 = Vec3::new(3.0, 5.0, 7.0);
        vec3 /= 10.0;
        assert_eq!(vec3, Vec3::new(0.3, 0.5, 0.7));
    }
}

#[test]
fn test_length_squared() {
    let vec3 = Vec3::new(3.0, 4.0, 0.0);
    assert_eq!(25.0, vec3.length_squared())
}

#[test]
fn test_length() {
    let vec3 = Vec3::new(5.0, 12.0, 0.0);
    assert_eq!(13.0, vec3.length())
}

#[test]
fn test_dot() {
    let v_1 = Vec3::new(3.0, 5.0, 7.0);
    let v_2 = Vec3::new(11.0, 13.0, 17.0);

    assert_eq!(v_1.dot(&v_2), 217.0)
}

#[test]
fn test_cross() {
    let v_1 = Vec3::new(3.0, 5.0, 7.0);
    let v_2 = Vec3::new(11.0, 13.0, 17.0);

    assert_eq!(v_1.cross(&v_2), Vec3::new(-6.0, 26.0, -16.0));
}

// #[test] 如何优雅地判断float？
// fn test_unit() {
//     let v = Vec3::new(3.0, 4.0, 0.0);
//     assert_eq!(v.unit(), Vec3::new(0.6, 0.8, 0.0));
// }

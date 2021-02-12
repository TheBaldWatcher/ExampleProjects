use super::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, unit: f64) -> Point3 {
        &self.origin + &self.direction * unit
    }
}

////////// UT //////////
#[test]
fn test_at() {
    let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(ray.at(2.0), Vec3::new(2.0, 5.0, 6.0));
}

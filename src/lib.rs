use std::io::Write;
#[derive(PartialEq, Debug, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn unit_vector(&self) -> Vec3 {
        let k = 1.0 / self.length();
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
}
impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl std::ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}
impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}
pub type Point3 = Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, color: &Color) -> std::io::Result<()> {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;
    writeln!(out, "{} {} {}", ir, ig, ib)?;
    Ok(())
}

struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        // this is most likely inefficient but better than taking ownership of the Ray
        self.orig.clone() + self.dir.clone() * t
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ray_origin() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let origin_compare = origin.clone();
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray {
            orig: origin,
            dir: direction,
        };
        assert_eq!(ray.origin(), &origin_compare);
    }

    #[test]
    fn test_ray_direction() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let direction_compare = direction.clone();
        let ray = Ray {
            orig: origin,
            dir: direction,
        };
        assert_eq!(ray.direction(), &direction_compare);
    }

    #[test]
    fn test_ray_at() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray {
            orig: origin,
            dir: direction,
        };
        let t = 2.0;
        let point = ray.at(t);
        assert_eq!(point, Point3::new(9.0, 12.0, 15.0));
    }

    #[test]
    fn test_vec3_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vec3_length() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.length(), 3.0);
    }

    #[test]
    fn test_vec3_length_squared() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.length_squared(), 9.0);
    }

    #[test]
    fn test_vec3_unit_vector() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        let unit_v = v.unit_vector();
        assert!((unit_v.length() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_neg() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let neg_v = -v;
        assert_eq!(neg_v.x, -1.0);
        assert_eq!(neg_v.y, 2.0);
        assert_eq!(neg_v.z, -3.0);
    }

    #[test]
    fn test_vec3_index() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_vec3_index_out_of_bounds() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let _ = v[3];
    }

    #[test]
    fn test_vec3_index_mut() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v[0] = 4.0;
        v[1] = 5.0;
        v[2] = 6.0;
        assert_eq!(v[0], 4.0);
        assert_eq!(v[1], 5.0);
        assert_eq!(v[2], 6.0);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_vec3_index_mut_out_of_bounds() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v[3] = 4.0;
    }

    #[test]
    fn test_vec3_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 += v2;
        assert_eq!(v1.x, 5.0);
        assert_eq!(v1.y, 7.0);
        assert_eq!(v1.z, 9.0);
    }

    #[test]
    fn test_vec3_mul_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 4.0);
        assert_eq!(v.z, 6.0);
    }

    #[test]
    fn test_vec3_div_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v /= 2.0;
        assert_eq!(v.x, 0.5);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.5);
    }
}

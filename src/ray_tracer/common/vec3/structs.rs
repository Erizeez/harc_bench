use rand::Rng;

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y() * rhs.z() - self.z() * rhs.y(),
            y: self.z() * rhs.x() - self.x() * rhs.z(),
            z: self.x() * rhs.y() - self.y() * rhs.x(),
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.clone() / self.length()
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        self.clone() - 2.0 * self.dot(normal.clone()) * normal.clone()
    }

    pub fn dot(&self, v: Vec3) -> f64 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }
}

impl Color {
    pub fn random() -> Self {
        Color::new(
            rand::thread_rng().gen_range(0.0..1.0),
            rand::thread_rng().gen_range(0.0..1.0),
            rand::thread_rng().gen_range(0.0..1.0),
        )
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Color::new(
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
        )
    }
}

// handle Color to u32
impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        let r = (255.999 * color.x()) as u32;
        let g = (255.999 * color.y()) as u32;
        let b = (255.999 * color.z()) as u32;
        (r << 16) | (g << 8) | b
    }
}

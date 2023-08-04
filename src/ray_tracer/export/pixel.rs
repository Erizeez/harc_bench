use crate::ray_tracer::common::vec3::structs::Color;

#[derive(Clone, Debug)]
pub struct Pixel {
    color: Color,
    x: u32,
    y: u32,
}

impl Pixel {
    pub fn new(color: Color, x: u32, y: u32) -> Self {
        Self { color, x, y }
    }

    pub fn color(&self) -> Color {
        self.color.clone()
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

unsafe impl Send for Pixel {
    // Empty
}

unsafe impl Sync for Pixel {
    // Empty
}

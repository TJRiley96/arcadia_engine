#![allow(dead_code)]


pub struct Color32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

pub struct Color64 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64
}

impl Color32 {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self{
            r,
            g,
            b,
            a
        }
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }

    pub fn hex_to_color32(hex: &str) -> Option<Self> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return None;
        }
        let r = u8::from_str_radix(&hex[1..3], 16).ok()? as f32 / 255.0;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()? as f32 / 255.0;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()? as f32 / 255.0;
        Some(Self { r, g, b, a: 1.0 })
    }


    pub fn rainbow_step_color(&mut self, time: f64){
        self.r = (time.sin() / 2.0 + 0.5) as f32;
        self.g = ((time + 2.0).sin() / 2.0 + 0.5) as f32;
        self.b = ((time + 4.0).sin() / 2.0 + 0.5) as f32;
        self.a = 1.0;
    }
}

impl Color64 {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self{
            r,
            g,
            b,
            a
        }
     }
}
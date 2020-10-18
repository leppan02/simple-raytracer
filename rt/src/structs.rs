pub struct Picture {
    pub position: Point,
    pub direction: Point,
    pub size: (u32, u32),
    pub data: Vec<Vec<Color>>,
}
impl Picture {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            position: Point::new(0.0, 0.0, 0.0),
            direction: Point::new(0.0, 0.0, 1.0),
            size,
            data: vec![vec![Color::new(); size.0 as usize]; size.1 as usize],
        }
    }
}
#[derive(Debug, Clone)]
pub struct Color(pub f64, pub f64, pub f64);
impl Color {
    pub fn new() -> Self {
        Self(0.0, 0.0, 0.0)
    }
    pub fn val(&self) -> (u8, u8, u8) {
        (
            (self.0 * 255.0) as u8,
            (self.1 * 255.0) as u8,
            (self.2 * 255.0) as u8,
        )
    }
}
pub struct Surface {
    reflection: f64,
    color: Color,
}
#[derive(Debug, Clone)]
pub struct Point(pub f64, pub f64, pub f64);
impl Point {
    pub fn len(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
    pub fn normalised(&self) -> Self {
        let len = self.len();
        self.div(len)
    }
    pub fn div(&self, divisor: f64) -> Self {
        Self(self.0 / divisor, self.1 / divisor, self.2 / divisor)
    }
    pub fn mult(&self, multiplier: f64) -> Self {
        Self(
            self.0 / multiplier,
            self.1 / multiplier,
            self.2 / multiplier,
        )
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
    pub fn pivot(&self, step: f64, x: u32, y: u32, xmax: u32, ymax: u32) -> Point {
        let mut p = self.clone();
        let xdiff = step * (((xmax as f64) / 2.0) - (x as f64));
        let ydiff = step * (((ymax as f64) / 2.0) - (y as f64));
        p.0 += xdiff;
        p.1 += ydiff;
        p.normalised()
    }
}
#[derive(Debug)]
pub struct Ray {
    origin: Point,
    direction: Point,
}

impl Ray {
    pub fn new(direction: Point, origin: Point) -> Self {
        Self {
            origin,
            direction: direction.normalised(),
        }
    }
}

pub struct Circle {
    pub position: Point,
    pub radius: f64,
    pub surface: Surface,
}

impl Circle {
    pub fn new() -> Self {
        Self {
            position: Point(0.0, 0.0, 100.0),
            radius: 30.0,
            surface: Surface {
                reflection: 1.0,
                color: Color(1.0, 0.0, 0.0),
            },
        }
    }
}

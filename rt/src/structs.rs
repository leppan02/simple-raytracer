pub struct Picture {
    pub position: Point,
    pub size: (u32, u32),
    pub data: Vec<Vec<Color>>,
}

impl Picture {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            position: Point::new(0.0, 0.0, 0.0),
            size,
            data: vec![vec![Color::new(); size.0 as usize]; size.1 as usize],
        }
    }
}
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub enum IntersectResult {
    intersect {
        surface: Option<Surface>,
        point: Point,
        normal: Point,
        dist: f64,
        ray: (Point, Point),
    },
    None,
}
impl IntersectResult {
    pub fn update(self, other: Self) -> Self {
        let mut dist2: f64 = 0.0;
        match other {
            IntersectResult::None => return self,
            IntersectResult::intersect { dist, .. } => dist2 = dist,
        }
        match self {
            IntersectResult::None => other,
            IntersectResult::intersect { dist, .. } => {
                if dist2 < dist {
                    other
                } else {
                    self
                }
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Surface {
    reflection: f64,
    color: Color,
}
#[derive(Debug, Clone, Copy)]
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
            self.0 * multiplier,
            self.1 * multiplier,
            self.2 * multiplier,
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
}
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
pub enum Object {
    Circle {
        position: Point,
        radius: f64,
        surface: Option<Surface>,
    },
    Plane {
        equation: (Point, f64),
        surface: Option<Surface>,
    },
}

impl Object {
    pub fn intersect(&self, direction: &Point, origin: &Point, debug: bool) -> IntersectResult {
        match self {
            Object::Circle {
                position,
                radius,
                surface,
            } => {
                let pos = position.clone()-origin.clone();
                let dist = direction.dot(&pos);
                let fake_r = (pos.len().powi(2) - dist.powi(2)).sqrt();
                if fake_r < *radius {
                    let len = dist - ((*radius).powi(2) - fake_r.powi(2)).sqrt();
                    if len <= 1.0 {
                        IntersectResult::None
                    } else {
                        let point = direction.mult(len);
                        IntersectResult::intersect {
                            dist,
                            surface: surface.clone(),
                            point: point.clone(),
                            normal: point.clone() - position.clone(),
                            ray: (direction.clone(), origin.clone()),
                        }
                    }
                } else {
                    IntersectResult::None
                }
            }
            _ => IntersectResult::None,
        }
    }
}

pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: vec![
                Object::Circle {
                    position: Point(0.0, 30.0, 40.0),
                    radius: 10.0,
                    surface: Some(Surface {
                        reflection: 1.0,
                        color: Color(0.5, 0.5, 0.0),
                    }),
                },
                Object::Circle {
                    position: Point(20.0, 0.0, 40.0),
                    radius: 20.0,
                    surface: None,
                },
                Object::Circle {
                    position: Point(-20.0, 0.0, 40.0),
                    radius: 20.0,
                    surface: None,
                },
                Object::Circle {
                    position: Point(0.0, -10.0, 40.0),
                    radius: 20.0,
                    surface: None,
                },
            ],
        }
    }
    pub fn generate_picture(&self, picture: &mut Picture, depth: u32) -> () {
        let origin = picture.position.clone();
        let (xsize, ysize) = picture.size.clone();
        let xrange = ((xsize - 1) as f64) / 2.0;
        let yrange = ((ysize - 1) as f64) / 2.0;
        let step = 1.0 / xrange;
        for xi in 0..xsize {
            for yi in 0..ysize {
                let x = step * (xi as f64 - xrange);
                let y = step * (yrange - yi as f64);
                let ray = Point(x, y, 1.0).normalised();
                let mut res = IntersectResult::None;
                for i in &self.objects {
                    res = res.update(i.intersect(&ray, &origin, false));
                }
                picture.data[xi as usize][yi as usize] = self.bounce(res);
            }
        }
    }
    fn bounce(&self, intersect: IntersectResult) -> Color {
        match intersect {
            IntersectResult::None => Color(0.0, 0.0, 0.0),
            IntersectResult::intersect {
                surface,
                normal,
                point,
                ..
            } => match surface {
                None => Color(1.0, 1.0, 1.0),
                _ => {
                    let mut res = IntersectResult::None;
                    for i in &self.objects {
                        res = res.update(i.intersect(&normal.normalised(), &point, true)); 
                    }
                    match res {
                        IntersectResult::None=>Color(0.5,0.0,0.0),
                        IntersectResult::intersect{..}=>Color(1.0,0.0,0.0)
                    }
                }
            },
        }
    }
}

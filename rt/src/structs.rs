const BLACK: Color = Color(0.0, 0.0, 0.0);
const WHITE: Color = Color(1.0, 1.0, 1.0);
const RED: Color = Color(1.0, 0.70, 0.1);
const BLUE: Color = Color(0.1, 0.4, 0.8);
const GREEN: Color = Color(0.1, 1.0, 0.1);

pub struct Picture {
    pub position: Point,
    pub size: (u32, u32),
    pub data: Vec<Vec<Color>>,
}

impl Picture {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            position: Point::new(0.0, 0.0, -199.0),
            size,
            data: vec![vec![Color::new(); size.0 as usize]; size.1 as usize],
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Color(pub f32, pub f32, pub f32);
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
    pub fn light(&self, light: Color, dist: f32) -> Self {
        let distf = dist.powf(0.03);
        Color(
            self.0 * light.0 / distf,
            self.1 * light.1 / distf,
            self.2 * light.2 / distf,
        )
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IntersectResult {
    Intersect {
        surface: Surface,
        point: Point,
        normal: Point,
        dist: f32,
        ray: Point,
    },
    None(Point),
}
impl IntersectResult {
    pub fn update(self, other: Self) -> Self {
        match other {
            IntersectResult::None(..) => self,
            IntersectResult::Intersect { dist: odist, .. } => match self {
                IntersectResult::None(..) => other,
                IntersectResult::Intersect { dist: sdist, .. } => {
                    if odist < sdist {
                        other
                    } else {
                        self
                    }
                }
            },
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Surface {
    Bounce { diffraction: f32, color: Color },
    Light { color: Color },
}
#[derive(Debug, Clone, Copy)]
pub struct Point(pub f32, pub f32, pub f32);
impl Point {
    pub fn len(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
    pub fn normalised(&self) -> Self {
        let len = self.len();
        self.div(len)
    }
    pub fn div(&self, divisor: f32) -> Self {
        Self(self.0 / divisor, self.1 / divisor, self.2 / divisor)
    }
    pub fn mult(&self, multiplier: f32) -> Self {
        Self(
            self.0 * multiplier,
            self.1 * multiplier,
            self.2 * multiplier,
        )
    }
    pub fn dot(&self, other: &Self) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }
    pub fn mirror(&self, other: &Self) -> Self {
        *self - other.mult(2.0 * self.dot(&other))
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
        radius: f32,
        surface: Surface,
    },
    Plane {
        center: Point,
        normal: Point,
        surface: Surface,
    },
}

impl Object {
    pub fn intersect(&self, dir: &Point, origin: &Point) -> IntersectResult {
        match self {
            Object::Circle {
                position: cposition,
                surface: csurface,
                radius
            } => {
                let L = *cposition - *origin;
                let tca = L.dot(&dir);
                if tca <= 1.0 {
                    return IntersectResult::None(*dir);
                }
                let d2 = L.dot(&L) - tca * tca;
                if d2 > (*radius).powi(2) {
                    return IntersectResult::None(*dir);
                }
                let thc = ((*radius).powi(2) - d2).sqrt();
                let t = f32::min(tca - thc, tca - thc);
                if 1.0 < t {
                    let intersect = dir.mult(t) + *origin;
                    IntersectResult::Intersect {
                        dist: t,
                        surface: csurface.clone(),
                        point: intersect,
                        normal: (intersect.clone() - cposition.clone()).normalised(),
                        ray: dir.clone(),
                    }
                } else {
                    IntersectResult::None(*dir)
                }
            }
            Object::Plane {
                center,
                normal,
                surface,
            } => {
                let denom = normal.dot(dir);
                if denom.abs() > 0.0001 {
                    let difference = *center - *origin;
                    let t = difference.dot(normal) / denom;
                    if t > 1.0 {
                        return IntersectResult::Intersect {
                            surface: surface.clone(),
                            dist: t,
                            point: *origin + dir.mult(t),
                            ray: dir.clone(),
                            normal: normal.clone(),
                        };
                    }
                }
                IntersectResult::None(*dir)
            }
            _ => IntersectResult::None(*dir),
        }
    }
}

pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new(t: i32) -> Self {
        let time = (3.141592 * t as f32) / 180.0;
        let x = 35.0 * time.sin();
        let y = 35.0 * time.cos();
        Scene {
            objects: vec![
                Object::Circle {
                    position: Point(x, y, 95.0),
                    radius: 15.0,
                    surface: Surface::Bounce {
                        diffraction: 1.0,
                        color: BLUE,
                    },
                },
                Object::Circle {
                    position: Point(0.0, 0.0, 95.0),
                    radius: 12.0,
                    surface: Surface::Bounce {
                        diffraction: 1.0,
                        color: WHITE,
                    },
                },
                Object::Circle {
                    position: Point(-x, -y, 95.0),
                    radius: 15.0,
                    surface: Surface::Bounce {
                        diffraction: 1.0,
                        color: BLUE,
                    },
                },
                Object::Circle {
                    position: Point(-y, 0.0, 95.0 - x),
                    radius: 15.0,
                    surface: Surface::Bounce {
                        diffraction: 1.0,
                        color: RED,
                    },
                },
                Object::Circle {
                    position: Point(y, 0.0, 95.0 + x),
                    radius: 15.0,
                    surface: Surface::Bounce {
                        diffraction: 1.0,
                        color: RED,
                    },
                },
                Object::Plane {
                    normal: Point(0.0, 1.0, 0.0).normalised(),
                    center: Point(0.0, -175.0, 0.0),
                    surface: Surface::Bounce {
                        diffraction: 1.0,
                        color: WHITE,
                    },
                },
                Object::Plane {
                    normal: Point(0.0, -1.0, 0.0).normalised(),
                    center: Point(0.0, 175.0, 0.0),
                    surface: Surface::Light { color:WHITE },
                },
                Object::Plane {
                    normal: Point(1.0, 0.0, 1.0).normalised(),
                    center: Point(50.0, 0.0, -250.0),
                    surface: Surface::Light { color:WHITE},
                },
                Object::Plane {
                    normal: Point(-1.0, 0.0, -1.0).normalised(),
                    center: Point(0.0, 0.0, 200.0+t as f32),
                   
                    surface: Surface::Light { color:WHITE },
                },
                Object::Plane {
                    normal: Point(1.0, 0.0, -1.0).normalised(),
                    center: Point(0.0, 0.0, 200.0+t as f32),
                    surface: Surface::Bounce {
                        diffraction: 1.0,
                        color: WHITE,
                    },
                },
                Object::Plane {
                    normal: Point(-1.0, 0.0, 1.0).normalised(),
                    center: Point(50.0, 0.0, -250.0),
                    surface: Surface::Bounce {
                        diffraction: 1.0,
                        color: WHITE,
                    },
                },
            ],
        }
    }
    pub fn generate_picture(&self, picture: &mut Picture, depth: u32) -> () {
        let origin = picture.position.clone();
        let (xsize, ysize) = picture.size.clone();
        let xrange = ((xsize - 1) as f32) / 2.0;
        let yrange = ((ysize - 1) as f32) / 2.0;
        let step = 1.0 / xrange;
        for xi in 0..xsize {
            for yi in 0..ysize {
                let x = step * (xi as f32 - xrange);
                let y = step * (yrange - yi as f32);
                let ray = Point(x, y, 1.0);
                picture.data[yi as usize][xi as usize] =
                    self.bounce(self.calculate(&ray, &origin), depth);
            }
        }
    }
    fn bounce(&self, intersect: IntersectResult, depth: u32) -> Color {
        if depth == 0 {
            return BLACK;
        }
        match intersect {
            IntersectResult::None(dir) => {
                return BLACK;
                let v = f32::min(
                    1.0,
                    f32::max(dir.dot(&Point(1.0, 0.0, -1.0).normalised()), 0.0).powf(3.0) + 0.01,
                )
                .powf(0.33);
                WHITE.light(Color(v, v, v), 1.0)
            }
            IntersectResult::Intersect {
                //hit something
                surface,
                normal,
                point,
                dist: adist,
                ray,
                ..
            } => match surface {
                Surface::Light { color } => color.light(WHITE, adist), //hit light return light
                Surface::Bounce { color, .. } => {
                    //hit object
                    let bounce = ray.mirror(&normal).normalised(); //reflection
                    let res = self.bounce(self.calculate(&bounce, &point), depth - 1);
                    color.light(res, adist)
                }
            },
        }
    }
    fn calculate(&self, direction: &Point, origin: &Point) -> IntersectResult {
        let mut ret = IntersectResult::None(*direction);
        for i in &self.objects {
            ret = ret.update(i.intersect(&direction.normalised(), &origin));
        }
        ret
    }
}

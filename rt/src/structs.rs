const BLACK: Color = Color(0.0, 0.0, 0.0);
const WHITE: Color = Color(1.0, 1.0, 1.0);
const RED: Color = Color(1.0, 0.0, 0.0);
const BLUE: Color = Color(0.0, 0.0, 1.0);
const GREEN: Color = Color(0.0, 1.0, 0.0);

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
    pub fn light(&self, light: Color, dist: f64) -> Self {
        let distf =dist.powf(0.05);
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
        dist: f64,
        ray: Point,
    },
    None,
}
impl IntersectResult {
    pub fn update(self, other: Self) -> Self {
        match other {
            IntersectResult::None => self,
            IntersectResult::Intersect { dist: odist, .. } => match self {
                IntersectResult::None => other,
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
    Bounce { diffraction: f64, color: Color },
    Light { color: Color },
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
        radius: f64,
        surface: Surface,
    },
    Plane {
        equation: (Point, Point),
        surface: Surface,
    },
}

impl Object {
    pub fn intersect(&self, dir: &Point, origin: &Point, debug: bool) -> IntersectResult {
        match self {
            Object::Circle {
                position: cposition,
                radius: radius,
                surface: csurface,
            } => {
                let L = *cposition-*origin;
                let tca = L.dot(&dir);
                if tca <= 1.0{
                    return IntersectResult::None;
                }
                let d2 = L.dot(&L)-tca*tca;
                if d2 > (*radius).powi(2){
                    return IntersectResult::None;
                }
                let thc = ((*radius).powi(2)-d2).sqrt();
                let t = f64::min(tca-thc,tca-thc);
                if 1.0 < t {
                    let intersect = dir.mult(t)+*origin;
                    IntersectResult::Intersect {
                        dist: t,
                        surface: csurface.clone(),
                        point: intersect,
                        normal: (intersect.clone() - cposition.clone()).normalised(),
                        ray: dir.clone(),
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
                    position: Point(0.0, 25.0, 80.0),
                    radius: 20.0,
                    surface: Surface::Bounce {
                        diffraction: 1.0,
                        color: WHITE,
                    },
                },
                Object::Circle {
                    position: Point(20.0, -20.0, 60.0),
                    radius: 20.0,
                    surface: Surface::Bounce {
                        diffraction: 1.0,
                        color: WHITE,
                    },
                },
                Object::Circle {
                    position: Point(0.0, 0.0, 0.0),
                    radius: 40.0,
                    surface: Surface::Light { color: BLUE },
                },
            ],
        }
    }
    pub fn generate_picture(&self, picture: &mut Picture, depth: u32) -> () {
        let origin = picture.position.clone();
        let (xsize, ysize) = picture.size.clone();
        let xrange = ((xsize - 1) as f64) / 2.0;
        let yrange = ((ysize - 1) as f64) / 2.0;
        let step = 0.6 / xrange;
        for xi in 0..xsize {
            for yi in 0..ysize {
                let x = step * (xi as f64 - xrange);
                let y = step * (yrange - yi as f64);
                let ray = Point(x, y, 1.0);
                picture.data[yi as usize][xi as usize] =
                    self.bounce(self.calculate(&ray, &origin), depth);
            }
        }
    }
    fn bounce(&self, intersect: IntersectResult, depth: u32) -> Color {
        if depth == 0 {
            return WHITE.light(Color(0.5, 0.5,0.5), 1.0);
        }
        match intersect {
            IntersectResult::None => WHITE.light(Color(0.25, 0.25,0.25), 1.0), //hit nothing
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
                    let res = self.bounce(self.calculate(&bounce, &point), depth-1);
                    color.light(res, adist)
                }
            },
        }
    }
    fn calculate(&self, direction: &Point, origin: &Point) -> IntersectResult {
        let mut ret = IntersectResult::None;
        for i in &self.objects {
            ret = ret.update(i.intersect(&direction.normalised(), &origin, false));
        }
        ret
    }
}

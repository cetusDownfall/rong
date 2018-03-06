use piston_window::{Context, G2d, rectangle};
#[derive(Copy, Clone)]
pub struct Vector(f64, f64);
#[derive(Copy, Clone)]
pub struct Size(Vector);
#[derive(Copy, Clone)]
pub struct Position(Vector);
#[derive(Copy, Clone)]
pub struct Velocity(Vector);
impl Vector {
    pub fn new(x: f64, y: f64) -> Vector { Vector(x,y) }
    pub fn x(&self) -> f64 { self.0 }
    pub fn y(&self) -> f64 { self.1 }
    pub fn comps(&self) -> (f64, f64) { (self.x(), self.y()) }
    pub fn set_x(&mut self, val: f64) { self.0 = val; }
    pub fn set_y(&mut self, val: f64) { self.1 = val; }
    pub fn set(&mut self, x_val: f64, y_val: f64) { self.0 = x_val; self.1 = y_val; }
    pub fn add_x(&self, val: f64) -> f64 { self.0 + val }
    pub fn add_y(&self, val: f64) -> f64 { self.1 + val }
    pub fn add(&self, v: Vector) -> Vector { (self.0 + v.x(), self.1 + v.y()).into() }
    pub fn add_x_mut(&mut self, val: f64) { self.0 = self.0 + val; }
    pub fn add_y_mut(&mut self, val: f64) { self.1 = self.1 + val; }
    pub fn add_mut(&mut self, v: Vector) { self.0 = self.0 + v.0; self.1 = self.1 + v.1; }
    pub fn scale(&self, factor: f64) -> Vector { (self.0 * factor, self.1 * factor).into() }
}
impl From<(f64, f64)> for Vector { fn from(xy: (f64, f64)) -> Vector { Vector(xy.0, xy.1) } }
impl<T: Into<Vector>> From<T> for Size { fn from(v: T) -> Size { Size(v.into()) } }
impl<T: Into<Vector>> From<T> for Position { fn from(v: T) -> Position { Position(v.into()) } }
impl<T: Into<Vector>> From<T> for Velocity { fn from(v: T) -> Velocity { Velocity(v.into()) } }
impl Size {
    pub fn v(&self) -> Vector { self.0 }
}

impl Position {
    pub fn v(&self) -> Vector { self.0 }
    pub fn dp(&mut self, dt: f64, vel: &Velocity) {
        self.0.add_mut(vel.v().scale(dt));
    }
}

impl Velocity {
    pub fn v(&self) -> Vector { self.0 }
    pub fn set_v(&mut self, v: Vector) { self.0 = v }
}

pub struct AABB {
    pos: Position,
    dims: Size,
}

impl AABB {
    pub fn new(pos: Position, dims: Size) -> AABB {
        AABB { pos, dims }
    }
    pub fn update(&mut self, dt: f64, vel: Velocity) {
        self.pos.dp(dt, &vel);
    }
    pub fn left(&self) -> f64 { self.pos.v().x() }
    pub fn top(&self) -> f64 { self.pos.v().y() }
    pub fn right(&self) -> f64 { self.pos.v().add_x(self.dims.v().x()) }
    pub fn bottom(&self) -> f64 { self.pos.v().add_y(self.dims.v().y()) }
    fn tl(&self) -> Position { self.pos.v().into() }
    fn br(&self) -> Position { self.pos.v().add(self.dims.v()).into() }
    fn bl(&self) -> Position { (self.left(), self.bottom()).into() }
    fn tr(&self) -> Position { (self.right(), self.top()).into() }
    fn pts(&self) -> Vec<Position> { vec![self.tl(), self.tr(), self.br(), self.bl()] }
    pub fn size(&self) -> Size { self.dims }
    fn contains(&self, pt: Position) -> bool {
        self.left() <= pt.v().x() && self.right() >= pt.v().x()
     && self.top() <= pt.v().y() && self.bottom() >= pt.v().y()
    }
    pub fn colliding(&self, other: &AABB) -> bool {
        let mut pts = self.pts().into_iter();
        let mut out = false;
        while let Some(pt) = pts.next() {
            if out {
                break;
            }
            if other.contains(pt) {
                out = true;
            }
        }
        if !out {
            pts = other.pts().into_iter();
            while let Some(pt) = pts.next() {
                if out {
                    break;
                }
                if self.contains(pt) {
                    out = true;
                }
            }
        }
        out
    }
    pub fn draw(&self, c: Context, g: &mut G2d) {
        rectangle([1.; 4], [self.left(), self.top(), self.dims.v().x(), self.dims.v().y()], c.transform, g);
    }
}

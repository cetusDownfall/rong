use fauxsics::{Vector, AABB, Velocity, Position, Size};
use control::Control;
use piston_window::keyboard::Key;
use piston_window::{Context, G2d, ellipse};

pub struct Player {
    pdl: Paddle,
    goal: Goal,
    score: i32,
}

impl Player {
    pub fn new(ku: Key, kd: Key, pos: Position, goal_x: f64, dims: Size, speed: f64) -> Player {
        Player {
            pdl: Paddle::new(ku, kd, pos, dims, speed),
            goal: Goal::new(goal_x, dims.v().y()),
            score: 0,
        }
    }
    pub fn update(&mut self, dt: f64, ball: &mut Ball) {
        self.pdl.update(dt, ball);
        self.goal.update(&ball);
        self.score = self.goal.score();
    }
    pub fn draw(&self, c: Context, g: &mut G2d) {
        self.pdl.draw(c,g);
    }
    pub fn press(&mut self, k: Key) {
        self.pdl.press(k);
    }
    pub fn release(&mut self, k: Key) {
        self.pdl.release(k);
    }
}

pub struct Paddle {
    up: Control,
    down: Control,
    hb: AABB,
    vel: Velocity,
    speed: f64,
}

impl Paddle {
    pub fn new(ku: Key, kd: Key, pos: Position, size: Size, speed: f64) -> Paddle {
        Paddle {
            up: Control::new(ku),
            down: Control::new(kd),
            hb: AABB::new(pos, size),
            vel: (0., 0.).into(),
            speed,
        }
    }
    pub fn update(&mut self, dt: f64, ball: &mut Ball) {
        let up = if self.up.get() { -self.speed } else { 0. };
        let down = if self.down.get() { self.speed } else { 0. };
        let net = up + down;
        self.vel.set_v((0., net).into());
        if self.hb.colliding(ball.hb()) {
            ball.vel = (-1.05 * ball.vel.v().x(), 
                        ball.vel.v().y() + self.vel.v().y() * 0.3).into();
            self.speed = self.speed * 1.03;
        }
        self.hb.update(dt, self.vel);
    }
    pub fn draw(&self, c: Context, g: &mut G2d) {
        self.hb.draw(c,g);
    }
    pub fn press(&mut self, k: Key) {
        if self.up.key() == k {
            self.up.set(true);
        }
        if self.down.key() == k {
            self.down.set(true);
        }
    }
    pub fn release(&mut self, k: Key) {
        if self.up.key() == k {
            self.up.set(false);
        }
        if self.down.key() == k {
            self.down.set(false);
        }
    }
}

pub struct Goal {
    hb: AABB,
    score: i32,
}

impl Goal {
    pub fn new(x: f64, height: f64) -> Goal {
        Goal {
            hb: AABB::new((x, 0.).into(), (1., height).into()),
            score: 0,
        }
    }
    pub fn hb(&self) -> &AABB {
        &self.hb
    }
    pub fn score(&self) -> i32 {
        self.score
    }
    pub fn update(&mut self, ball: &Ball) {
        if self.hb().colliding(ball.hb()) {
            self.score = self.score + 1;
        }
    }
}

pub struct Wall(AABB);

impl Wall {
    pub fn new(y: f64, width: f64) -> Wall {
        Wall(AABB::new((0., y-5.).into(), (width, 10.).into()))
    }
    pub fn hb(&self) -> &AABB {
        &self.0
    }
    pub fn update(&self, ball: &mut Ball) {
        if self.hb().colliding(ball.hb()) {
            ball.vel = (ball.vel.v().x(),  -1.05 * ball.vel.v().y()).into();
        }
    }
}

pub struct Ball {
    hb: AABB,
    pub vel: Velocity,
}

impl Ball {
    pub fn new(pos: Position, size: Size, vel: Velocity) -> Ball {
        Ball {
            hb: AABB::new(pos, size),
            vel,
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.hb.update(dt, self.vel);
    }
    pub fn hb(&self) -> &AABB {
        &self.hb
    }
    pub fn draw(&self, c: Context, g: &mut G2d) {
        ellipse([1.;4], [self.hb.left(), self.hb.top(), 
                self.hb.size().v().x(), self.hb.size().v().y()], c.transform, g);
    }
}

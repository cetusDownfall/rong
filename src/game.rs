extern crate piston_window;
use piston_window::*;
use object::{Ball, Wall, Player};
pub struct Game {
    p1: Player,
    p2: Player,
    ball: Ball,
    walls: Vec<Wall>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            p1: Player::new(Key::Up, Key::Down, 
                            (579.0f64, 225.0f64).into(), 0.0f64, 
                            (20.0f64, 80.0f64).into(), 380.),
            p2: Player::new(Key::W, Key::S, 
                            (49.0f64, 225.0f64).into(), 640.0f64, 
                            (20.0f64, 80.0f64).into(), 380.),
            ball: Ball::new((315.0f64, 235.0f64).into(), 
                            (5.0f64, 5.0f64).into(), 
                            (640.0f64, 150.0f64).into()),
            walls: vec![Wall::new(0., 640.), Wall::new(480., 640.)],
        }
    }
    pub fn start(&mut self) {
        let mut win: PistonWindow = WindowSettings::new("Rong", [640, 480]).build().unwrap();
        self.run(&mut win);
    }
    pub fn run(&mut self, win: &mut PistonWindow) {
        while let Some(e) = win.next() {
            if let Some(args) = e.render_args() {
                win.draw_2d(&e, |c, g| {
                    rectangle([0.,0.,0.,1.], [0., 0., 640., 480.], c.transform, g);
                    self.p1.draw(c,g);
                    self.p2.draw(c,g);
                    self.ball.draw(c,g);
                });
            }
            if let Some(args) = e.update_args() {
                self.p1.update(args.dt, &mut self.ball);
                self.p2.update(args.dt, &mut self.ball);
                for w in self.walls.iter() {
                    w.update(&mut self.ball);
                }
                self.ball.update(args.dt);
                /*
                if self.p1.won() {
                    println!("P1 won");
                    break;
                }
                if self.p2.won() {
                    println!("P2 won");
                    break;
                }
                */
            }
            if let Some(Button::Keyboard(k)) = e.press_args() {
                self.p1.press(k);
                self.p2.press(k);
            }
            if let Some(Button::Keyboard(k)) = e.release_args() {
                self.p1.release(k);
                self.p2.release(k);
            }
        }
    }
}

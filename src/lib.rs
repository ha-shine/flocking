extern crate rand;

const BOID_SIZE: f64 = 7.0;
const BOID_RADIUS: f64 = BOID_SIZE / 2.0;
const MAX_SPEED: f64 = BOID_SIZE * 4.0;

const COHESION_RADIUS: f64 = BOID_SIZE * 8.0;
const COHESION_WEIGHT: f64 = 0.2;

const ALIGNMENT_RADIUS: f64 = BOID_SIZE * 8.0;
const ALIGNMENT_WEIGHT: f64 = 1.0;

const SEPARATION_RADIUS: f64 = BOID_SIZE * 5.0;
const SEPARATION_WEIGHT: f64 = 1.0;


struct Boid {
    // position
    x: f64,
    y: f64,

    // velocity
    vx: f64,
    vy: f64,
}

impl Boid {
    // returns a boid within given rectangle
    fn new(width: i32, height: i32) -> Boid {
        Boid {
            x: rand::random::<f64>() * width as f64,
            y: rand::random::<f64>() * height as f64,

            vx: rand::random::<f64>() * BOID_SIZE,
            vy: rand::random::<f64>() * BOID_SIZE
        }
    }

    // flock based on it's neighbors
    // returns the x and y components of resultant vector
    fn flock(&self, boids: &[Boid]) -> (f64, f64) {
        let alignment = self.align(&boids);
        let cohesion = self.cohere(&boids);
        let separation = self.separate(&boids);

        let mut vx = self.vx + alignment.0 * ALIGNMENT_WEIGHT + cohesion.0 * COHESION_WEIGHT + separation.0 * SEPARATION_WEIGHT;
        let mut vy = self.vy + alignment.1 * ALIGNMENT_WEIGHT + cohesion.1 * COHESION_WEIGHT + separation.1 * SEPARATION_WEIGHT;

        vx = self.limit_speed(vx);
        vy = self.limit_speed(vy);

        (vx, vy)
    }

    // make sure the velocity doesn't go too fast
    fn limit_speed(&self, v: f64) -> f64 {
        if v > MAX_SPEED {
            MAX_SPEED
        } else if v < -MAX_SPEED {
            -MAX_SPEED
        } else {
            v
        }
    }

    fn align(&self, boids: &[Boid]) -> (f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut count = 0.0;

        for b in boids.iter() {
            let distance = self.distance_to(b);
            if distance > 0.0 && distance < ALIGNMENT_RADIUS {
                x += b.vx;
                y += b.vy;
                count += 1.0;
            }
        }

        if count > 0.0 {
            x /= count;
            y /= count;
        }

        (x, y)
    }

    fn cohere(&self, boids: &[Boid]) -> (f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut count = 0.0;

        for b in boids.iter() {
            let distance = self.distance_to(b);
            if distance > 0.0 && distance < COHESION_RADIUS {
                x += b.x;
                y += b.y;
                count += 1.0;
            }
        }

        if count > 0.0 {
            x /= count;
            y /= count;

            x -= self.x;
            y -= self.y;
        }

        (x,y)
    }

    fn separate(&self, boids: &[Boid]) -> (f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;

        for b in boids.iter() {
            let distance = self.distance_to(b);
            if distance > 0.0 && distance < SEPARATION_RADIUS {
                x -= b.x - self.x;
                y -= b.y - self.y;
            }
        }

        (x, y)
    }

    // returns a distance between two boids
    fn distance_to(&self, b: &Boid) -> f64 {
        let x = self.x - b.x;
        let y = self.y - b.y;

        (x*x + y*y).sqrt()
    }
}

// rendering goes here
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlGraphics};
use piston::input::*;

pub struct App {
    gl: GlGraphics,     // OpenGL drawing backend.
    boids: Vec<Boid>,
    width: u32,
    height: u32,
    ups: u64, // update per seconds
}

impl App {
    pub fn new(gl: GlGraphics, width: u32, height: u32, count: u32, ups: u64) -> App {
        let mut app = App {
            gl,
            width,
            height,
            ups,
            boids: Vec::new()
        };

        for _ in 0..count {
            app.boids.push(Boid::new(width as i32, height as i32));
        }

        app
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, BOID_SIZE as f64);

        let positions: Vec<(f64, f64)> = self.boids.iter().map(|boid| (boid.x, boid.y)).collect();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            for pos in positions.iter() {
                let transform = c.transform.trans(pos.0 - BOID_RADIUS, pos.1 - BOID_RADIUS);

                ellipse(RED, square, transform, gl);
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        for i in 0..self.boids.len() {
            let (x, y, vx, vy) = {
                let boid = &self.boids[i];
                let (vx, vy) = boid.flock(&self.boids);

                let x = boid.x + (vx / self.ups as f64);
                let y = boid.y + (vy / self.ups as f64);

                let (x, y) = self.stay_in_view(x, y);

                (x, y, vx, vy)
            };

            self.boids[i].x = x;
            self.boids[i].y = y;
            self.boids[i].vx = vx;
            self.boids[i].vy = vy;
        }
    }

    // make sure the boid stays inside the current view frame
    fn stay_in_view(&self, x: f64, y: f64) -> (f64, f64) {
        let mut x = x;
        let mut y = y;

        if x > self.width as f64 {
            x -= self.width as f64;
        } else if x < 0.0 {
            x += self.width as f64;
        }

        if y > self.height as f64 {
            y -= self.height as f64;
        } else if y < 0.0 {
            y += self.height as f64;
        }

        (x, y)
    }
}
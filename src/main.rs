extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player1: Cat,
    player2: Cat
}

pub struct Cattery {
    Chester: Cat,
    Gigabyte: Cat,
    Frank: Cat
}

struct Cat {
    pub color: [f32; 4], // R G B brightness?
    pub position: [f64; 4], // x, y, width, height
    pub stats: [f64; 5], // attack, spd, def, current health, total health
    pub movement: [bool; 4], // left, right, crouch, jump
    pub stance: [bool; 4] // stand, attack, defend, injured
}

impl Cat {
    fn new(color: [f32; 4], position: [f64; 4], stats: [f64; 5]) -> Self {
        Cat {
            color: color,
            position: position,
            stats: stats,
            movement: [false, false, false, false],
            stance: [true, false, false, false]
        }
    }

    fn clone(&mut self) -> Cat {
        return Cat::new(self.color, self.position, self.stats);
    }

    fn move_cat(&mut self) {
        // left
        if self.movement[0] && self.position[0] >= 0.0 {
            self.stance[0] = false;
            self.position[0] -= self.stats[1];
        }

        //right
        if self.movement[1] && self.position[0] <= (400.0 - self.position[2]) {
            self.stance[0] = false;
            self.position[0] += self.stats[1];
        }

        // jump rise
        if self.movement[3] {
            if self.position[1] > 25.0 {
                self.position[1] -= self.stats[1];
            } else {
                self.movement[3] = false;
            }
        }
        // jump fall
        if !self.movement[3] && self.position[1] < 75.0 {
            self.position[1] += self.stats[1];
        }

        // crouch
        if self.movement[2] && self.position[3] >= 25.0 {
            self.position[3] -= self.stats[1];
            self.position[1] += self.stats[1];
        }
        if !self.movement[2] && self.position[3] < 50.0 {
            self.position[3] += self.stats[1];
            self.position[1] -= self.stats[1];
        }
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        let square1: Cat = self.player1.clone();
        let square2: Cat = self.player2.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            // Draw a box rotating around the middle of the screen.
            rectangle(square1.color, square1.position, c.transform, gl);
            rectangle(square2.color, square2.position, c.transform, gl);
        });
    }

    fn input(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(Key::Right) => {
                self.player2.movement[1] = true;
            }
            Button::Keyboard(Key::Left) => {
                self.player2.movement[0] = true;
            }
            Button::Keyboard(Key::Down) => {
                self.player2.movement[2] = true;
            }
            Button::Keyboard(Key::Up) => {
                self.player2.movement[3] = true;
            }
            Button::Keyboard(Key::D) => {
                self.player1.movement[1] = true;
            }
            Button::Keyboard(Key::A) => {
                self.player1.movement[0] = true;
            }
            Button::Keyboard(Key::S) => {
                self.player1.movement[2] = true;
            }
            Button::Keyboard(Key::W) => {
                self.player1.movement[3] = true;
            }
            _ => {}
        }
    }

    fn release(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(Key::Right) => {
                self.player2.movement[1] = false;
            }
            Button::Keyboard(Key::Left) => {
                self.player2.movement[0] = false;
            }
            Button::Keyboard(Key::Down) => {
                self.player2.movement[2] = false;
            }
            Button::Keyboard(Key::Up) => {
                self.player2.movement[3] = false;
            }
            Button::Keyboard(Key::D) => {
                self.player1.movement[1] = false;
            }
            Button::Keyboard(Key::A) => {
                self.player1.movement[0] = false;
            }
            Button::Keyboard(Key::S) => {
                self.player1.movement[2] = false;
            }
            Button::Keyboard(Key::W) => {
                self.player1.movement[3] = false;
            }
            _ => {}
        }
    }

    fn update(&mut self) {
        self.player1.move_cat();
        self.player2.move_cat();
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Mortal Tomcat",
            [400, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player1: Cat::new([0.57, 0.40, 0.18, 1.0], [20.0, 75.0, 50.0, 50.0], [2.0, 0.5, 1.0, 10.0, 10.0]),
        player2: Cat::new([0.12, 0.12, 0.12, 1.0], [130.0, 75.0, 50.0, 50.0], [1.0, 2.0, 0.5, 10.0, 10.0])
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(b) = e.button_args() {
            match b.state {
                ButtonState::Press => {
                    app.input(&b);
                }
                ButtonState::Release => {
                    app.release(&b);
                }
            }
        }

        if let Some(_u) = e.update_args() {
            app.update();
        }
    }
}
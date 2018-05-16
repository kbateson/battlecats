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

struct Cat {
    pub color: [f32; 4],
    pub position: [f64; 4],
    pub stats: [f64; 5], // attack, spd, def, current health, total health
}

impl Cat {
    fn new(color: [f32; 4], position: [f64; 4]) -> Self {
        Cat {
            color: color,
            position: position,
            stats: [1.0, 1.0, 1.0, 10.0, 10.0]
        }
    }

    fn clone(&mut self) -> Cat {
        return Cat::new(self.color, self.position);
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

    fn move_square(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(Key::Right) => {
                self.player2.position[0] += 2.0;
            }
            Button::Keyboard(Key::Left) => {
                self.player2.position[0] -= 2.0;
            }
            Button::Keyboard(Key::Down) => {
                self.player2.position[1] += 2.0;
            }
            Button::Keyboard(Key::Up) => {
                self.player2.position[1] -= 2.0;
            }
            Button::Keyboard(Key::D) => {
                self.player1.position[0] += 2.0;
            }
            Button::Keyboard(Key::A) => {
                self.player1.position[0] -= 2.0;
            }
            Button::Keyboard(Key::S) => {
                self.player1.position[1] += 2.0;
            }
            Button::Keyboard(Key::W) => {
                self.player1.position[1] -= 2.0;
            }
            _ => {}
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Mortal Tomcat",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player1: Cat::new([1.0, 0.0, 0.0, 1.0], [20.0, 75.0, 50.0, 50.0]),
        player2: Cat::new([0.0, 0.0, 1.0, 1.0], [130.0, 75.0, 50.0, 50.0])
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(b) = e.button_args() {
            app.move_square(&b);
        }
    }
}
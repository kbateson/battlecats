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
    player1: ColoredRect,
    player2: ColoredRect
}

struct ColoredRect {
    pub color: [f32; 4],
    pub position: [f64; 4],
    velocity: [f64; 2]
}

impl ColoredRect {
    fn new(color: [f32; 4], position: [f64; 4]) -> Self {
        ColoredRect {
            color: color,
            position: position,
            velocity: [0.3, 0.3]
        }
    }

    fn update_color(color: f32)->f32 {
        if color <= 0.0 {
            1.0
        } else {
            color - 0.001
        }
    }

    fn update(&mut self, size: (f64, f64)) {
        self.color[0] = Self::update_color(self.color[0]);
        self.color[1] = Self::update_color(self.color[1]);
        self.color[2] = Self::update_color(self.color[2]);

        // update X
        if self.position[0] + self.position[2] >= size.0||
           self.position[0] < 0.0 {
            self.velocity[0] = -self.velocity[0];
        }
        self.position[0] += self.velocity[0];

        // update Y
        if self.position[1] + self.position[3] >= size.1||
           self.position[1] < 0.0 {
            self.velocity[1] = -self.velocity[1];
        }
        self.position[1] += self.velocity[1];
    }

    fn clone(&mut self) -> ColoredRect {
        return ColoredRect::new(self.color, self.position);
    }
    
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        let square1: ColoredRect = self.player1.clone();
        let square2: ColoredRect = self.player2.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            // Draw a box rotating around the middle of the screen.
            rectangle(square1.color, square1.position, c.transform, gl);
            rectangle(square2.color, square2.position, c.transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
    }

    fn move_square(&mut self, args: &ButtonArgs) {
        println!("Scancode {:?}", args.scancode);
        match args.scancode {
            Some(124) => {
                self.player2.position[0] += 2.0;
            }
            Some(123) => {
                self.player2.position[0] -= 2.0;
            }
            Some(125) => {
                self.player2.position[1] += 2.0;
            }
            Some(126) => {
                self.player2.position[1] -= 2.0;
            }
            Some(2) => {
                self.player1.position[0] += 2.0;
            }
            Some(0) => {
                self.player1.position[0] -= 2.0;
            }
            Some(1) => {
                self.player1.position[1] += 2.0;
            }
            Some(13) => {
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
            "spinning-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player1: ColoredRect::new([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 50.0, 50.0]),
        player2: ColoredRect::new([0.0, 0.0, 1.0, 1.0], [50.0, 50.0, 50.0, 50.0])
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(b) = e.button_args() {
            app.move_square(&b);
        }
    }
}
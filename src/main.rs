extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate sprite;
extern crate find_folder;

mod cat;
mod hud;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture, TextureSettings };
use cat::LeftCat;
use cat::RightCat;
use cat::Cat;
use hud::HUD;
use std::path::Path;
use graphics::rectangle::square;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player1: LeftCat,
    player2: RightCat,
    hud: HUD
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let left_alive: bool = self.player1.check_alive();
        let right_alive: bool = self.player2.check_alive();
        if left_alive && right_alive {
            const GREEN: [f32; 4] = [0.07, 0.3, 0.12, 1.0];
            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
            const LIME: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
            const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

            let square1: LeftCat = self.player1.clone();
            let square2: RightCat = self.player2.clone();
            let outer_health_left = self.hud.health_bar_left.outer;
            let inner_health_left = self.hud.health_bar_left.inner;
            let damage_health_left = self.hud.health_bar_left.damage;
            let outer_health_right = self.hud.health_bar_right.outer;
            let inner_health_right = self.hud.health_bar_right.inner;
            let damage_health_right = self.hud.health_bar_right.damage;
            let chester: Texture = Texture::from_path(Path::new("/Users/Kristen/Desktop/CS Stuff/Rust/Battlecats/battlecats/src/imgs/chester.gif"), &TextureSettings::new()).unwrap();
            let chester_paw: Texture = Texture::from_path(Path::new("/Users/Kristen/Desktop/CS Stuff/Rust/Battlecats/battlecats/src/imgs/chesterpaw.gif"), &TextureSettings::new()).unwrap();
            let gigabyte: Texture = Texture::from_path(Path::new("/Users/Kristen/Desktop/CS Stuff/Rust/Battlecats/battlecats/src/imgs/gigabyte.gif"), &TextureSettings::new()).unwrap();
            let gigabyte_paw: Texture = Texture::from_path(Path::new("/Users/Kristen/Desktop/CS Stuff/Rust/Battlecats/battlecats/src/imgs/gigabytepaw.gif"), &TextureSettings::new()).unwrap();

            self.gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                clear(GREEN, gl);
                let chester_box = Image::new().rect(square(square1.position[0], square1.position[1], 100.0));
                let chester_box_paw = Image::new().rect([square1.position[0]+45.0, square1.position[1]+72.0, 70.0, 20.0]);
                let gigs_box = Image::new().rect(square(square2.position[0], square2.position[1], 100.0));
                let gigs_box_paw = Image::new().rect([square2.position[0]-15.0, square2.position[1]+72.0, 70.0, 20.0]);
                
                // bg_box.draw(&bg, &DrawState::default(), c.transform, gl);
                gigs_box_paw.draw(&gigabyte_paw, &DrawState::default(), c.transform, gl);
                gigs_box.draw(&gigabyte, &DrawState::default(), c.transform, gl);
                chester_box_paw.draw(&chester_paw, &DrawState::default(), c.transform, gl);
                chester_box.draw(&chester, &DrawState::default(), c.transform, gl);
                rectangle(WHITE, outer_health_left, c.transform, gl);
                rectangle(LIME, inner_health_left, c.transform, gl);
                rectangle(RED, damage_health_left, c.transform, gl);
                rectangle(WHITE, outer_health_right, c.transform, gl);
                rectangle(LIME, inner_health_right, c.transform, gl);
                rectangle(RED, damage_health_right, c.transform, gl);
            });
        } else {
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            let winner: Texture;

            if left_alive {
                winner = Texture::from_path(Path::new("/Users/Kristen/Desktop/CS Stuff/Rust/Battlecats/battlecats/src/imgs/lwin.png"), &TextureSettings::new()).unwrap();
            } else {
                winner = Texture::from_path(Path::new("/Users/Kristen/Desktop/CS Stuff/Rust/Battlecats/battlecats/src/imgs/rwin.png"), &TextureSettings::new()).unwrap();
            }

            let winner_box = Image::new().rect([0.0, 25.0, 500.0, 150.0]);
            self.gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                clear(BLACK, gl);
                winner_box.draw(&winner, &DrawState::default(), c.transform, gl)
            });
        }
    }

    fn input(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(Key::Return) => {
                let health = 100.0 - (10.0 * self.player2.attack(self.player1.position[0] + self.player1.position[2]));
                let damage = 100.0 - (10.0 * self.player1.attacked(self.player2.stance[1], self.player2.stats[0]));
                self.hud.update(damage, health);
            }
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
            Button::Keyboard(Key::Space) => {
                let health = 100.0 - (10.0 * self.player1.attack(self.player2.position[0]));
                let damage = 100.0 - (10.0 * self.player2.attacked(self.player1.stance[1], self.player1.stats[0]));
                self.hud.update(health, damage);
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
            Button::Keyboard(Key::Return) => {
                self.player2.stance[1] = false;
            }
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
            Button::Keyboard(Key::Space) => {
                self.player1.stance[1] = false;
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
        self.player1.move_cat(self.player2.position[0]);
        self.player2.move_cat(self.player1.position[0] + self.player1.position[2]);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Mortal Tomcat",
            [500, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player1: LeftCat::new([1.0, 0.0, 0.0, 1.0], [20.0, 75.0, 50.0, 50.0], [2.0, 0.5, 1.0, 10.0, 10.0]),
        player2: RightCat::new([0.0, 0.0, 1.0, 1.0], [410.0, 75.0, 50.0, 50.0], [1.0, 2.0, 0.5, 10.0, 10.0]),
        hud: HUD::new()
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
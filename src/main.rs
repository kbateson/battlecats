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
use std::fs;
use std::path::PathBuf;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player1: LeftCat,
    player2: RightCat,
    hud: HUD,
    reset: bool
}

impl App {
    fn reset(&mut self) {
        self.player1 = LeftCat::new([1.0, 0.0, 0.0, 1.0], [105.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        self.player2 = RightCat::new([0.0, 0.0, 1.0, 1.0], [595.0, 200.0, 50.0, 50.0], [1.0, 2.0, 0.75, 10.0, 10.0], [true, false, false, false, false]);
        self.hud = HUD::new();
    }

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
            let chester_relative = PathBuf::from("./src/imgs/chester.gif");
            let chester_absolute = fs::canonicalize(&chester_relative);
            let chester_paw_relative = PathBuf::from("./src/imgs/chesterpaw.gif");
            let chester_paw_absolute = fs::canonicalize(&chester_paw_relative);
            let gigabyte_relative = PathBuf::from("./src/imgs/gigabyte.gif");
            let gigabyte_absolute = fs::canonicalize(&gigabyte_relative);
            let gigabyte_paw_relative = PathBuf::from("./src/imgs/gigabytepaw.gif");
            let gigabyte_paw_absolute = fs::canonicalize(&gigabyte_paw_relative);
            let chester: Texture = Texture::from_path(Path::new(&chester_absolute.unwrap()), &TextureSettings::new()).unwrap();
            let chester_paw: Texture = Texture::from_path(Path::new(&chester_paw_absolute.unwrap()), &TextureSettings::new()).unwrap();
            let gigabyte: Texture = Texture::from_path(Path::new(&gigabyte_absolute.unwrap()), &TextureSettings::new()).unwrap();
            let gigabyte_paw: Texture = Texture::from_path(Path::new(&gigabyte_paw_absolute.unwrap()), &TextureSettings::new()).unwrap();
            
            self.gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                clear(GREEN, gl);
                let chester_box = Image::new().rect(square(square1.position[0], square1.position[1], 100.0));
                let chester_box_paw = Image::new().rect([square1.position[0]+45.0, square1.position[1]+72.0, 70.0, 20.0]);
                let gigs_box = Image::new().rect(square(square2.position[0], square2.position[1], 100.0));
                let gigs_box_paw = Image::new().rect([square2.position[0]-15.0, square2.position[1]+72.0, 70.0, 20.0]);
                let rhiss_relative = PathBuf::from("./src/imgs/rhiss.png");
                let rhiss_absolute = fs::canonicalize(&rhiss_relative);
                let lhiss_relative = PathBuf::from("./src/imgs/lhiss.png");
                let lhiss_absolute = fs::canonicalize(&lhiss_relative);
                let lhiss: Texture = Texture::from_path(Path::new(&lhiss_absolute.unwrap()), &TextureSettings::new()).unwrap();
                let rhiss: Texture = Texture::from_path(Path::new(&rhiss_absolute.unwrap()), &TextureSettings::new()).unwrap();
                gigs_box_paw.draw(&gigabyte_paw, &DrawState::default(), c.transform, gl);
                chester_box_paw.draw(&chester_paw, &DrawState::default(), c.transform, gl);
                 
                // bg_box.draw(&bg, &DrawState::default(), c.transform, gl);
                if square1.stance[2] {
                    let chester_hiss_relative = PathBuf::from("./src/imgs/chesterhiss.png");
                    let chester_hiss_absolute = fs::canonicalize(&chester_hiss_relative);
                    let chester_hiss: Texture = Texture::from_path(Path::new(&chester_hiss_absolute.unwrap()), &TextureSettings::new()).unwrap();
                    let lhiss_box = Image::new().rect(square(square1.position[0]+100.0, square1.position[1], 100.0));
                    chester_box.draw(&chester_hiss, &DrawState::default(), c.transform, gl);
                    lhiss_box.draw(&lhiss, &DrawState::default(), c.transform, gl);
                } else {
                    chester_box.draw(&chester, &DrawState::default(), c.transform, gl);
                }

                if square2.stance[2] {
                    println!("gigs hiss!");
                    let gigabyte_hiss_relative = PathBuf::from("./src/imgs/gigabytehiss.png");
                    let gigabyte_hiss_absolute = fs::canonicalize(&gigabyte_hiss_relative);
                    let gigabyte_hiss: Texture = Texture::from_path(Path::new(&gigabyte_hiss_absolute.unwrap()), &TextureSettings::new()).unwrap();
                    let rhiss_box = Image::new().rect(square(square2.position[0]-100.0, square2.position[1], 100.0));  
                
                    gigs_box.draw(&gigabyte_hiss, &DrawState::default(), c.transform, gl);
                    rhiss_box.draw(&rhiss, &DrawState::default(), c.transform, gl);
                } else {
                    gigs_box.draw(&gigabyte, &DrawState::default(), c.transform, gl);
                }

                rectangle(WHITE, outer_health_left, c.transform, gl);
                rectangle(LIME, inner_health_left, c.transform, gl);
                rectangle(RED, damage_health_left, c.transform, gl);
                rectangle(WHITE, outer_health_right, c.transform, gl);
                rectangle(LIME, inner_health_right, c.transform, gl);
                rectangle(RED, damage_health_right, c.transform, gl);
            });
        } else {
            if self.reset {
                self.reset();
            }
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            let winner: Texture;
            let lwin_relative = PathBuf::from("./src/imgs/lwin.png");
            let lwin_absolute = fs::canonicalize(&lwin_relative);
            let rwin_relative = PathBuf::from("./src/imgs/rwin.png");
            let rwin_absolute = fs::canonicalize(&rwin_relative);

            if left_alive {
                winner = Texture::from_path(Path::new(&lwin_absolute.unwrap()), &TextureSettings::new()).unwrap();
            } else {
                winner = Texture::from_path(Path::new(&rwin_absolute.unwrap()), &TextureSettings::new()).unwrap();
            }

            let winner_box = Image::new().rect([150.0, 125.0, 500.0, 150.0]);
            self.gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                clear(BLACK, gl);
                winner_box.draw(&winner, &DrawState::default(), c.transform, gl)
            });
        }
    }

    fn input(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(Key::O) => {
                self.player2.hiss();
                self.player1.hissed(self.player2.position[2], self.player2.position[1], self.player2.stance[2]);
            }
            Button::Keyboard(Key::E) =>{
                self.player1.hiss();
                self.player2.hissed(self.player1.position[0] + self.player1.position[2], self.player1.position[1], self.player1.stance[2]);
            }
            Button::Keyboard(Key::U) => {
                let damage = 100.0 - (10.0 * self.player2.attack(self.player1.position[0] + self.player1.position[2], self.player1.position[1]));
                let health = 100.0 - (10.0 * self.player1.attacked(self.player2.stance[1], self.player2.stats[0]));
                self.hud.update(health, damage);
            }
            Button::Keyboard(Key::L) => {
                self.player2.movement[1] = true;
            }
            Button::Keyboard(Key::J) => {
                self.player2.movement[0] = true;
            }
            Button::Keyboard(Key::K) => {
                self.player2.movement[2] = true;
            }
            Button::Keyboard(Key::I) => {
                self.player2.movement[3] = true;
            }
            Button::Keyboard(Key::Q) => {
                let damage = 100.0 - (10.0 * self.player1.attack(self.player2.position[0] + self.player2.position[2], self.player2.position[1]));
                let health = 100.0 - (10.0 * self.player2.attacked(self.player1.stance[1], self.player1.stats[0]));
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
            Button::Keyboard(Key::Return) => {
                self.reset = true;
            }
            _ => {}
        }
    }

    fn release(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(Key::O) => {
                self.player2.stance[2] = false;
            }
            Button::Keyboard(Key::E) =>{
                self.player1.stance[2] = false;
            }
            Button::Keyboard(Key::U) => {
                self.player2.stance[1] = false;
            }
            Button::Keyboard(Key::L) => {
                self.player2.movement[1] = false;
            }
            Button::Keyboard(Key::J) => {
                self.player2.movement[0] = false;
            }
            Button::Keyboard(Key::K) => {
                self.player2.movement[2] = false;
            }
            Button::Keyboard(Key::I) => {
                self.player2.movement[3] = false;
            }
            Button::Keyboard(Key::Q) => {
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
            Button::Keyboard(Key::Return) => {
                self.reset = false;
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
            [800, 400]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player1: LeftCat::new([1.0, 0.0, 0.0, 1.0], [105.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]),
        player2: RightCat::new([0.0, 0.0, 1.0, 1.0], [595.0, 200.0, 50.0, 50.0], [1.0, 2.0, 0.75, 10.0, 10.0], [true, false, false, false, false]),
        hud: HUD::new(),
        reset: false
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
pub struct LeftCat {
    pub color: [f32; 4], // R G B brightness?
    pub position: [f64; 4], // x, y, width, height
    pub stats: [f64; 5], // attack, spd, def, current health, total health
    pub movement: [bool; 4], // left, right, crouch, jump
    pub stance: [bool; 5], // stand, attack, defend, injured
}

pub struct RightCat {
    pub color: [f32; 4], // R G B brightness?
    pub position: [f64; 4], // x, y, width, height
    pub stats: [f64; 5], // attack, spd, def, current health, total health
    pub movement: [bool; 4], // left, right, crouch, jump
    pub stance: [bool; 5], // stand, attack, defend, injured
}

pub trait Cat {
    fn new(color: [f32; 4], position: [f64; 4], stats: [f64; 5]) -> Self;
    fn clone(&mut self) -> Self;
    fn move_cat(&mut self, position: f64);
    fn attacked(&mut self, other_cat_attacking: bool, other_cat_attack: f64) -> f64;
    fn attack(&mut self, other_cat: f64) -> f64;
    fn check_alive(&mut self) -> bool;
    fn hiss(&mut self, other_cat: f64);
    fn hissed(&mut self, other_cat_hissing: bool);
}

impl Cat for LeftCat {
    fn new(color: [f32; 4], position: [f64; 4], stats: [f64; 5]) -> Self {
        LeftCat {
            color: color,
            position: position,
            stats: stats,
            movement: [false, false, false, false],
            stance: [true, false, false, false, false]
        }
    }
    fn clone(&mut self) -> LeftCat {
        return LeftCat::new(self.color, self.position, self.stats);
    }
    fn attacked(&mut self, other_cat_attacking: bool, other_cat_attack: f64) -> f64 {
        if other_cat_attacking == true {
            self.stats[3] = self.stats[3] - other_cat_attack;
            println!("{}", self.stats[3]);
        }
        return self.stats[3];
    }
    fn attack(&mut self, other_cat: f64) -> f64 {
        if self.position[0] + self.position[2] >= (other_cat - 35.0) {
            self.stance[1] = true;
        }
        return self.stats[3];
    }
    fn hiss(&mut self, other_cat: f64) {
        if self.position[0] + self.position[2] >= (other_cat - 60.0) {
            self.stance[4] = true;
        }
        else {
            self.stance[4] = false;
        }
    }
    fn hissed(&mut self, other_cat_hissing: bool) {
        if other_cat_hissing == true {
            self.stance[0] = false;
            self.position[0] -= 30.0;
        }
    }
    fn move_cat(&mut self, other_cat: f64) {
        // left
        if self.movement[0] && self.position[0] >= 0.0 {
            self.stance[0] = false;
            self.position[0] -= self.stats[1];
        }
        //right
        if self.movement[1] && self.position[0] <= (750.0 - self.position[2]) && (self.position[0] + self.position[2] + 30.0) <= other_cat {
            self.stance[0] = false;
            self.position[0] += self.stats[1];
        }

        // jump rise
        if self.movement[3] {
            if self.position[1] > 145.0 {
                self.position[1] -= self.stats[1];
            } else {
                self.movement[3] = false;
            }
        }
        // jump fall
        if !self.movement[3] && self.position[1] < 200.0 {
            self.position[1] += self.stats[1];
        }

        // crouch
        if self.movement[2] && self.position[3] >= 145.0 {
            self.position[3] -= self.stats[1];
            self.position[1] += self.stats[1];
        }
        if !self.movement[2] && self.position[3] < 200.0 {
            self.position[3] += self.stats[1];
            self.position[1] -= self.stats[1];
        }
    }

    fn check_alive(&mut self) -> bool {
        if self.stats[3] <= 0.0 {
            return false;
        } else {
            return true;
        }
    }
}

impl Cat for RightCat {
    fn new(color: [f32; 4], position: [f64; 4], stats: [f64; 5]) -> Self {
        RightCat {
            color: color,
            position: position,
            stats: stats,
            movement: [false, false, false, false],
            stance: [true, false, false, false, false]
        }
    }
    fn clone(&mut self) -> RightCat {
        return RightCat::new(self.color, self.position, self.stats);
    }
    fn attacked(&mut self, other_cat_attacking: bool, other_cat_attack: f64) -> f64 {
        if other_cat_attacking == true {
            self.stats[3] = self.stats[3] - other_cat_attack;
            println!("Attacked!");
            println!("{}", self.stats[3]);
        }
        return self.stats[3];
    }
    fn attack(&mut self, other_cat: f64) -> f64 {
        if self.position[0] <= (other_cat + 35.0)  {
            self.stance[1] = true;
        }
        return self.stats[3];
    }
    fn hiss(&mut self, other_cat: f64) {
        if self.position[0] <= (other_cat + 60.0) {
            self.stance[4] = true;
        }
        else {
            self.stance[4] = false;
        }
    }
    fn hissed(&mut self, other_cat_hissing: bool) {
        if other_cat_hissing == true {
            self.stance[0] = false;
            self.position[0] += 30.0;
        }
    }
    fn move_cat(&mut self, other_cat: f64) {
        // left
        if self.movement[0] && self.position[0] >= 0.0 && (self.position[0] - 30.0) >= other_cat {
            self.stance[0] = false;
            self.position[0] -= self.stats[1];
        }

        //right
        if self.movement[1] && self.position[0] <= (750.0 - self.position[2]) {
            self.stance[0] = false;
            self.position[0] += self.stats[1];
        }

        // jump rise
        if self.movement[3] {
            if self.position[1] > 145.0 {
                self.position[1] -= self.stats[1];
            } else {
                self.movement[3] = false;
            }
        }
        // jump fall
        if !self.movement[3] && self.position[1] < 200.0 {
            self.position[1] += self.stats[1];
        }

        // crouch
        if self.movement[2] && self.position[3] >= 145.0 {
            self.position[3] -= self.stats[1];
            self.position[1] += self.stats[1];
        }
        if !self.movement[2] && self.position[3] < 200.0 {
            self.position[3] += self.stats[1];
            self.position[1] -= self.stats[1];
        }
    }

    fn check_alive(&mut self) -> bool {
        if self.stats[3] <= 0.0 {
            return false;
        } else {
            return true;
        }
    }
}
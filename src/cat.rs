pub struct LeftCat {
    pub color: [f32; 4], // R G B brightness?
    pub position: [f64; 4], // x, y, width, height
    pub stats: [f64; 5], // attack, spd, def, current health, total health
    pub movement: [bool; 4], // left, right, crouch, jump
    pub stance: [bool; 4], // stand, attack, defend, injured
}

pub struct RightCat {
    pub color: [f32; 4], // R G B brightness?
    pub position: [f64; 4], // x, y, width, height
    pub stats: [f64; 5], // attack, spd, def, current health, total health
    pub movement: [bool; 4], // left, right, crouch, jump
    pub stance: [bool; 4], // stand, attack, defend, injured
}

pub trait Cat {
    fn new(color: [f32; 4], position: [f64; 4], stats: [f64; 5]) -> Self;
    fn clone(&mut self) -> Self;
    fn move_cat(&mut self, position: f64);
}

impl Cat for LeftCat {
    fn new(color: [f32; 4], position: [f64; 4], stats: [f64; 5]) -> Self {
        LeftCat {
            color: color,
            position: position,
            stats: stats,
            movement: [false, false, false, false],
            stance: [true, false, false, false]
        }
    }
    fn clone(&mut self) -> LeftCat {
        return LeftCat::new(self.color, self.position, self.stats);
    }
    fn move_cat(&mut self, other_cat: f64) {
        // left
        if self.movement[0] && self.position[0] >= 0.0 {
            self.stance[0] = false;
            self.position[0] -= self.stats[1];
        }

        //right
        if self.movement[1] && self.position[0] <= (400.0 - self.position[2]) && (self.position[0] + self.position[2]) <= other_cat {
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

impl Cat for RightCat {
    fn new(color: [f32; 4], position: [f64; 4], stats: [f64; 5]) -> Self {
        RightCat {
            color: color,
            position: position,
            stats: stats,
            movement: [false, false, false, false],
            stance: [true, false, false, false]
        }
    }
    fn clone(&mut self) -> RightCat {
        return RightCat::new(self.color, self.position, self.stats);
    }
    fn move_cat(&mut self, other_cat: f64) {
        // left
        if self.movement[0] && self.position[0] >= 0.0 && self.position[0] >= other_cat {
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
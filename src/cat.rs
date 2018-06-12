pub struct LeftCat {
    pub color: [f32; 4], // R G B brightness?
    pub position: [f64; 4], // x, y, width, height
    pub stats: [f64; 5], // attack, spd, def, current health, total health
    pub movement: [bool; 4], // left, right, crouch, jump
    pub stance: [bool; 5] // stand, attack, defend, injured
}

pub struct RightCat {
    pub color: [f32; 4], // R G B brightness?
    pub position: [f64; 4], // x, y, width, height
    pub stats: [f64; 5], // attack, spd, def, current health, total health
    pub movement: [bool; 4], // left, right, crouch, jump
    pub stance: [bool; 5] // stand, attack, defend, injured
}

pub trait Cat {
    fn new(color: [f32; 4], position: [f64; 4], stats: [f64; 5], stance: [bool; 5]) -> Self;
    fn clone(&mut self) -> Self;
    fn move_cat(&mut self, position: f64);
    fn attacked(&mut self, other_cat_attacking: bool, other_cat_attack: f64, other_cat_x: f64, other_cat_y: f64) -> f64;
    fn attack(&mut self) -> f64;
    fn check_alive(&mut self) -> bool;
    fn hiss(&mut self);
    fn hissed(&mut self, other_cat_x: f64, other_cat_y: f64);
}

impl Cat for LeftCat {
    fn new(color: [f32; 4], position: [f64; 4], stats: [f64; 5], stance: [bool; 5]) -> Self {
        LeftCat {
            color: color,
            position: position,
            stats: stats,
            movement: [false, false, false, false],
            stance: stance
        }
    }
    fn clone(&mut self) -> LeftCat {
        return LeftCat::new(self.color, self.position, self.stats, self.stance);
    }
    fn attacked(&mut self, other_cat_attacking: bool, other_cat_attack: f64, other_cat_x: f64, other_cat_y: f64) -> f64 {
        if (other_cat_x - 100.0) <= self.position[0] + self.position[2] && (other_cat_y - 25.0) <= self.position[1] && other_cat_attacking == true {
            self.stats[3] = self.stats[3] - other_cat_attack;
        }
        return self.stats[3];
    }
    fn attack(&mut self) -> f64 {
        self.stance[1] = true;
        return self.stats[3];
    }
    fn hiss(&mut self) {
        self.stance[2] = true;
    }
    fn hissed(&mut self, other_cat_x: f64, other_cat_y: f64) {
        if self.position[0] + self.position[2] >= (other_cat_x - 100.0) && self.position[1] <= (other_cat_y + 25.0) {
            self.stance[0] = false;
            self.position[0] -= 30.0;
        }
    }
    fn move_cat(&mut self, other_cat: f64) {
        // left
        if self.movement[0] && self.position[0] > 0.0 {
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
    fn new(color: [f32; 4], position: [f64; 4], stats: [f64; 5], stance: [bool; 5]) -> Self {
        RightCat {
            color: color,
            position: position,
            stats: stats,
            movement: [false, false, false, false],
            stance: stance
        }
    }
    fn clone(&mut self) -> RightCat {
        return RightCat::new(self.color, self.position, self.stats, self.stance);
    }
    fn attacked(&mut self, other_cat_attacking: bool, other_cat_attack: f64, other_cat_x: f64, other_cat_y: f64) -> f64 {
        if (other_cat_x + 100.0) >= self.position[0] && (other_cat_y - 25.0) <= self.position[1] && other_cat_attacking == true {
            self.stats[3] = self.stats[3] - other_cat_attack;
        }
        return self.stats[3];
    }
    fn attack(&mut self) -> f64 {
            self.stance[1] = true;
        return self.stats[3];
    }
    fn hiss(&mut self) {
        self.stance[2] = true;
    }
    fn hissed(&mut self, other_cat_x: f64, other_cat_y: f64) {
        if (other_cat_x + 100.0) >= self.position[0] && (other_cat_y - 25.0) <= self.position[1] {
            self.stance[0] = false;
            self.position[0] += 30.0;
        }
    }
    fn move_cat(&mut self, other_cat: f64) {
        // left
        if self.movement[0] && self.position[0] > 0.0 && (self.position[0] - 30.0) >= other_cat {
            self.stance[0] = false;
            self.position[0] -= self.stats[1];
        }

        //right
        if self.movement[1] && self.position[0] < (750.0 - self.position[2]) {
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

/*************************/
/******* Cat Tests *******/
/*************************/

#[cfg(test)]
mod tests {
    /**************************/
    /***** Left Cat Tests *****/
    /**************************/

    #[test]
    fn left_cat_move_left_in_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let mut left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [50.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        left_cat.movement[0] = true;
        left_cat.move_cat(right_cat.position[0]);
        assert_eq!(left_cat.position[0], 49.0);
    }

    #[test]
    fn left_cat_move_left_out_of_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let mut left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [0.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        left_cat.movement[0] = true;
        left_cat.move_cat(right_cat.position[0]);
        assert_eq!(left_cat.position[0], 0.0);
    }

    #[test]
    fn left_cat_move_right_free() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let mut left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [0.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        left_cat.movement[1] = true;
        left_cat.move_cat(right_cat.position[0]);
        assert_eq!(left_cat.position[0], 1.0);
    }

    #[test]
    fn left_cat_move_right_blocked() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let mut left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        left_cat.movement[1] = true;
        left_cat.move_cat(right_cat.position[0]);
        assert_eq!(left_cat.position[0], 100.0);
    }

    #[test]
    fn check_live_left_cat_alive() {
        use cat::Cat;
        use cat::LeftCat;
        let mut cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [105.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let result = cat.check_alive();
        assert_eq!(result, true);
    }

    #[test]
    fn check_dead_left_cat_alive() {
        use cat::Cat;
        use cat::LeftCat;
        let mut cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [105.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        cat.stats[3] = 0.0;
        let result = cat.check_alive();
        assert_eq!(result, false);
    }

    #[test]
    fn left_cat_attacked_right_cat_in_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [50.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let mut right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let result = right_cat.attacked(true, 1.0, left_cat.position[0], left_cat.position[1]);
        assert_eq!(result, 9.0);
    }

    #[test]
    fn left_cat_attacked_right_cat_out_of_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [0.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let mut right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [200.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let result = right_cat.attacked(true, 1.0, left_cat.position[0], left_cat.position[1]);
        assert_eq!(result, 10.0);
    }

    #[test]
    fn left_cat_hissed_at_right_cat_in_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [50.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let mut right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        right_cat.hissed(left_cat.position[0], left_cat.position[1]);
        assert_eq!(right_cat.position[0], 130.0);
    }

    #[test]
    fn left_cat_hissed_at_right_cat_out_of_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [0.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let mut right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [200.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        right_cat.hissed(left_cat.position[0], left_cat.position[1]);
        assert_eq!(right_cat.position[0], 200.0);
    }

    /***************************/
    /***** Right Cat Tests *****/
    /***************************/
    
     #[test]
    fn right_cat_move_right_in_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [50.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let mut right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        right_cat.movement[1] = true;
        right_cat.move_cat(left_cat.position[0]);
        assert_eq!(right_cat.position[0], 101.0);
    }

    #[test]
    fn right_cat_move_right_out_of_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [0.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let mut right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [700.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        right_cat.movement[1] = true;
        right_cat.move_cat(left_cat.position[0]);
        assert_eq!(right_cat.position[0], 700.0);
    }

    #[test]
    fn right_cat_move_left_free() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [0.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let mut right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        right_cat.movement[0] = true;
        right_cat.move_cat(left_cat.position[0]);
        assert_eq!(right_cat.position[0], 99.0);
    }

    #[test]
    fn right_cat_move_left_blocked() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let mut right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 1.0, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        right_cat.movement[0] = true;
        right_cat.move_cat(left_cat.position[0]);
        assert_eq!(right_cat.position[0], 100.0);
    }

    #[test]
    fn check_live_right_cat_alive() {
        use cat::Cat;
        use cat::RightCat;
        let mut cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [105.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let result = cat.check_alive();
        assert_eq!(result, true);
    }

    #[test]
    fn check_dead_right_cat_alive() {
        use cat::Cat;
        use cat::RightCat;
        let mut cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [105.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        cat.stats[3] = 0.0;
        let result = cat.check_alive();
        assert_eq!(result, false);
    }

    #[test]
    fn right_cat_attacked_left_cat_in_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let mut left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [50.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let result = left_cat.attacked(true, 1.0, right_cat.position[0], right_cat.position[1]);
        assert_eq!(result, 9.0);
    }

    #[test]
    fn right_cat_attacked_left_cat_out_of_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let mut left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [0.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [200.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let result = left_cat.attacked(true, 1.0, right_cat.position[0], right_cat.position[1]);
        assert_eq!(result, 10.0);
    }

    #[test]
    fn right_cat_hissed_at_left_cat_in_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let mut left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [50.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [100.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        left_cat.hissed(right_cat.position[0], right_cat.position[1]);
        assert_eq!(left_cat.position[0], 20.0);
    }

    #[test]
    fn right_cat_hissed_at_left_cat_out_of_bounds() {
        use cat::Cat;
        use cat::LeftCat;
        use cat::RightCat;
        let mut left_cat: LeftCat = LeftCat::new([1.0, 0.0, 0.0, 1.0], [0.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        let right_cat: RightCat = RightCat::new([1.0, 0.0, 0.0, 1.0], [200.0, 200.0, 50.0, 50.0], [2.0, 0.75, 1.0, 10.0, 10.0], [true, false, false, false, false]);
        left_cat.hissed(right_cat.position[0], right_cat.position[1]);
        assert_eq!(left_cat.position[0], 0.0);
    }
}
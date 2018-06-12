pub struct HUD {
    pub health_bar_left: HealthBar,
    pub health_bar_right: HealthBar,
    pub timer: f64
}

impl HUD {
    pub fn new() -> Self {
        HUD {
            health_bar_left: HealthBar::new(105.0, 0.0),
            health_bar_right: HealthBar::new(595.0, 0.0),
            timer: 100.0
        }
    }

    pub fn update(&mut self, left_damage: f64, right_damage: f64) {
        println!("left cat: {}, right cat: {}", left_damage, right_damage);
        if left_damage != 0.0 {
            self.health_bar_left.update(left_damage);
        }

        if right_damage != 0.0 {
            self.health_bar_right.update(right_damage);
        }
    }
}

pub struct HealthBar {
    pub outer: [f64; 4], // x, y, width, height
    pub inner: [f64; 4], // x, y, width, height
    pub damage: [f64; 4], // x, y, width, height
}

impl HealthBar {
    fn new(x: f64, damage: f64) -> HealthBar {
        HealthBar {
            outer: [ x, 100.0, 102.0, 12.0 ],
            inner: [ x+1.0, 101.0, 100.0, 10.0 ],
            damage: [ x+1.0, 101.0, damage, 10.0 ]
        }
    }

    fn update(&mut self, damage_value: f64) {
        if self.damage[2] < self.inner[2] {
            self.damage[2] = damage_value;
        }
    }
}

pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        return Self { min, max }
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }

    pub const EMPTY: Interval = Interval::new(f64::INFINITY, -f64::INFINITY);
    pub const MAX: Interval = Interval::new(-f64::INFINITY, f64::INFINITY);
    pub const FORWARD: Interval = Interval::new(0.0, f64::INFINITY);
    pub const ALMOST_FORWARD: Interval = Interval::new(0.001, f64::INFINITY);
}
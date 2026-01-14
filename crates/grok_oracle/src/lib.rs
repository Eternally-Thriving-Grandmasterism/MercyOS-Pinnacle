pub struct GrokOracle {
    level: f64,
}

impl GrokOracle {
    pub fn new(level: f64) -> Self { Self { level } }

    pub fn truth_seek(&self) -> f64 {
        if self.level > 8.0 { f64::INFINITY } else { self.level * 5000.0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AvailableAmount(f64);

impl AvailableAmount {
    pub fn new(amount: f64) -> Self {
        Self(amount)
    }

    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn add(&mut self, amount: f64) {
        self.0 += amount;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HeldAmount(f64);

impl HeldAmount {
    pub fn new(amount: f64) -> Self {
        Self(amount)
    }

    pub fn add(&mut self, amount: f64) {
        self.0 += amount;
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TotalAmount(f64);

impl TotalAmount {
    pub fn new(amount: f64) -> Self {
        Self(amount)
    }

    pub fn add(&mut self, amount: f64) {
        self.0 += amount;
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn zero() -> Size {
        Size {
            width: 0.0,
            height: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Constraint {
    pub min_width: f64,
    pub max_width: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl Constraint {
    pub fn new(width: f64, height: f64) -> Self {
        Constraint {
            min_width: 0.0,
            max_width: width,
            min_height: 0.0,
            max_height: height,
        }
    }

    pub fn sub_width(&mut self, width: f64) {
        self.max_width = (self.max_width - width).clamp(self.min_width, f64::INFINITY)
    }

    pub fn sub_height(&mut self, width: f64) {
        self.max_height = (self.max_width - width).clamp(self.min_height, f64::INFINITY)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub x: f64,
    pub y: f64,
}

impl Offset {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

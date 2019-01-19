#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn normalized(self) -> Vec2 {
        let len = self.x.hypot(self.y);
        if len <= 0.0 {
            self
        } else {
            self / len
        }
    }

    pub fn rot90(self) -> Vec2 {
        vec2(self.y, -self.x)
    }

    pub fn length(self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, factor: f32) -> Vec2 {
        Vec2 {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2 {
            x: self * vec.x,
            y: self * vec.y,
        }
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, factor: f32) -> Vec2 {
        Vec2 {
            x: self.x / factor,
            y: self.y / factor,
        }
    }
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Rect {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn from_min_size(min: Vec2, size: Vec2) -> Self {
        Rect { pos: min, size }
    }

    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        Rect {
            pos: center - size * 0.5,
            size,
        }
    }

    pub fn contains(&self, p: Vec2) -> bool {
        self.pos.x <= p.x
            && p.x <= self.pos.x + self.size.x
            && self.pos.y <= p.y
            && p.y <= self.pos.y + self.size.y
    }

    pub fn center(&self) -> Vec2 {
        Vec2 {
            x: self.pos.x + self.size.x / 2.0,
            y: self.pos.y + self.size.y / 2.0,
        }
    }

    pub fn min(&self) -> Vec2 {
        self.pos
    }
    pub fn max(&self) -> Vec2 {
        self.pos + self.size
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    // Convenience functions:
    pub fn left_top(&self) -> Vec2 {
        vec2(self.min().x, self.min().y)
    }
    pub fn center_top(&self) -> Vec2 {
        vec2(self.center().x, self.min().y)
    }
    pub fn right_top(&self) -> Vec2 {
        vec2(self.max().x, self.min().y)
    }
    pub fn left_center(&self) -> Vec2 {
        vec2(self.min().x, self.center().y)
    }
    pub fn right_center(&self) -> Vec2 {
        vec2(self.max().x, self.center().y)
    }
    pub fn left_bottom(&self) -> Vec2 {
        vec2(self.min().x, self.max().y)
    }
    pub fn center_bottom(&self) -> Vec2 {
        vec2(self.center().x, self.max().y)
    }
    pub fn right_bottom(&self) -> Vec2 {
        vec2(self.max().x, self.max().y)
    }
}

// ----------------------------------------------------------------------------

pub fn lerp(min: f32, max: f32, t: f32) -> f32 {
    (1.0 - t) * min + t * max
}

pub fn remap(from: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let t = (from - from_min) / (from_max - from_min);
    lerp(to_min, to_max, t)
}

pub fn remap_clamp(from: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let t = if from <= from_min {
        0.0
    } else if from >= from_max {
        1.0
    } else {
        (from - from_min) / (from_max - from_min)
    };
    lerp(to_min, to_max, t)
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x <= min {
        min
    } else if x >= max {
        max
    } else {
        x
    }
}

pub const TAU: f32 = 2.0 * std::f32::consts::PI;

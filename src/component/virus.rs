use specs::{Component, VecStorage};
use config;

#[storage(VecStorage)]
#[derive(Component, Default, Clone, Copy)]
pub struct Virus {
    pub lifetime: f64,
    pub amplitude: f64,
    pub offset: f64
}

impl Virus {
    pub fn new(amplitude: f64, offset: f64) -> Self {
        Virus{
            lifetime: 0.0,
            amplitude,
            offset
        }
    }

    /// Straight virus hit on botton
    pub fn straight_easy_bottom() -> Self {
        Virus::new(20.0, config::GROUND + 30.0)
    }

    /// sinus motion end at the top
    pub fn sin_hard_bottom() -> Self {
         Virus::new(50.0, config::GROUND + 60.0)
    }

    /// sinus motion end at the top
    pub fn sin_hard_top() -> Self {
         Virus::new(60.0, config::GROUND + 70.0)
    }

    pub fn sin_easy_bottom() -> Self {
        Virus::new(20.0, config::GROUND + 50.0)
    }

    pub fn sin_easy_top() -> Self {
        Virus::new(100.0, config::GROUND + 170.0)
    }
}


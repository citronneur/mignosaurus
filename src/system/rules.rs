use specs::{System, Read, Write};
use system::dino::Jump;
use system::rules::RulesState::RUN;

#[derive(Eq, PartialEq)]
pub enum RulesState {
    PENDING,
    RUN,
    END
}

impl Default for RulesState {
    fn default() -> Self {
        RulesState::PENDING
    }
}

#[derive(Default)]
pub struct Rules {
    pub state: RulesState,
    pub score: u32,
    pub rel_speed: f64
}

impl Rules {
    pub fn new() -> Self {
        Rules {
            state: RulesState::PENDING,
            score: 0,
            rel_speed: 0.0
        }
    }
}

pub struct RulesSystem;
impl<'a> System<'a> for RulesSystem {
    type SystemData = (
        Write<'a, Rules>,
        Read<'a, Jump>
    );

    fn run(&mut self, (mut rules, jump): Self::SystemData) {
        if *jump && rules.state == RulesState::PENDING{
            rules.state = RulesState::RUN;
        }
    }
}
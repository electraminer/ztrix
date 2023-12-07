
use std::fmt;
use std::str::FromStr;

use crate::condition::event::Conditions;
use crate::game::Action;
use crate::game::Game;
use crate::game::game::Event;
use crate::kirb_generator;
use crate::tsd_generator;
use crate::replay::Info;
use crate::serialize::DeserializeError;
use crate::serialize::SerializeUrlSafe;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Puzzle {
    pub game: Game,
    pub win_conditions: Conditions,
    pub end_conditions: Conditions,
    pub won: bool,
    pub over: bool,
}

impl Puzzle {
    pub fn new(game: Game) -> Self {
        Self {
            game: game,
            win_conditions: Conditions{ conditions: Vec::new() },
            end_conditions: Conditions{ conditions: Vec::new() },
            won: false,
            over: false,
        }
    }

    pub fn update<F>(&mut self, action: Action, info: &mut Info, event_handler: &mut F)
    where   F: FnMut(&Event) {
        if self.over {
            return;
        }
        self.game.update(action, info, &mut |event| {
            event_handler(event);
            self.win_conditions.handle_event(event);
            self.end_conditions.handle_event(event);
        });
        if self.win_conditions.conditions.len() > 0 &&
            self.win_conditions.statuses().iter().all(|s| *s) {
            self.won = true;
        }
        if self.game.over || self.end_conditions.statuses().iter().any(|s| *s) {
            self.over = true;
        }
    }

    pub fn get_game(&self) -> &Game {
		&self.game
	}

    pub fn generate_kirb_puzzle(difficulty: u32) -> Self {
        kirb_generator::generate(difficulty)
    }
    
    pub fn generate_tsd_puzzle() -> Self {
        tsd_generator::generate()
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self::new(Game::default())
    }
}

impl SerializeUrlSafe for Puzzle {
    fn serialize(&self) -> String {
        format! {"{}{}{}{}{}",
            self.game.serialize(),
            self.win_conditions.serialize(),
            self.end_conditions.serialize(),
            self.won.serialize(),
            self.over.serialize(),
        }
    }

    fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
        Ok(Self {
            game: Game::deserialize(input)?,
            win_conditions: Conditions::deserialize(input)?,
            end_conditions: Conditions::deserialize(input)?,
            won: bool::deserialize(input)?,
            over: bool::deserialize(input)?,
        })
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

impl FromStr for Puzzle {
	type Err = DeserializeError;
	fn from_str(string: &str) -> Result<Self, DeserializeError> {
		Self::deserialize_string(string)
	}
}
use crate::condition::all_clear::AllClearType;
use crate::condition::chain::ChainConditions;
use crate::condition::chain::ChainHandler;
use crate::condition::chain::ChainScorer;
use crate::condition::event::Conditions;
use crate::condition::event::EventConditions;
use crate::condition::event::EventScorer;
use crate::condition::event::ReqOrMin;
use crate::condition::event::ScoreTarget;
use crate::condition::spin::SpinConditions;
use crate::condition::spin::SpinHandler;
use crate::game::Action;
use crate::game::Game;
use crate::game::game::Event;
use crate::replay::Info;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Puzzle {
    game: Game,
    pub win_conditions: Conditions,
    pub end_conditions: Conditions,
    pub won: bool,
    pub over: bool,
}

impl Puzzle {
    pub fn new(game: Game) -> Self {
        // TEMPORARY PUZZLE TESTING HERE
        Self {
            game: game,
            win_conditions: Conditions{ conditions: vec![
                EventConditions::TSpinContext(SpinHandler::new(None), vec![
                    SpinConditions::ZoneChainContext(ChainHandler::new(false, 0), vec![
                        ChainConditions::Condition(
                            ScoreTarget{score: 0, target: 0},
                            ChainScorer::DamageDealt {count_zone_damage: true}
                        ),
                        ChainConditions::Condition(
                            ScoreTarget{score: 0, target: 1},
                            ChainScorer::ZoneClear {req_lines: ReqOrMin::Min(0)}
                        ),
                    ])
                ])
            ] },
            end_conditions: Conditions{ conditions: vec![
                EventConditions::Condition(
                    ScoreTarget{score: 0, target: 1},
                    EventScorer::LineClear { req_lines: ReqOrMin::Min(1), req_piece: None,
                        req_all_clear: AllClearType::ALL_CLEAR, negate: false }
                ),
                EventConditions::Condition(
                    ScoreTarget{score: 0, target: 1},
                    EventScorer::ZoneClear {req_lines: ReqOrMin::Min(0)}
                ),
            ] },
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
}

impl Default for Puzzle {
    fn default() -> Self {
        Self::new(Game::default())
    }
}
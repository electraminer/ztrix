use core::ops::Add;

use crate::condition::all_clear::AllClearType;
use crate::condition::chain::ChainConditions;
use crate::condition::chain::ChainHandler;
use crate::condition::event::ScoreTarget;
use crate::game::PieceType;
use crate::game::game::Event;
use crate::game::game::LineClear;
use crate::position::Vector;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum SpinType {
    Full,
    Mini,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct SpinClear<'a> {
    pub clear: &'a LineClear,
    pub spin: Option<SpinType>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum SpinEvent<'a> {
    LineClear(SpinClear<'a>),
    ZoneClear(usize),
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct SpinHandler {
    last_kick: Option<usize>,
}

impl SpinHandler {
    pub fn new(last_kick: Option<usize>) -> Self {
        Self {last_kick}
    }

    fn handle_last_kick(&mut self, event: &Event) -> Option<usize> {
        match *event {
            Event::Spawn => self.last_kick = None,
            Event::Move => self.last_kick = None,
            Event::Rotate(kick) => self.last_kick = Some(kick),
            _ => (),
        }
        return self.last_kick;
    }

    pub fn handle_t_spin<'a>(&mut self, event: &'a Event) -> Option<SpinEvent<'a>> {
        let kick = self.handle_last_kick(event);
        match event {
            Event::LineClear(clear) => Some(SpinEvent::LineClear(SpinClear {
                clear: clear,
                spin: kick.and_then(|k| {
                    let main_corners = [Vector::new(-1, 1),
                        Vector::new(1, 1)].iter()
                        .map(|v| clear.active.pos.add(v.rotate(clear.active.rot)))
                        .filter(|p| clear.board[*p] != None).count();
                    let mini_corners = [Vector::new(-1, 1),
                        Vector::new(1, 1)].iter()
                        .map(|v| clear.active.pos.add(v.rotate(clear.active.rot)))
                        .filter(|p| clear.board[*p] != None).count();
                    let corners = main_corners + mini_corners;
                    if corners < 3 {
                        None
                    } else {
                        Some(match main_corners < 2 && k != 4 {
                            false => SpinType::Full,
                            true => SpinType::Mini,
                        })
                    }
                }),
            })),
            Event::ZoneClear(lines) => Some(SpinEvent::ZoneClear(*lines)),
            _ => None,
        }
    }

    pub fn handle_all_spin<'a>(&mut self, event: &'a Event) -> Option<SpinEvent<'a>> {
        let kick = self.handle_last_kick(event);
        match event {
            Event::LineClear(clear) => Some(SpinEvent::LineClear(SpinClear {
                clear: clear,
                spin: kick.and_then(|k| {
                    if [Vector::ONE_UP, Vector::ONE_DOWN,
                        Vector::ONE_LEFT, Vector::ONE_RIGHT].iter()
                        .any(|v| clear.active.clone().try_move(&clear.board, *v)) {
                        None
                    } else {
                        let positions = clear.active.get_mino_positions();
                        let min = positions.iter().map(|p| p.y).min()
                            .unwrap_or(0);
                        let max = positions.iter().map(|p| p.y).max()
                            .unwrap_or(0);
                        let height = (max - min + 1) as usize;
                        Some(match clear.lines < height && k > 0 {
                            false => SpinType::Full,
                            true => SpinType::Mini,
                        })
                    }
                }),
            })),
            Event::ZoneClear(lines) => Some(SpinEvent::ZoneClear(*lines)),
            _ => None,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum SpinConditions {
    ChainContext(ChainHandler, Vec<ChainConditions>),
    ZoneChainContext(ChainHandler, Vec<ChainConditions>),
    Condition(ScoreTarget, SpinScorer),
}

impl SpinConditions {
    pub fn handle_event(&mut self, event: &SpinEvent) {
        match self {
            Self::ChainContext(handler, conditions) =>
                if let Some(chain_event) = handler.handle_no_zone(event) {
                    for condition in conditions.iter_mut() {
                        condition.handle_event(&chain_event);
                    }
                }
            Self::ZoneChainContext(handler, conditions) =>
                if let Some(chain_event) = handler.handle_zone(event) {
                    for condition in conditions.iter_mut() {
                        condition.handle_event(&chain_event);
                    }
                }
            Self::Condition(target, scorer) =>
                target.score += scorer.score_event(event),
        }
    }
    
    pub fn statuses(&self) -> Vec<bool> {
        match self {
            Self::ChainContext(_, conditions) =>
                conditions.iter().flat_map(|c| c.statuses()).collect(),
            Self::ZoneChainContext(_, conditions) =>
                conditions.iter().flat_map(|c| c.statuses()).collect(),
            Self::Condition(target, _) => vec![target.score >= target.target],
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum SpinScorer {
    // Count occurrences, increasing by only one
    LineClear {
        required_lines: Option<usize>,
        required_piece: Option<PieceType>,
        required_all_clear: AllClearType,
        required_spin: Option<Option<SpinType>>,
        negate: bool,
    },
    ZoneClear {
        min_lines: usize,
    },
    // Count totals, sometimes increasing by more than one
    LinesCleared,
}

impl SpinScorer {
    fn score_event(&self, event: &SpinEvent) -> usize {
        match self {
            Self::LineClear{required_lines, required_piece, required_all_clear,
                required_spin,
                negate} => if let SpinEvent::LineClear(spin_clear) = event {
                    let clear = spin_clear.clear;
                    let is_clear = clear.lines > 0;
                    let reqs_met = required_lines.map_or(is_clear,
                        |r| clear.lines == r)
                        && required_piece.map_or(true,
                        |r| clear.active.get_type() == r)
                        && AllClearType::from_line_clear(&clear)
                            .fits_req(&required_all_clear)
                        && required_spin.as_ref().map_or(true,
                            |r| spin_clear.spin.as_ref().map_or(false,
                                |t| r.as_ref().map_or(false, |r| r == t)));
                    if match negate {
                        false => reqs_met,
                        true => !reqs_met && is_clear,
                    } {
                        return 1;
                    }
                }
            Self::ZoneClear {min_lines} =>
                if let SpinEvent::ZoneClear(lines) = *event {
                    if lines >= *min_lines {
                        return 1;
                    }
                }
            Self::LinesCleared =>
                if let SpinEvent::LineClear(clear) = event {
                    return clear.clear.lines;
                }
        }
        0
    }
}
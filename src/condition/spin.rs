use core::ops::Add;

use crate::condition::all_clear::AllClearType;
use crate::condition::chain::ChainConditions;
use crate::condition::chain::ChainHandler;
use crate::condition::event::ReqOrMin;
use crate::condition::event::ScoreTarget;
use crate::game::PieceType;
use crate::game::game::Event;
use crate::game::game::LineClear;
use crate::position::Vector;
use crate::serialize::DeserializeError;
use crate::serialize::SerializeUrlSafe;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum SpinType {
    Full,
    Mini,
}

impl SerializeUrlSafe for SpinType {
	fn serialize(&self) -> String {
		match self {
			Self::Full => "F",
			Self::Mini => "M",
		}.to_owned()
	}

	fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
		Ok(match input.next()? {
			'F' => Self::Full,
			'M' => Self::Mini,
			_ => return Err(DeserializeError::new("SpinType should be represented by F or M.")),
		})
	}
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct SpinClear<'a> {
    pub clear: &'a LineClear,
    pub spin: Option<SpinType>,
    pub hard: bool,
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
            Event::LineClear(clear) => {
                let spin = if clear.active.piece_type == PieceType::T {
                    kick.and_then(|k| {
                        let back_corners = [Vector::new(-1, -1),
                            Vector::new(1, -1)].iter()
                            .map(|v| clear.active.pos.add(v.rotate(clear.active.rot)))
                            .filter(|p| clear.board[*p] != None).count();
                        let front_corners = [Vector::new(-1, 1),
                            Vector::new(1, 1)].iter()
                            .map(|v| clear.active.pos.add(v.rotate(clear.active.rot)))
                            .filter(|p| clear.board[*p] != None).count();
                        if back_corners + front_corners < 3 {
                            None
                        } else {
                            Some(match front_corners == 2 || k == 4 {
                                true => SpinType::Full,
                                false => SpinType::Mini,
                            })
                        }
                    })
                } else {
                    None
                };
                Some(SpinEvent::LineClear(SpinClear {
                    clear: clear,
                    spin: spin.clone(),
                    hard: clear.lines >= 4 || spin != None,
                }))
            }
            Event::ZoneClear(lines) => Some(SpinEvent::ZoneClear(*lines)),
            _ => None,
        }
    }

    pub fn handle_all_spin<'a>(&mut self, event: &'a Event) -> Option<SpinEvent<'a>> {
        let kick = self.handle_last_kick(event);
        match event {
            Event::LineClear(clear) => {
                let spin = kick.and_then(|k| {
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
                });
                Some(SpinEvent::LineClear(SpinClear {
                    clear: clear,
                    spin: spin.clone(),
                    hard: clear.lines >= 4 || spin != None,
                }))
            }
            Event::ZoneClear(lines) => Some(SpinEvent::ZoneClear(*lines)),
            _ => None,
        }
    }
}

impl SerializeUrlSafe for SpinHandler {
    fn serialize(&self) -> String {
        format! {"{}",
            self.last_kick.serialize(),
        }
    }

    fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
        Ok(Self {
            last_kick: Option::deserialize(input)?,
        })
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

impl SerializeUrlSafe for SpinConditions {
    fn serialize(&self) -> String {
        match self {
            Self::ChainContext(handler, conditions) =>
                format! {"H{}{}", handler.serialize(), conditions.serialize()},
            Self::ZoneChainContext(handler, conditions) =>
                format! {"Z{}{}", handler.serialize(), conditions.serialize()},
            Self::Condition(target, scorer) => 
                format! {"C{}{}", target.serialize(), scorer.serialize()},
        }
    }

    fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
        Ok(match input.next()? {
            'H' => Self::ChainContext(ChainHandler::deserialize(input)?, Vec::deserialize(input)?),
            'Z' => Self::ZoneChainContext(ChainHandler::deserialize(input)?, Vec::deserialize(input)?),
            'C' => Self::Condition(ScoreTarget::deserialize(input)?, SpinScorer::deserialize(input)?),
            _ => return Err(DeserializeError::new("SpinConditions type should be represented by H, Z, or C."),
            )
        })
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum SpinScorer {
    // Count occurrences, increasing by only one
    LineClear {
        req_lines: ReqOrMin,
        req_piece: Option<PieceType>,
        req_all_clear: AllClearType,
        req_spin: Option<Option<SpinType>>,
        req_hard: Option<bool>,
        negate: bool,
    },
    ZoneClear {
        req_lines: ReqOrMin,
    },
    // Count totals, sometimes increasing by more than one
    LinesCleared,
}

impl SpinScorer {
    fn score_event(&self, event: &SpinEvent) -> usize {
        match self {
            Self::LineClear{req_lines, req_piece, req_all_clear,
                req_spin, req_hard, negate} =>
                if let SpinEvent::LineClear(spin_clear) = event {
                    let clear = spin_clear.clear;
                    let is_clear = clear.lines > 0;
                    let reqs_met = req_lines.matches(clear.lines)
                        && req_piece.map_or(true,
                        |r| clear.active.get_type() == r)
                        && AllClearType::from_line_clear(&clear)
                            .fits_req(&req_all_clear)
                        && req_spin.as_ref().map_or(true,
                            |r| spin_clear.spin.as_ref().map_or(false,
                                |t| r.as_ref().map_or(true, |r| r == t)))
                        && req_hard.map_or(true,
                            |r| spin_clear.hard == r);
                    if match negate {
                        false => reqs_met,
                        true => !reqs_met && is_clear,
                    } {
                        return 1;
                    }
                }
            Self::ZoneClear {req_lines} =>
                if let SpinEvent::ZoneClear(lines) = *event {
                    if req_lines.matches(lines) {
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

impl SerializeUrlSafe for SpinScorer {
    fn serialize(&self) -> String {
        match self {
            Self::LineClear { req_lines, req_piece, req_all_clear,
                req_spin, req_hard, negate } => format! {"C{}{}{}{}{}{}",
                req_lines.serialize(), req_piece.serialize(), req_all_clear.serialize(),
                req_spin.serialize(), req_hard.serialize(), negate.serialize()},
            Self::ZoneClear { req_lines } => format!("Z{}", req_lines.serialize()),
            Self::LinesCleared => "L".to_owned(),
        }
    }

    fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
        Ok(match input.next()? {
            'C' => Self::LineClear {
                req_lines: ReqOrMin::deserialize(input)?,
                req_piece: Option::deserialize(input)?,
                req_all_clear: AllClearType::deserialize(input)?,
                req_spin: Option::deserialize(input)?,
                req_hard: Option::deserialize(input)?,
                negate: bool::deserialize(input)?,
            },
            'Z' => Self::ZoneClear { req_lines: ReqOrMin::deserialize(input)? },
            'L' => Self::LinesCleared,
            _ => return Err(DeserializeError::new("SpinScorer type should be represented by C, Z, or L.")),
        })
    }
}
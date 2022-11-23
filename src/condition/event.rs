use crate::condition::all_clear::AllClearType;
use crate::condition::spin::SpinConditions;
use crate::condition::spin::SpinHandler;
use crate::game::PieceType;
use crate::game::game::Event;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ScoreTarget {
    pub score: usize,
    pub target: usize,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Conditions {
    pub conditions: Vec<EventConditions>
}

impl Conditions {
    pub fn handle_event(&mut self, event: &Event) {
        for condition in self.conditions.iter_mut() {
            condition.handle_event(event);
        }
    }

    pub fn statuses(&self) -> Vec<bool> {
        self.conditions.iter().flat_map(|c| c.statuses()).collect()
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum EventConditions {
    TSpinContext(SpinHandler, Vec<SpinConditions>),
    AllSpinContext(SpinHandler, Vec<SpinConditions>),
    Condition(ScoreTarget, EventScorer),
}

impl EventConditions {
    pub fn handle_event(&mut self, event: &Event) {
        match self {
            Self::TSpinContext(handler, conditions) =>
                if let Some(spin_event) = handler.handle_t_spin(event) {
                    for condition in conditions.iter_mut() {
                        condition.handle_event(&spin_event);
                    }
                }
            Self::AllSpinContext(handler, conditions) =>
                if let Some(spin_event) = handler.handle_all_spin(event) {
                    for condition in conditions.iter_mut() {
                        condition.handle_event(&spin_event);
                    }
                }
            Self::Condition(target, scorer) =>
                target.score += scorer.score_event(event),
        }
    }

    pub fn statuses(&self) -> Vec<bool> {
        match self {
            Self::TSpinContext(_, conditions) =>
                conditions.iter().flat_map(|c| c.statuses()).collect(),
            Self::AllSpinContext(_, conditions) =>
                conditions.iter().flat_map(|c| c.statuses()).collect(),
            Self::Condition(target, _) => vec![target.score >= target.target],
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum ReqOrMin {
    Req(usize),
    Min(usize),
}

impl ReqOrMin {
    pub fn matches(self, value: usize) -> bool {
        match self {
            Self::Req(req) => value == req,
            Self::Min(min) => value >= min,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum EventScorer {
    // Count occurrences, increasing by only one
    LineClear {
        req_lines: ReqOrMin,
        req_piece: Option<PieceType>,
        req_all_clear: AllClearType,
        negate: bool,
    },
    ZoneClear {
        req_lines: ReqOrMin,
    },
    // Count totals, sometimes increasing by more than one
    LinesCleared,
}

impl EventScorer {
    fn score_event(&self, event: &Event) -> usize {
        match self {
            Self::LineClear{req_lines, req_piece, req_all_clear,
                negate} => if let Event::LineClear(clear) = event {
                    let is_clear = clear.lines > 0;
                    let reqs_met = req_lines.matches(clear.lines)
                        && req_piece.map_or(true,
                        |r| clear.active.get_type() == r)
                        && AllClearType::from_line_clear(&clear)
                            .fits_req(&req_all_clear);
                    if match negate {
                        false => reqs_met,
                        true => !reqs_met && is_clear,
                    } {
                        return 1;
                    }
                }
            Self::ZoneClear {req_lines} =>
                if let Event::ZoneClear(lines) = event {
                    if req_lines.matches(*lines) {
                        return 1;
                    }
                }
            Self::LinesCleared =>
                if let Event::LineClear(clear) = event {
                    return clear.lines;
                }
        }
        0
    }
}
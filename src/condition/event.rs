use crate::condition::all_clear::AllClearType;
use crate::condition::spin::SpinConditions;
use crate::condition::spin::SpinHandler;
use crate::game::PieceType;
use crate::game::game::Event;
use crate::serialize::DeserializeError;
use crate::serialize::SerializeUrlSafe;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ScoreTarget {
    pub score: usize,
    pub target: usize,
}

impl SerializeUrlSafe for ScoreTarget {
    fn serialize(&self) -> String {
        format! {"{}{}",
            self.score.serialize(),
            self.target.serialize(),
        }
    }

    fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
        Ok(Self {
            score: usize::deserialize(input)?,
            target: usize::deserialize(input)?,
        })
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

impl SerializeUrlSafe for ReqOrMin {
    fn serialize(&self) -> String {
        match self {
            Self::Req(req) => format! {"R{}", req.serialize()},
            Self::Min(min) => format! {"M{}", min.serialize()},
        }
    }

    fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
        Ok(match input.next()? {
            'R' => Self::Req(usize::deserialize(input)?),
            'M' => Self::Min(usize::deserialize(input)?),
            _ => return Err(DeserializeError::new("ReqOrMin type should be represented by R or M."),
            )
        })
    }
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

impl SerializeUrlSafe for Conditions {
    fn serialize(&self) -> String {
        self.conditions.serialize()
    }

    fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
        Ok(Self { conditions: Vec::deserialize(input)? })
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

impl SerializeUrlSafe for EventConditions {
    fn serialize(&self) -> String {
        match self {
            Self::TSpinContext(handler, conditions) =>
                format! {"T{}{}", handler.serialize(), conditions.serialize()},
            Self::AllSpinContext(handler, conditions) =>
                format! {"A{}{}", handler.serialize(), conditions.serialize()},
            Self::Condition(target, scorer) => 
                format! {"C{}{}", target.serialize(), scorer.serialize()},
        }
    }

    fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
        Ok(match input.next()? {
            'T' => Self::TSpinContext(SpinHandler::deserialize(input)?, Vec::deserialize(input)?),
            'A' => Self::AllSpinContext(SpinHandler::deserialize(input)?, Vec::deserialize(input)?),
            'C' => Self::Condition(ScoreTarget::deserialize(input)?, EventScorer::deserialize(input)?),
            _ => return Err(DeserializeError::new("EventConditions type should be represented by T, A, or C."),
            )
        })
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

impl SerializeUrlSafe for EventScorer {
    fn serialize(&self) -> String {
        match self {
            Self::LineClear { req_lines, req_piece, req_all_clear, negate } =>
                format! {"C{}{}{}{}", req_lines.serialize(), req_piece.serialize(), req_all_clear.serialize(), negate.serialize()},
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
                negate: bool::deserialize(input)?,
            },
            'Z' => Self::ZoneClear { req_lines: ReqOrMin::deserialize(input)? },
            'L' => Self::LinesCleared,
            _ => return Err(DeserializeError::new("EventScorer type should be represented by C, Z, or L.")),
        })
    }
}
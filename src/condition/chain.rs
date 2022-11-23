use crate::condition::all_clear::AllClearType;
use crate::condition::event::ReqOrMin;
use crate::condition::event::ScoreTarget;
use crate::condition::spin::SpinClear;
use crate::condition::spin::SpinEvent;
use crate::condition::spin::SpinType;
use crate::game::PieceType;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ChainClear<'a> {
    pub clear: &'a SpinClear<'a>,
    pub b2b: bool,
    pub combo: usize,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum ChainEvent<'a> {
    LineClear(ChainClear<'a>),
    ZoneClear(usize),
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ChainHandler {
    pub b2b: bool,
    pub combo: usize,
}

impl ChainHandler {
    pub fn new(b2b: bool, combo: usize) -> Self {
        Self {b2b, combo}
    }

    pub fn handle_zone<'a>(&mut self, event: &'a SpinEvent<'a>)
            -> Option<ChainEvent<'a>> {
        match event {
            SpinEvent::LineClear(spin_clear) => {
                if spin_clear.clear.lines > 0 {
                    self.combo += 1;
                    let b2b = self.b2b;
                    self.b2b = spin_clear.hard;
                    Some(ChainEvent::LineClear(ChainClear {
                        clear: spin_clear,
                        b2b: b2b && self.b2b,
                        combo: self.combo,
                    }))
                } else {
                    self.combo = 0;
                    Some(ChainEvent::LineClear(ChainClear {
                        clear: spin_clear,
                        b2b: false,
                        combo: self.combo,
                    }))
                }
                
            },
            &SpinEvent::ZoneClear(lines) => {
                self.b2b = lines >= 4;
                if lines > 0 {
                    self.combo += 1;
                } else {
                    self.combo = 0;
                }
                Some(ChainEvent::ZoneClear(lines))
            }
        }
    }

    pub fn handle_no_zone<'a>(&mut self, event: &'a SpinEvent<'a>)
            -> Option<ChainEvent<'a>> {
        match event {
            SpinEvent::LineClear(_) => self.handle_zone(event),
            SpinEvent::ZoneClear(lines) => Some(ChainEvent::ZoneClear(*lines)),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum ChainConditions {
    Condition(ScoreTarget, ChainScorer),
}

impl ChainConditions {
    pub fn handle_event(&mut self, event: &ChainEvent) {
        match self {
            Self::Condition(target, scorer) =>
                target.score += scorer.score_event(event),
        }
    }

    pub fn statuses(&self) -> Vec<bool> {
        match self {
            Self::Condition(target, _) => vec![target.score >= target.target],
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum ChainScorer {
    // Count occurrences, increasing by only one
    LineClear {
        req_lines: ReqOrMin,
        req_piece: Option<PieceType>,
        req_all_clear: AllClearType,
        req_spin: Option<Option<SpinType>>,
        req_hard: Option<bool>,
        req_b2b: Option<bool>,
        req_combo: ReqOrMin,
        negate: bool,
    },
    ZoneClear {
        req_lines: ReqOrMin,
    },
    // Count totals, sometimes increasing by more than one
    LinesCleared,
    DamageDealt {
        count_zone_damage: bool,
    },
    JeapordyDealt,
}

impl ChainScorer {
    fn score_event(&self, event: &ChainEvent) -> usize {
        match self {
            Self::LineClear{req_lines, req_piece, req_all_clear,
                req_spin, req_hard, req_b2b, req_combo, negate} =>
                if let ChainEvent::LineClear(chain_clear) = event {
                    let spin_clear = chain_clear.clear;
                    let clear = spin_clear.clear;
                    let is_clear = clear.lines > 0;
                    let reqs_met = req_lines.matches(clear.lines)
                        && req_piece.map_or(true,
                        |r| clear.active.get_type() == r)
                        && AllClearType::from_line_clear(&clear)
                            .fits_req(&req_all_clear)
                        && req_spin.as_ref().map_or(true,
                        |r| spin_clear.spin.as_ref().map_or(false,
                            |t| r.as_ref().map_or(false, |r| r == t)))
                        && req_hard.map_or(true,
                        |r| spin_clear.hard == r)
                        && req_b2b.map_or(true,
                        |r| chain_clear.b2b == r)
                        && req_combo.matches(chain_clear.combo);
                    if match negate {
                        false => reqs_met,
                        true => !reqs_met && is_clear,
                    } {
                        return 1;
                    }
                }
            Self::ZoneClear {req_lines} =>
                if let ChainEvent::ZoneClear(lines) = event {
                    if req_lines.matches(*lines) {
                        return 1;
                    }
                }
            Self::LinesCleared =>
                if let ChainEvent::LineClear(clear) = event {
                    return clear.clear.clear.lines;
                }
            Self::DamageDealt {count_zone_damage}=> {
                match event {
                    ChainEvent::LineClear(chain_clear) => {
                        let spin_clear = chain_clear.clear;
                        let clear = spin_clear.clear;
                        if clear.lines == 0 {
                            return 0;
                        }
                        if AllClearType::from_line_clear(&clear) == AllClearType::ALL_CLEAR {
                            return 10;
                        }
                        let mut damage = match spin_clear.spin {
                            Some(SpinType::Full) => clear.lines * 2,
                            _ => match clear.lines {
                                1 => 0,
                                2 => 1,
                                3 => 2,
                                _ => 4,
                            }
                        };
                        if chain_clear.b2b {
                            damage += 1;
                        };
                        let combo_damage = if clear.in_zone && *count_zone_damage {
                            match chain_clear.combo {
                                0..=2 => 0,
                                3..=4 => 1,
                                5..=10 => 2,
                                _ => 3,
                            }
                        } else {
                            match chain_clear.combo {
                                0..=2 => 0,
                                3..=4 => 1,
                                5..=7 => 2,
                                8..=13 => 3,
                                _ => 4,
                            }
                        };
                        return damage + combo_damage;
                    }
                    ChainEvent::ZoneClear(lines) =>
                        if *count_zone_damage {
                            return match lines {
                                20 => 17 + 17,
                                21 => 18 + 18 + 10,
                                22 => 20 + 20 + 12 + 10,
                                23 => 20 + 20 + 20 + 20,
                                _ => lines + lines / 2,
                            }
                        }
                }
            }
            Self::JeapordyDealt => {
                if let ChainEvent::LineClear(chain_clear) = event {
                    let spin_clear = chain_clear.clear;
                    let clear = spin_clear.clear;
                    if clear.lines == 0 {
                        return 0;
                    }
                    if AllClearType::from_line_clear(&clear) == AllClearType::ALL_CLEAR {
                        return 8;
                    }
                    let mut damage = match spin_clear.spin {
                        Some(SpinType::Full) => clear.lines * 2,
                        Some(SpinType::Mini) => clear.lines * 2 - 1,
                        _ => match clear.lines {
                            1 => 0,
                            2 => 1,
                            3 => 2,
                            _ => 4,
                        }
                    };
                    if chain_clear.b2b {
                        damage += 1;
                    };
                    let combo_damage = match chain_clear.combo {
                        0..=2 => 0,
                        3..=4 => 1,
                        5 => 2,
                        6 => 3,
                        7 => 4,
                        8 => 3,
                        _ => 2,
                    };
                    return damage.max(combo_damage);
                }
            }
        }
        0
    }
}
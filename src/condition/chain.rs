use crate::condition::all_clear::AllClearType;
use crate::condition::event::ScoreTarget;
use crate::condition::spin::SpinClear;
use crate::condition::spin::SpinEvent;
use crate::condition::spin::SpinType;
use crate::game::PieceType;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ChainClear<'a> {
    pub clear: &'a SpinClear<'a>,
    pub hard: bool,
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
    b2b: bool,
    combo: usize,
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
                    let hard = spin_clear.clear.lines >= 4 || spin_clear.spin != None;
                    let b2b = self.b2b;
                    self.b2b = hard;
                    Some(ChainEvent::LineClear(ChainClear {
                        clear: spin_clear,
                        hard,
                        b2b,
                        combo: self.combo,
                    }))
                } else {
                    self.combo = 0;
                    Some(ChainEvent::LineClear(ChainClear {
                        clear: spin_clear,
                        hard: false,
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
        required_lines: Option<usize>,
        required_piece: Option<PieceType>,
        required_all_clear: AllClearType,
        required_spin: Option<Option<SpinType>>,
        required_hard: Option<bool>,
        required_b2b: Option<bool>,
        required_combo: Option<usize>,
        negate: bool,
    },
    ZoneClear {
        min_lines: usize,
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
            Self::LineClear{required_lines, required_piece, required_all_clear,
                required_spin, required_hard, required_b2b, required_combo,
                negate} => if let ChainEvent::LineClear(chain_clear) = event {
                    let spin_clear = chain_clear.clear;
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
                            |t| r.as_ref().map_or(false, |r| r == t)))
                        && required_hard.map_or(true,
                        |r| chain_clear.hard == r)
                        && required_b2b.map_or(true,
                        |r| chain_clear.b2b == r)
                        && required_combo.map_or(true,
                        |r| chain_clear.combo == r);
                    if match negate {
                        false => reqs_met,
                        true => !reqs_met && is_clear,
                    } {
                        return 1;
                    }
                }
            Self::ZoneClear {min_lines} =>
                if let ChainEvent::ZoneClear(lines) = event {
                    if lines >= min_lines {
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
                                0..=1 => 0,
                                2..=3 => 1,
                                4..=9 => 2,
                                _ => 3,
                            }
                        } else {
                            match chain_clear.combo {
                                0..=1 => 0,
                                2..=3 => 1,
                                4..=6 => 2,
                                7..=12 => 3,
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
                        0..=1 => 0,
                        2..=3 => 1,
                        4 => 2,
                        5 => 3,
                        6 => 4,
                        7 => 3,
                        _ => 2,
                    };
                    return damage.max(combo_damage);
                }
            }
        }
        0
    }
}
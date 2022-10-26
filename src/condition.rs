use crate::game::PieceType;

pub struct Condition {
    num_left: u32,
    typ: CounterType,
}

pub enum CounterType {
    // Count occurrences, increasing by only one
    LineClear {
        required_lines: Option<usize>,
    },
    TSpinClear {
        required_lines: Option<usize>,
        required_mini: Option<bool>,
        last_kick: usize,
    },
    AllSpinClear {
        required_type: Option<PieceType>,
        required_lines: Option<usize>,
        required_mini: Option<bool>,
        last_kick: usize,
    },
    PerfectClear {
        required_lines: Option<usize>,
    },
    ColorClear {
        required_lines: Option<usize>,
    },
    GarbageClear,
    ZoneClear {
        min_lines: usize,
    },
    // Count totals, sometimes increasing by more than one
    LinesCleared,
    ZoneLinesCleared,
    DamageDealt {
        b2b: bool,
        combo: usize,
        last_kick: usize,
    },
    JeapordyDealt {
        b2b: bool,
        combo: usize,
        last_kick: usize,
    },
}
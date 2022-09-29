use enum_map::Enum;
use enumset::EnumSetType;
use component::button::ButtonViewable;

use serde::Serialize;
use serde::Deserialize;

#[derive(EnumSetType, Enum, Serialize, Deserialize, Debug)]
pub enum Action {
	Left,
	Right,
	DownSlow,
	DownFast,
	Clockwise,
	Anticlockwise,
	Flip,
	Place,
	Hold,
	Zone,
	Undo,
	Redo,
    RerollCurrent,
    RerollNext1,
    RerollNext2,
    RerollNext3,
    RerollNext4,
	Restart,
}

impl ButtonViewable for Action {
	fn get_name(&self) -> String {
        match self {
            Action::Left => "Move Left",
            Action::Right => "Move Right",
            Action::DownSlow => "Move Down",
            Action::DownFast => "Instant Drop",
            Action::Clockwise => "Rotate CW",
            Action::Anticlockwise => "Rotate ACW",
            Action::Flip => "Rotate 180",
            Action::Place => "Place Piece",
            Action::Hold => "Hold Piece",
            Action::Zone => "Toggle Zone",
            Action::Undo => "Undo",
            Action::Redo => "Redo",
            Action::RerollCurrent => "Reroll Current",
            Action::RerollNext1 => "Reroll Next #1",
            Action::RerollNext2 => "Reroll Next #2",
            Action::RerollNext3 => "Reroll Next #3",
            Action::RerollNext4 => "Reroll Next #4",
            Action::Restart => "Restart",
        }.to_string()
    }

    fn get_icon_url(&self) -> Option<String> {
        match self {
            Action::Left => Some("assets/left.png"),
            Action::Right => Some("assets/right.png"),
            Action::DownSlow => Some("assets/down.png"),
            Action::DownFast => Some("assets/instant.png"),
            Action::Clockwise => Some("assets/cw.png"),
            Action::Anticlockwise => Some("assets/anticw.png"),
            Action::Flip => Some("assets/180.png"),
            Action::Place => Some("assets/place.png"),
            Action::Hold => Some("assets/hold.png"),
            Action::Zone => Some("assets/zone.png"),
            Action::Undo => Some("assets/undo.png"),
            Action::Redo => Some("assets/redo.png"),
            Action::RerollCurrent => Some("assets/reroll.png"),
            Action::Restart => Some("assets/restart.png"),
            _ => None,
        }.and_then(|s| Some(s.to_string()))
    }
}

#[derive(Copy, Clone)]
pub enum MetaAction {
	Action(ztrix::game::Action),
    Undo,
    Redo,
    Reroll(usize),
	Restart,
}
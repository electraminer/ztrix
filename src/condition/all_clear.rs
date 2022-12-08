use crate::game::Mino;
use crate::game::game::LineClear;
use crate::serialize::{SerializeUrlSafe, DeserializeError};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct AllClearType {
    pub is_gray_clear: bool,
    pub is_color_clear: bool,
}

impl AllClearType {
    pub const NONE: Self = Self{ is_gray_clear: false, is_color_clear: false};
    pub const GRAY_CLEAR: Self = Self{ is_gray_clear: true, is_color_clear: false};
    pub const COLOR_CLEAR: Self = Self{ is_gray_clear: false, is_color_clear: true};
    pub const ALL_CLEAR: Self = Self{ is_gray_clear: true, is_color_clear: true};

    pub fn from_line_clear(clear: &LineClear) -> Self {
        let active = clear.active.clone();
        let mut board = clear.board.clone();
        active.place(&mut board);
        board.clear_lines();
        Self {
            is_gray_clear: board.matrix.iter().all(|r| r.iter().all(
                |m| !matches! {m, Some(Mino::Gray)})),
            is_color_clear: board.matrix.iter().all(|r| r.iter().all(
                |m| !matches! {m, Some(Mino::Piece(_))})),
        }
    }

    pub fn fits_req(&self, req: &AllClearType) -> bool {
        (!req.is_gray_clear || self.is_gray_clear)
        && (!req.is_color_clear || self.is_color_clear)
    }
}

impl SerializeUrlSafe for AllClearType {
	fn serialize(&self) -> String {
		match self {
			&Self::NONE => "_",
			&Self::GRAY_CLEAR => "G",
			&Self::COLOR_CLEAR => "C",
			&Self::ALL_CLEAR => "A",
		}.to_owned()
	}

	fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
		Ok(match input.next()? {
			'_' => Self::NONE,
			'C' => Self::GRAY_CLEAR,
			'G' => Self::COLOR_CLEAR,
			'A' => Self::ALL_CLEAR,
			_ => return Err(DeserializeError::new("AllClearType should be represented by _, C, G, or A")),
		})
	}
}
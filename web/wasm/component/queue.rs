use ztrix::game::PieceType;
use component::piece_box;
use component::piece_box::PieceBoxSubcomponent;
use ztrix::game::Queue;

use component::subcomponent::Subcomponent;
use component::button;
use component::game_interface::GameInterface;

use yew::prelude::*;

fn get_piece_char(piece: PieceType) -> char {
	match piece {
		PieceType::S => 'S',
		PieceType::Z => 'Z',
		PieceType::J => 'J',
		PieceType::T => 'T',
		PieceType::L => 'L',
		PieceType::O => 'O',
		PieceType::I => 'I',
	}
}

pub struct QueueSubcomponent {
	boxes: [PieceBoxSubcomponent; 4]
}

impl Subcomponent for QueueSubcomponent {
	type Component = GameInterface;
	type Properties<'a> = &'a Queue;

	fn new() -> QueueSubcomponent {
		QueueSubcomponent{
			boxes: [0; 4].map(|_| PieceBoxSubcomponent::new()),
		}
	}

	fn view<'a>(&self, ctx: &Context<Self::Component>,
			queue: &'a Queue) -> Html {
		let bag: String = queue.get_rando().options()
			.map(|p| get_piece_char(p))
			.collect();
		let indicator = (bag.len() + 4) % 7;
		let func = |i| if indicator == i {
			None
		} else {
			Some("visibility: hidden")
		};
		html! {
			<div class="queue">
				{
					self.boxes.iter().enumerate().map(|(i, b)|
						html! {<>
							<hr class="indicator"
								style={func(i)}/>
							{button::view_button_custom(ctx,
								format! { "RerollNext{}", i+1},
								b.view(ctx, piece_box::Props{
									piece: Some(queue.get(i)),
									grayed: false,
							}))}
						</>})
						.collect::<Html>()
				}
				<hr class="indicator"
					style={func(4)}/>
        		<p><strong>{"BAG"}</strong></p>
        		<p>{bag}</p>
				<hr class="indicator"
					style={func(5)}/>
			</div>
		}
	}

	fn rendered<'a>(&self, ctx: &Context<Self::Component>,
			queue: &'a Queue, first: bool) {
		for (i, b) in self.boxes.iter().enumerate() {
			b.rendered(ctx, piece_box::Props{
				piece: Some(queue.get(i)),
				grayed: false,
			}, first);
		}
    }
}
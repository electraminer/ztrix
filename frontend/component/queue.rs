use controller::input_handler::ButtonEvent;
use crate::component::button::ButtonComponent;
use crate::component::piece_box::PieceBoxComponent;

use ztrix::game::PieceType;
use ztrix::game::Queue;

use yew::prelude::*;

#[derive(Copy, Clone)]
pub enum QueueButton {
	NextBox(usize),
	BagInfo,
}

#[derive(Properties, PartialEq)]
pub struct Props {
	pub queue: Queue,
	#[prop_or_default]
	pub num_speculative: usize,

	#[prop_or_default]
	pub onbutton: Callback<ButtonEvent<QueueButton>>
}

#[function_component(QueueComponent)]
pub fn queue(props: &Props) -> Html {
	let bag_str: String = props.queue.rando.options()
		.map(|p| match p {
			PieceType::I => 'I',
			PieceType::O => 'O',
			PieceType::J => 'J',
			PieceType::L => 'L',
			PieceType::S => 'S',
			PieceType::Z => 'Z',
			PieceType::T => 'T',
		}).collect();
	let bag_pos = (bag_str.len() + 4) % 7;
	let bag_indicator = |i: usize| html! {
		<hr class={classes!(
			"spacer",
			(bag_pos == i).then_some("bag-pos"),
		)}/>
	};
	let onbutton = |b: QueueButton|
		props.onbutton.reform(move |e: ButtonEvent<()>|
			e.map(|_| b));

	html! {
		<div class="queue">
			{for (0..4).map(|i|
				html! { <>
					{bag_indicator(i)}
					<PieceBoxComponent
						piece={Some(props.queue.get(i))}
						speculative=
							{props.num_speculative+i >= 4}
						onbutton={onbutton(
							QueueButton::NextBox(i))}/>
				</> }
			)}
			{bag_indicator(4)}
			<ButtonComponent onbutton={onbutton(
				QueueButton::BagInfo)}>
        		<p><strong>{"BAG"}</strong></p>
        		<p class="bag-text">{bag_str}</p>
        	</ButtonComponent>
			{bag_indicator(5)}
		</div>
	}
}
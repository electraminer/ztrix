use controller::input_handler::ButtonEvent;
use crate::component::button::ButtonComponent;
use crate::component::piece_box::PieceBoxComponent;

use ztrix::game::Queue;

use yew::prelude::*;

#[derive(Copy, Clone)]
pub enum QueueButton {
	NextBox(usize),
	NextText,
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
		.map(|p| format!{"{}", p}).collect();
	let fill = props.queue.fill();
	let length = props.queue.length;
	let num_fixed = if props.num_speculative < length {
		length - props.num_speculative
	} else {
		0
	};
	let mut upcoming: String = (length..fill.clamp(
			length, length + 14))
		.map(|i| props.queue.get(i))
		.map(|p| format!{"{}", p}).collect();
	if upcoming == "" {
		upcoming = "None".to_string();
	}
	if fill > length + 14 {
		upcoming.push_str("...");
	}
	let bag_pos = (bag_str.len() + fill) % 7;
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
			{for (0..fill.clamp(0, length)).map(|i|
				html! { <>
					{bag_indicator(i)}
					<PieceBoxComponent
						piece={Some(props.queue.get(i))}
						speculative={i >= num_fixed}
						onbutton={onbutton(
							QueueButton::NextBox(i))}/>
				</> }
			)}
			{bag_indicator(fill.clamp(0, length))}
			<div class="next-button">
				<ButtonComponent onbutton={onbutton(
					QueueButton::NextText)}>
	        		<p><strong>{"NEXT"}</strong></p>
	        		<p class="next-text">{
	        			format!{"{}", upcoming}}
	        			</p>
	        	</ButtonComponent>
			</div>
			<div class="bag-button">
				<ButtonComponent onbutton={onbutton(
					QueueButton::BagInfo)}>
	        		<p><strong>{"BAG"}</strong></p>
	        		<p class="bag-text">{bag_str}</p>
	        	</ButtonComponent>
				{bag_indicator(fill.clamp(0, length)+1)}
			</div>
		</div>
	}
}

use controller::input_handler::ButtonEvent;
use component::config_interface::BINDABLE_PLAY;
use component::play_interface::PlayButton;

use component::button::ButtonComponent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub name: String,
	pub bound: Vec<PlayButton>,

	#[prop_or_default]
	pub onchange: Callback<Vec<PlayButton>>,
}

#[function_component(ButtonBinding)]
pub fn button_binding(props: &Props) -> Html {
	let onchange = props.onchange.clone();
	let bound = props.bound.clone();
	html! {
		<div class="binding-row">
			<div class="thin-row">
				<h3>{props.name.clone()}</h3>
				<div class="add-binding">
					<button>
						<img src="/assets/add.png" alt="+"/>
						<div class="dropdown">
							{for BINDABLE_PLAY.iter().map(|b| {
								let button = b.clone();
								let onchange = onchange.clone();
								let bound = bound.clone();
								let callback = Callback::from(
									move |_| {
									    let mut vec = bound.clone();
										vec.push(button);
										onchange.emit(vec);
									});
								html! {
									<button onmousedown={callback}>
										{b.get_name()}
									</button>
								}
							})}
						</div>
					</button>
				</div>
				<div class="remove-binding">
					<ButtonComponent
						onbutton={Callback::from(
							move |e: ButtonEvent<()>|
								if let ButtonEvent::Press(_) = e {
									let mut vec = bound.clone();
									vec.pop();
									onchange.emit(vec);
								})}>
						<img src="/assets/remove.png" alt="-"/>
					</ButtonComponent>
				</div>
			</div>
			{if props.bound.len() > 0 {
				html! {	
					<div class="row">
						{for props.bound.iter().map(|b|
							b.view_button(Callback::noop()))}
					</div>
				}
			} else {
				html! {}

			}}
		</div>
	}
}
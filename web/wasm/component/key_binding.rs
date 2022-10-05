use controller::input_handler::ButtonEvent;
use component::button::ButtonComponent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub name: String,
	pub bound: Vec<String>,

	#[prop_or_default]
	pub onbind: Callback<String>,
	#[prop_or_default]
	pub onunbind: Callback<String>,
}

#[function_component(KeyBinding)]
pub fn key_binding(props: &Props) -> Html {
	let bound = props.bound.clone();
	let onunbind = props.onunbind.clone();
	html! {
		<div class="thin-row">
			<div class="binding-name">
				<p><strong>{props.name.clone()}</strong></p>
			</div>
			<div class="binding-list">
				{for bound.iter().map(|c| html! {
					<p>{c}</p>
				})}
			</div>
			<div class="add-binding">
				<button
					onkeydown={props.onbind.reform(
						|e: KeyboardEvent| e.code())}>
					<img src="/assets/add.png" alt="+"/>
				</button>
			</div>
			<div class="remove-binding">
				<ButtonComponent
					onbutton={Callback::from(
						move |e: ButtonEvent<()>|
							if let ButtonEvent::Press(_) = e {
								if let Some(c) = bound.last() {
									onunbind.emit(c.to_string());
								}
							})}>
					<img src="/assets/remove.png" alt="-"/>
				</ButtonComponent>
			</div>
		</div>
	}
}
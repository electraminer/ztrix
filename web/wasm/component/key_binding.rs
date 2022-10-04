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
		<div class="row">
			<p><strong>{props.name.clone()}</strong></p>
			{for props.bound.iter().map(|c| html! {
				<p>{c}</p>
			})}
			<button
				onkeydown={props.onbind.reform(
					|e: KeyboardEvent| e.code())}>
				{"+"}
			</button>
			<ButtonComponent
				onbutton={Callback::from(
					move |e: ButtonEvent<()>|
						if let ButtonEvent::Press(_) = e {
							if let Some(c) = bound.last() {
								onunbind.emit(c.to_string());
							}
						})}>
				{"-"}
			</ButtonComponent>
		</div>
	}
}
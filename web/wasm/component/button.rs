use controller::input_handler::ButtonEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	#[prop_or_default]
	pub onbutton: Callback<ButtonEvent<()>>,

	#[prop_or_default]
	pub children: Children,
}

#[function_component(ButtonComponent)]
pub fn button_component(props: &Props) -> Html {
	let state_pressed = use_mut_ref(|| false);

	let pressed = state_pressed.clone();
	let onbutton = props.onbutton.clone();
	let onpress = Callback::from(move |_|
		if !*pressed.borrow() {
			*pressed.borrow_mut() = true;
			onbutton.emit(ButtonEvent::Press(()));
		});

	let pressed = state_pressed.clone();
	let onbutton = props.onbutton.clone();
	let onrelease = Callback::from(move |_|
		if *pressed.borrow() {
			*pressed.borrow_mut() = false;
			onbutton.emit(ButtonEvent::Release(()));
		});

    html! {
        <button
        	onmousedown={onpress.reform(|_| ())}
        	onmouseup={onrelease.reform(|_| ())}
        	onmouseleave={onrelease.reform(|_| ())}
        	ontouchstart={onpress.reform(|e: TouchEvent|
    			e.prevent_default())}
        	ontouchend={onrelease.reform(|e: TouchEvent|
    			e.prevent_default())}>
        	{for props.children.iter()}
        </button>
    }
}
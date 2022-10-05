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
	let state_pressed = use_state(|| false);

	let pressed = state_pressed.clone();
	let onbutton = props.onbutton.clone();
	let onpress = Callback::from(move |_|
		if !*pressed {
			pressed.set(true);
			onbutton.emit(ButtonEvent::Press(()));
		});
	let onpress_clone = onpress.clone();
	let onmousedown = Callback::from(move |e: MouseEvent|
		if e.button() == 0 {
			onpress_clone.emit(());
		});

	let pressed = state_pressed.clone();
	let onbutton = props.onbutton.clone();
	let onrelease = Callback::from(move |_|
		if *pressed {
			pressed.set(false);
			onbutton.emit(ButtonEvent::Release(()));
		});
	let onrelease_clone = onrelease.clone();
	let onmouseup = Callback::from(move |e: MouseEvent|
		if e.button() == 0 {
			onrelease_clone.emit(());
		});
    html! {
        <button class={classes!(
        	(*state_pressed).then_some("active"),
        )}
        	{onmousedown} {onmouseup}
        	onmouseleave={onrelease.reform(|_| ())}
        	ontouchstart={onpress.reform(|e: TouchEvent|
    			e.prevent_default())}
        	ontouchend={onrelease.reform(|e: TouchEvent|
    			e.prevent_default())}>
        	{for props.children.iter()}
        </button>
    }
}
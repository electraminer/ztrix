
use enum_map::Enum;

use controller::input_handler::InputEvent;
use component::game_interface::GameInterface;
use yew::prelude::*;

pub trait ButtonViewable {
	fn get_name(&self) -> String;

	fn get_icon_url(&self) -> Option<String>;
}

pub fn view_button_custom(ctx: &Context<GameInterface>,
		code: String, contents: Html) -> Html {
	let code1 = code.clone();
	let code2 = code.clone();
    html! {
        <button tabindex="-1" value={code.clone()}
        	ontouchstart={ctx.link().callback(move |_|
        		InputEvent::BtnTouchDown(code.clone()))}
        	ontouchend={ctx.link().callback(move |_|
        		InputEvent::BtnTouchUp(code1.clone()))}
        	onmousedown={ctx.link().callback(move |_|
        		InputEvent::BtnClick(code2.clone()))}>
        	{contents}
        </button>
    }
}

pub fn view_button<A>(ctx: &Context<GameInterface>,
		code: String, action: &A) -> Html
where	A: Enum + ButtonViewable {
	let contents = if let Some(url) = action.get_icon_url() {
		html! { <img src={url}
			alt={action.get_name()}/> }
	} else {
		html! { <p>{action.get_name()}</p> }
	};
	view_button_custom(ctx, code, contents)
}

pub fn view_button_list<A>(ctx: &Context<GameInterface>,
		code: String, actions: &Vec<A>) -> Html
where	A: Enum + ButtonViewable {
	html! {
		<div class="button-list">{
			actions.iter().enumerate()
				.map(|(i, a)| view_button(ctx,
					format!("{}[{}]", code, i), a))
				.collect::<Html>()
		}</div>
	}
}

pub fn view_button_grid<A>(ctx: &Context<GameInterface>,
		code: String, actions: &Vec<Vec<A>>) -> Html
where	A: Enum + ButtonViewable {
	html! {
		<div class="button-grid">{
			actions.iter().enumerate()
				.map(|(i, a)| view_button_list(ctx,
					format!("{}[{}]", code, i), a))
				.collect::<Html>()
		}</div>
	}
}
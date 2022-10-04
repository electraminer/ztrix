
use component::key_binding::KeyBinding;
use user_prefs::UserPrefs;
use component::play_interface::PlayButton;
use component::edit_interface::EditButton;
use controller::input_bindings::InputBindings;


use controller::input_handler::ButtonEvent;


use yew::prelude::*;

pub enum AnyButton {
	PlayButton(PlayButton),
	EditButton(EditButton),
}

pub enum Msg {
	KeyButton(ButtonEvent<String>),
	Bind(AnyButton, String),
	Unbind(AnyButton, String),
}

pub struct BindingInterface {
	bindings: InputBindings,
}

impl Component for BindingInterface {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		let user_prefs = UserPrefs::get();
		Self {
			bindings: user_prefs.get_input_bindings().clone(),
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let bindable_play = vec![
			PlayButton::Left, PlayButton::Right
		];
		html! {
			<div class="scrollable">
				{for bindable_play.iter().map(|b| html! {
					<KeyBinding
						name={b.get_name()}
						bound={vec![]}/>
				})}
			</div>
		}
	}

	fn update(&mut self, _ctx: &Context<Self>,
			msg: Self::Message) -> bool {
		match msg {
			_ => (),
		}
		true
	}
}
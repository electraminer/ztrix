
use component::key_binding::KeyBinding;
use user_prefs::UserPrefs;
use component::play_interface::PlayButton;
use component::edit_interface::EditButton;
use controller::input_bindings::KeyBindings;





use yew::prelude::*;

pub enum AnyButton {
	PlayButton(PlayButton),
	EditButton(EditButton),
}

pub enum Msg {
	Bind(AnyButton, String),
	Unbind(AnyButton, String),
}

pub struct BindingInterface {
	key_bindings: KeyBindings,
}

const BINDABLE_PLAY: [PlayButton; 19] = [
	PlayButton::Left, PlayButton::Right,
	PlayButton::DownSlow, PlayButton::DownFast,
	PlayButton::Clockwise, PlayButton::Anticlockwise,
	PlayButton::Flip, PlayButton::Place,
	PlayButton::Hold, PlayButton::Zone,
	PlayButton::Undo, PlayButton::Redo,
	PlayButton::RerollCurrent,
	PlayButton::RerollNext(1), PlayButton::RerollNext(2),
	PlayButton::RerollNext(3), PlayButton::RerollNext(4),
	PlayButton::Restart, PlayButton::Edit,
];

const BINDABLE_EDIT: [EditButton; 14] = [
	EditButton::SetHold, EditButton::SetCurrent,
	EditButton::SetNext(1), EditButton::SetNext(2),
	EditButton::SetNext(3), EditButton::SetNext(4),
	EditButton::SetBagPos, EditButton::ToggleZone,
	EditButton::ToggleHoldUsed, EditButton::Play,
	EditButton::Import, EditButton::Export,
	EditButton::Revert, EditButton::EraseAll,
];

impl Component for BindingInterface {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		let user_prefs = UserPrefs::get();
		Self {
			key_bindings: user_prefs.key_bindings.clone(),
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		html! {
			<div class="interface">
				<div class="row">
					<button>{"Back"}</button>
					<h1>{"Settings"}</h1>
					<button>{"Apply"}</button>
				</div>
				<div class="row">
					<button>{"Keybinds"}</button>
					<button>{"Buttons"}</button>
					<button>{"Handling"}</button>
				</div>
				<div class="scrollable">
					<div class="row">
						<h2>{"Play Controls"}</h2>
					</div>
					{for BINDABLE_PLAY.iter().map(|b| html! {
						<KeyBinding
							name={b.get_name()}
							bound={self.key_bindings.play_bindings
								.iter().filter(|(_, u)| *u == b)
								.map(|(c, _)| c.to_string())
								.collect::<Vec<String>>()}
							onbind={ctx.link().callback(
								move |s: String| Msg::Bind(
									AnyButton::PlayButton(*b), s))}
							onunbind={ctx.link().callback(
								move |s: String| Msg::Unbind(
									AnyButton::PlayButton(*b), s))}/>
					})}
					<div class="row">
						<h2>{"Edit Controls"}</h2>
					</div>
					{for BINDABLE_EDIT.iter().map(|b| html! {
						<KeyBinding
							name={b.get_name()}
							bound={self.key_bindings.edit_bindings
								.iter().filter(|(_, u)| *u == b)
								.map(|(c, _)| c.to_string())
								.collect::<Vec<String>>()}
							onbind={ctx.link().callback(
								move |s: String| Msg::Bind(
									AnyButton::EditButton(*b), s))}
							onunbind={ctx.link().callback(
								move |s: String| Msg::Unbind(
									AnyButton::EditButton(*b), s))}/>
					})}
				</div>
			</div>
		}
	}

	fn update(&mut self, _ctx: &Context<Self>,
			msg: Self::Message) -> bool {
		match msg {
			Msg::Bind(button, code) => match button {
				AnyButton::PlayButton(b) => {
					self.key_bindings.play_bindings
						.insert(code, b);
				}
				AnyButton::EditButton(b) => {
					self.key_bindings.edit_bindings
						.insert(code, b);
				}
			}

			Msg::Unbind(button, code) => match button {
				AnyButton::PlayButton(_) => {
					self.key_bindings.play_bindings
						.remove(&code);
				}
				AnyButton::EditButton(_) => {
					self.key_bindings.edit_bindings
						.remove(&code);
				}
			}
		}
		true
	}
}
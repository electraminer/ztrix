
use ztrix::game::Game;
use yew_router::history::History;
use yew_router::scope_ext::RouterScopeExt;
use controller::action_handler::HandlingSettings;
use controller::input_handler::ButtonEvent;
use component::button::ButtonComponent;
use controller::input_bindings::ButtonBindings;
use component::button_binding::ButtonBinding;
use component::key_binding::KeyBinding;
use user_prefs::UserPrefs;
use component::play_interface::PlayButton;
use component::edit_interface::EditButton;
use controller::input_bindings::KeyBindings;
use component::router::Route;

use yew::prelude::*;

pub enum AnyButton {
	PlayButton(PlayButton),
	EditButton(EditButton),
}

pub enum Msg {
	Bind(AnyButton, String),
	Unbind(AnyButton, String),
	LeftRow(Vec<PlayButton>),
	RightRow(Vec<PlayButton>),
	BottomRow(usize, Vec<PlayButton>),
	AddRow,
	RemoveRow,
	Apply,
	Cancel,
	RevertDefault,
}

#[derive(Properties, PartialEq)]
#[derive(Default)]
pub struct Props {
	#[prop_or_default]
	pub game: Game,
}

pub struct ConfigInterface {
	key_bindings: KeyBindings,
	button_bindings: ButtonBindings,
}

pub const BINDABLE_PLAY: [PlayButton; 19] = [
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

pub const BINDABLE_EDIT: [EditButton; 14] = [
	EditButton::SetHold, EditButton::SetCurrent,
	EditButton::SetNext(1), EditButton::SetNext(2),
	EditButton::SetNext(3), EditButton::SetNext(4),
	EditButton::SetBagPos, EditButton::ToggleZone,
	EditButton::ToggleHoldUsed, EditButton::Play,
	EditButton::Import, EditButton::Export,
	EditButton::Revert, EditButton::EraseAll,
];

impl Component for ConfigInterface {
	type Message = Msg;
	type Properties = Props;

	fn create(_ctx: &Context<Self>) -> Self {
		let user_prefs = UserPrefs::get();
		Self {
			key_bindings: user_prefs.key_bindings.clone(),
			button_bindings: user_prefs.button_bindings.clone(),
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let game = ctx.props().game.clone();
     	let history = ctx.link().history()
     		.expect("should be a history");
		html! {
			<div class="interface">
				<div class="config-row">
					<ButtonComponent
						onbutton={ctx.link().batch_callback(
							move |e: ButtonEvent<()>| match e {
								ButtonEvent::Release(_) =>
									Some(Msg::Cancel),
								_ => None,
							})}>
						<img src="/assets/cancel.png"
		        		alt="Cancel Changes"/>
					</ButtonComponent>
					<h1>{"User Config"}</h1>
					<ButtonComponent
						onbutton={ctx.link().batch_callback(
							move |e: ButtonEvent<()>| match e {
								ButtonEvent::Release(_) =>
									Some(Msg::Apply),
								_ => None,
							})}>
						<img src="/assets/apply.png"
		        		alt="Apply Changes"/>
					</ButtonComponent>
				</div>
				<div class="scrollable">
					<div class="thin-row">
						<h3>{"About Ztrix"}</h3>
						<ButtonComponent
							onbutton={Callback::from(
								move |e: ButtonEvent<()>|
									if let ButtonEvent::Press(_) = e {
										history.push(Route::AboutGame {
											game: game.clone(),
										});
									})}>
							<img src="/assets/help.png" alt="About"/>
						</ButtonComponent>
					</div>
					<hr/>
					<div class="thin-row">
						<h2>{"Side Buttons"}</h2>
					</div>
					<ButtonBinding
						name={"Left Buttons"}
						bound={self.button_bindings
							.left_buttons.clone()}
						onchange={ctx.link().callback(
							move |r: Vec<PlayButton>|
								Msg::LeftRow(r))}/>
					<ButtonBinding
						name={"Right Buttons"}
						bound={self.button_bindings
							.right_buttons.clone()}
						onchange={ctx.link().callback(
							move |r: Vec<PlayButton>|
								Msg::RightRow(r))}/>
					<div class="thin-row">
						<h2>{"Bottom Buttons"}</h2>
					</div>
					{for self.button_bindings.bottom_buttons
						.iter().enumerate().map(|(i, r)| html! {
							<ButtonBinding
								name={{format!{
									"Row {} Buttons", i+1}}}
								bound={r.clone()}
							onchange={ctx.link().callback(
								move |r: Vec<PlayButton>|
									Msg::BottomRow(i, r))}/>
						})}
					<div class="thin-row">
						<h3>{"Add/Remove Rows"}</h3>
						<div class="add-binding">
							<ButtonComponent
								onbutton={ctx.link().batch_callback(
									move |e: ButtonEvent<()>| match e {
										ButtonEvent::Press(_) =>
											Some(Msg::AddRow),
										_ => None,
									})}>
								<img src="/assets/add.png" alt="+"/>
							</ButtonComponent>
						</div>
						<div class="remove-binding">
							<ButtonComponent
								onbutton={ctx.link().batch_callback(
									move |e: ButtonEvent<()>| match e {
										ButtonEvent::Press(_) =>
											Some(Msg::RemoveRow),
										_ => None,
									})}>
								<img src="/assets/remove.png" alt="-"/>
							</ButtonComponent>
						</div>
					</div>
					<hr/>
					<div class="thin-row">
						<h2>{"Play Mode Keybinds"}</h2>
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
					<div class="thin-row">
						<h2>{"Edit Mode Keybinds"}</h2>
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
					<hr/>
					<div class="row">
						<h3>{"Revert to Defaults"}</h3>
						<ButtonComponent
							onbutton={ctx.link().batch_callback(
								move |e: ButtonEvent<()>| match e {
									ButtonEvent::Release(_) =>
										Some(Msg::RevertDefault),
									_ => None,
								})}>
							<img src="/assets/revert.png"
								alt="Revert"/>
						</ButtonComponent>
					</div>
				</div>
			</div>
		}
	}

	fn update(&mut self, ctx: &Context<Self>,
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
			Msg::LeftRow(row) =>
				self.button_bindings.left_buttons = row,
			Msg::RightRow(row) =>
				self.button_bindings.right_buttons = row,
			Msg::BottomRow(i, row) =>
				if i < self.button_bindings.bottom_buttons.len() {
					self.button_bindings.bottom_buttons[i] = row;
				},
			Msg::AddRow =>
				self.button_bindings.bottom_buttons.push(
					Vec::new()),
			Msg::RemoveRow => {
				self.button_bindings.bottom_buttons.pop();
			}
			Msg::Apply => {
				UserPrefs::set(
					self.key_bindings.clone(),
					self.button_bindings.clone(),
					HandlingSettings::default());
				let history = ctx.link().history()
					.expect("should be a history");
				history.push(Route::GameGame {
					game: ctx.props().game.clone()
				})
			}
			Msg::Cancel => {
				let history = ctx.link().history()
					.expect("should be a history");
				history.push(Route::GameGame {
					game: ctx.props().game.clone()
				})
			}
			Msg::RevertDefault => {
				self.key_bindings = KeyBindings::default();
				self.button_bindings = ButtonBindings::default();
			}
		}
		true
	}
}
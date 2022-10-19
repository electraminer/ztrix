
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use instant::Duration;
use component::field_config::FieldConfig;
use ztrix::game::Game;
use yew_router::history::History;
use yew_router::scope_ext::RouterScopeExt;

use controller::input_handler::ButtonEvent;
use controller::action_handler::IrsMode;
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
	LeftRow(Vec<PlayButton>),
	RightRow(Vec<PlayButton>),
	BottomRow(usize, Vec<PlayButton>),
	AddRow,
	RemoveRow,
	BindKey(AnyButton, String),
	UnbindKey(AnyButton, String),
	SetIrsMode(IrsMode),
	SetDas(Duration),
	SetArr(Duration),
	SetDownDas(Duration),
	SetDownArr(Duration),
	SetEntryDelay(Duration),
	SetFreezeDelay(Duration),
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
	user_prefs: UserPrefs,
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
			user_prefs: (*user_prefs).clone(),
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
						<h2>{"Handling Settings"}</h2>
					</div>
					<div class="thin-row">
						<h3>{"IRS/IHS Mode"}</h3>
						<select onchange={ctx.link().batch_callback(
							move |e: Event| match e.target()
								.expect("should be a target")
								.dyn_into::<HtmlSelectElement>()
		    					.expect("element should be an input")
	    						.value().as_str() {
	    							"lenient" => Some(Msg::SetIrsMode(
	    								IrsMode::Lenient)),
	    							"accurate" => Some(Msg::SetIrsMode(
	    								IrsMode::Accurate)),
	    							_ => None,
	    						})}>
							<option value="lenient"
								selected={{matches!{
									self.user_prefs.handling_settings
									.irs_mode, IrsMode::Lenient}}}>
								{"Lenient"}</option>
							<option value="accurate"
								selected={{matches!{
									self.user_prefs.handling_settings
									.irs_mode, IrsMode::Accurate}}}>
								{"Accurate"}</option>
						</select>
					</div>
					<p>{"How Initial Rotation/Hold System behaves.
						Use Accurate if you want to learn/practice
						IRS mechanics identical to TE:C, and Lenient
						if you want smoother controls that allow
						buffering after piece placement."}</p>
					<FieldConfig name="DAS (Auto-Shift Delay)"
						value={{format!{
	      				"{}", self.user_prefs.handling_settings
	      					.das_duration.as_millis()}}}
	      				onchange={ctx.link().batch_callback(
	      					|s: String|
	      						s.parse::<u64>().ok()
	      						.map(|t| Msg::SetDas(
	      							Duration::from_millis(t))))}/>
	      			<p>{"Delay before sideways movement repeats."}</p>
					<FieldConfig name="ARR (Auto-Repeat Rate)"
						value={{format!{
	      				"{}", self.user_prefs.handling_settings
	      					.arr_duration.as_millis()}}}
	      				onchange={ctx.link().batch_callback(
	      					|s: String|
	      						s.parse::<u64>().ok()
	      						.map(|t| Msg::SetArr(
	      							Duration::from_millis(t))))}/>
	      			<p>{"Time between each repeated sideways movement."}</p>
					<FieldConfig name="Downward DAS"
						value={{format!{
	      				"{}", self.user_prefs.handling_settings
	      					.down_das_duration.as_millis()}}}
	      				onchange={ctx.link().batch_callback(
	      					|s: String|
	      						s.parse::<u64>().ok()
	      						.map(|t| Msg::SetDownDas(
	      							Duration::from_millis(t))))}/>
	      			<p>{"Delay before downward movement repeats."}</p>
					<FieldConfig name="Downward ARR"
						value={{format!{
	      				"{}", self.user_prefs.handling_settings
	      					.down_arr_duration.as_millis()}}}
	      				onchange={ctx.link().batch_callback(
	      					|s: String|
	      						s.parse::<u64>().ok()
	      						.map(|t| Msg::SetDownArr(
	      							Duration::from_millis(t))))}/>
	      			<p>{"Time between each repeated downward movement."}</p>
					<FieldConfig name="Entry Delay"
						value={{format!{
	      				"{}", self.user_prefs.handling_settings
	      					.entry_delay.as_millis()}}}
	      				onchange={ctx.link().batch_callback(
	      					|s: String|
	      						s.parse::<u64>().ok()
	      						.map(|t| Msg::SetEntryDelay(
	      							Duration::from_millis(t))))}/>
	      			<p>{"Buffer used to change DAS and IRS as a piece spawns."}</p>
					<FieldConfig name="IRS + IHS Buffer Delay"
						value={{format!{
	      				"{}", self.user_prefs.handling_settings
	      					.buffer_delay.as_millis()}}}
	      				onchange={ctx.link().batch_callback(
	      					|s: String|
	      						s.parse::<u64>().ok()
	      						.map(|t| Msg::SetFreezeDelay(
	      							Duration::from_millis(t))))}/>
	      			<p>{"Buffer used to simultaneously IRS and IHS."}</p>
					<hr/>
					<div class="thin-row">
						<h2>{"Side Buttons"}</h2>
					</div>
					<ButtonBinding
						name={"Left Buttons"}
						bound={self.user_prefs.button_bindings
							.left_buttons.clone()}
						onchange={ctx.link().callback(
							move |r: Vec<PlayButton>|
								Msg::LeftRow(r))}/>
					<ButtonBinding
						name={"Right Buttons"}
						bound={self.user_prefs.button_bindings
							.right_buttons.clone()}
						onchange={ctx.link().callback(
							move |r: Vec<PlayButton>|
								Msg::RightRow(r))}/>
					<div class="thin-row">
						<h2>{"Bottom Buttons"}</h2>
					</div>
					{for self.user_prefs.button_bindings
						.bottom_buttons.iter().enumerate()
						.map(|(i, r)| html! {
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
							bound={self.user_prefs.key_bindings
								.play_bindings.iter()
								.filter(|(_, u)| *u == b)
								.map(|(c, _)| c.to_string())
								.collect::<Vec<String>>()}
							onbind={ctx.link().callback(
								move |s: String| Msg::BindKey(
									AnyButton::PlayButton(*b), s))}
							onunbind={ctx.link().callback(
								move |s: String| Msg::UnbindKey(
									AnyButton::PlayButton(*b), s))}/>
					})}
					<div class="thin-row">
						<h2>{"Edit Mode Keybinds"}</h2>
					</div>
					{for BINDABLE_EDIT.iter().map(|b| html! {
						<KeyBinding
							name={b.get_name()}
							bound={self.user_prefs.key_bindings
								.edit_bindings.iter()
								.filter(|(_, u)| *u == b)
								.map(|(c, _)| c.to_string())
								.collect::<Vec<String>>()}
							onbind={ctx.link().callback(
								move |s: String| Msg::BindKey(
									AnyButton::EditButton(*b), s))}
							onunbind={ctx.link().callback(
								move |s: String| Msg::UnbindKey(
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
			Msg::LeftRow(row) =>
				self.user_prefs.button_bindings.left_buttons = row,
			Msg::RightRow(row) =>
				self.user_prefs.button_bindings.right_buttons = row,
			Msg::BottomRow(i, row) =>
				if i < self.user_prefs.button_bindings.bottom_buttons.len() {
					self.user_prefs.button_bindings.bottom_buttons[i] = row;
				},
			Msg::AddRow =>
				self.user_prefs.button_bindings.bottom_buttons.push(
					Vec::new()),
			Msg::RemoveRow => {
				self.user_prefs.button_bindings.bottom_buttons.pop();
			}
			Msg::BindKey(button, code) => match button {
				AnyButton::PlayButton(b) => {
					self.user_prefs.key_bindings.play_bindings
						.insert(code, b);
				}
				AnyButton::EditButton(b) => {
					self.user_prefs.key_bindings.edit_bindings
						.insert(code, b);
				}
			}

			Msg::UnbindKey(button, code) => match button {
				AnyButton::PlayButton(_) => {
					self.user_prefs.key_bindings.play_bindings
						.remove(&code);
				}
				AnyButton::EditButton(_) => {
					self.user_prefs.key_bindings.edit_bindings
						.remove(&code);
				}
			}
			Msg::SetIrsMode(mode) => self.user_prefs
				.handling_settings.irs_mode = mode,
			Msg::SetDas(duration) => self.user_prefs
				.handling_settings.das_duration = duration,
			Msg::SetArr(duration) => self.user_prefs
				.handling_settings.arr_duration = duration,
			Msg::SetDownDas(duration) => self.user_prefs
				.handling_settings.down_das_duration = duration,
			Msg::SetDownArr(duration) => self.user_prefs
				.handling_settings.down_arr_duration = duration,
			Msg::SetEntryDelay(duration) => self.user_prefs
				.handling_settings.entry_delay = duration,
			Msg::SetFreezeDelay(duration) => self.user_prefs
				.handling_settings.buffer_delay = duration,
			Msg::Apply => {
				UserPrefs::set(self.user_prefs.clone());
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
				self.user_prefs.key_bindings = KeyBindings::default();
				self.user_prefs.button_bindings = ButtonBindings::default();
			}
		}
		true
	}
}
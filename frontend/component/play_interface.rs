
use ztrix::game::game::Clear;
use yew_router::history::History;
use yew_router::scope_ext::RouterScopeExt;
use ztrix::game::MaybeActive;
use component::keyboard_interface::KeyboardInterface;
use component::button::ButtonComponent;
use component::game::GameButton;
use component::queue::QueueButton;
use controller::input_handler::ButtonEvent;

use controller::input_handler::ButtonHandler;
use controller::input_handler::TimeHandler;

use controller::action_handler::MetaAction;
use controller::action_handler::ActionHandler;
use component::router::Route;

use component::game::GameComponent;

use controller::input_handler::InputEvent;

use yew::prelude::*;

use ztrix::game::Game;
use ztrix::game::Mino;
use ztrix::replay::Replay;

use user_prefs::UserPrefs;

use gloo_timers::callback::Interval;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum PlayButton {
	Left,
	Right,
	DownSlow,
	DownFast,
	Clockwise,
	Anticlockwise,
	Flip,
	Place,
	Hold,
	Zone,
	Undo,
	Redo,
    RerollCurrent,
    RerollNext(usize),
	Restart,
	Edit,
}

impl PlayButton {
	pub fn get_name(&self) -> String {
        match self {
            PlayButton::Left => "Move Left",
            PlayButton::Right => "Move Right",
            PlayButton::DownSlow => "Move Down",
            PlayButton::DownFast => "Instant Drop",
            PlayButton::Clockwise => "Rotate CW",
            PlayButton::Anticlockwise => "Rotate ACW",
            PlayButton::Flip => "Rotate 180",
            PlayButton::Place => "Place Piece",
            PlayButton::Hold => "Hold Piece",
            PlayButton::Zone => "Toggle Zone",
            PlayButton::Undo => "Undo",
            PlayButton::Redo => "Redo",
            PlayButton::RerollCurrent => "Reroll Current",
            PlayButton::RerollNext(n) =>
            	return format!{"Reroll Next #{}", n},
            PlayButton::Restart => "Restart",
            PlayButton::Edit => "Enter Edit Mode",
        }.to_string()
    }

    pub fn get_icon_url(&self) -> Option<String> {
        match self {
            PlayButton::Left => Some("/assets/left.png"),
            PlayButton::Right => Some("/assets/right.png"),
            PlayButton::DownSlow => Some("/assets/down.png"),
            PlayButton::DownFast => Some("/assets/instant.png"),
            PlayButton::Clockwise => Some("/assets/cw.png"),
            PlayButton::Anticlockwise => Some("/assets/anticw.png"),
            PlayButton::Flip => Some("/assets/180.png"),
            PlayButton::Place => Some("/assets/place.png"),
            PlayButton::Hold => Some("/assets/hold.png"),
            PlayButton::Zone => Some("/assets/zone.png"),
            PlayButton::Undo => Some("/assets/undo.png"),
            PlayButton::Redo => Some("/assets/redo.png"),
            PlayButton::RerollCurrent => Some("/assets/reroll.png"),
            PlayButton::Restart => Some("/assets/restart.png"),
            _ => None,
        }.and_then(|s| Some(s.to_string()))
    }

    pub fn view_button(self,
    	onbutton: Callback<ButtonEvent<()>>) -> Html {
    	html! {
			<ButtonComponent
				onbutton={onbutton}>
				{match self.get_icon_url() {
					None => html! {
						<p>{self.get_name()}</p>
					},
					Some(src) => html! {
						<img src={src}
							alt={self.get_name()}/>
					},
				}}
			</ButtonComponent>
		}
    }
}

pub enum Msg {
	KeyButton(ButtonEvent<String>),
	GameButton(ButtonEvent<GameButton>),
	Button(ButtonEvent<PlayButton>),
	Interval,
	Config,
}

#[derive(Properties, PartialEq)]
#[derive(Default)]
pub struct Props {
	#[prop_or_default]
	pub game: Game,
}

pub struct PlayInterface {
	replay: Replay,
	last_clear: Option<Clear>,
	button_handler: ButtonHandler<PlayButton>,
	time_handler: TimeHandler,
	action_handler: ActionHandler,
	_interval: Interval,
}

impl Component for PlayInterface {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
		let link = ctx.link().clone();
        Self {
        	replay: Replay::new(ctx.props().game.clone()),
        	last_clear: None,
			button_handler: ButtonHandler::new(),
			time_handler: TimeHandler::new(),
        	action_handler: ActionHandler::new(),
        	_interval: Interval::new(16, move ||
				link.send_message(Msg::Interval))
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
    	let user_prefs = UserPrefs::get();
     	let button_bindings = &user_prefs.button_bindings;
        html! {
        	<KeyboardInterface
        		onkey={ctx.link().callback(
        			|e: ButtonEvent<String>|
        				Msg::KeyButton(e))}>
            	<GameComponent
            		game={self.replay.get_game().clone()}
            		frame={self.replay.get_frame()}
	      			top_left={{ html! {
						<ButtonComponent
							onbutton={ctx.link().batch_callback(
								move |e: ButtonEvent<()>| match e {
									ButtonEvent::Release(_) =>
										Some(Msg::Config),
									_ => None,
								})}>
							<img src="/assets/config.png"
								alt="Config"/>
						</ButtonComponent>
	      				}}}
	      			top_right={{ html! {
		        		<ButtonComponent
		        			onbutton={ctx.link().callback(
								|e: ButtonEvent<()>| Msg::Button(
									e.map(|_| PlayButton::Edit)))}>
							<img src="/assets/edit.png"
			        			alt="Enter Edit Mode"/>
		        		</ButtonComponent>
	      				}}}
		        	bottom_left={{html! { <>
		        		<p><strong>{"FRAME"}</strong></p>
		        		<p><strong>
		        			{self.replay.get_frame()}
		        		</strong></p>
		        		{button_bindings.left_buttons
			            	.iter().map(|b| {
			            		let button = b.clone();
		        				b.view_button(ctx.link().callback(
									move |u: ButtonEvent<()>| Msg::Button(
										u.map(|_| button))))
		    				}).collect::<Html>()}
		            </> }}}
		        	bottom_right={button_bindings.right_buttons
		            	.iter().map(|b| {
			            	let button = b.clone();
	        				b.view_button(ctx.link().callback(
								move |u: ButtonEvent<()>| Msg::Button(
									u.map(|_| button))))
	    				}).collect::<Html>()}
	    			onbutton={ctx.link().callback(
	        			|e: ButtonEvent<GameButton>|
	        				Msg::GameButton(e))}/>
	            {for button_bindings.bottom_buttons
	            	.iter().map(|v| html! {
	            		<div class="row">
	            			{for v.iter().map(|b| {
			            		let button = b.clone();
	            				b.view_button(ctx.link().callback(
									move |u: ButtonEvent<()>| Msg::Button(
										u.map(|_| button))))
            				})}
	     				</div>
	            	})}
            </KeyboardInterface>
        }
    }

       fn update(&mut self, ctx: &Context<Self>,
    		msg: Msg) -> bool {
    	let user_prefs = UserPrefs::get();
    	let key_bindings = &user_prefs.key_bindings;
    	let event = match msg {
    		Msg::KeyButton(event) =>
    			match event.maybe_map(|b|
    				key_bindings.play_bindings.get(&b)
    				.copied())
    				.and_then(|e| self.button_handler
    					.update(e)) {
					Some(event) => InputEvent::Button(event),
					None => return false,
				}
    		Msg::GameButton(event) =>
    			match event.maybe_map(|b| match b {
    				GameButton::Queue(
    					QueueButton::NextBox(n)) =>
    						Some(PlayButton::RerollNext(n+1)),
    				_ => None,
    			}).and_then(|e| self.button_handler
    				.update(e)) {
					Some(event) => InputEvent::Button(event),
					None => return false,
    			}
    		Msg::Button(event) => match self.button_handler
    				.update(event) {
					Some(event) => InputEvent::Button(event),
					None => return false,
    			}
    		Msg::Interval => {
    			InputEvent::PassTime(
    				self.time_handler.update())
    		}
    		Msg::Config => {
				for _ in 0..self.replay.get_frame() {
					self.replay.undo();
				}
				self.replay.revert();
				let history = ctx.link().history()
					.expect("should be a history");
				history.push(Route::ConfigGame {
					game: self.replay.get_game().clone(),
				});
				return false;
    		}
    	};
    	
		let meta_actions = self.action_handler
			.update(&self.replay, event);
		
    	meta_actions.into_iter().map(|e| match e {
    			MetaAction::Action(action) => {
    				let mut clears = self.replay.update(action);
    				if let Some(clear) = clears.pop() {
    					self.last_clear = Some(clear);
    				}
    			}
    			MetaAction::Revert => self.replay.revert(),
    			MetaAction::Undo =>
	    			if self.replay.get_frame() > 0 {
	    				self.replay.undo();
	    			} else {
	    				self.replay.revert();
	    			}
    			MetaAction::Redo => {
    				let mut clears = self.replay.redo();
    				if let Some(clear) = clears.pop() {
    					self.last_clear = Some(clear);
    				}
    			}
    			MetaAction::Reroll(back) =>
    				self.replay.reroll_backward(back),
    			MetaAction::Restart => {
    				for _ in 0..self.replay.get_frame() {
    					self.replay.undo();
    				}
				},
				MetaAction::Edit => {
					self.replay.revert();
					let mut game = self.replay.get_game().clone();
					game.piece = MaybeActive::Inactive(
						game.get_current());
					for row in game.board.matrix.iter_mut() {
						for mino in row.iter_mut() {
							*mino = match mino {
								None => None,
								Some(_) => Some(Mino::Gray),
							}
						}
					}
					let window = web_sys::window()
						.expect("should be a window");
					let url = format!{"/edit/{}", game};
					window.open_with_url_and_target(
						&url, "_blank")
						.expect("should be able to open url");
				}
    		}).count() != 0
    }
}

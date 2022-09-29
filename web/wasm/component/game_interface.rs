
use controller::action::MetaAction;
use controller::input_handler::InputHandler;
use controller::input_handler::InputEvent;
use controller::input_handler::VirtualInputEvent;
use controller::action_handler::ActionHandler;

use component::subcomponent::Subcomponent;
use component::button;
use component::game;
use component::game::GameSubcomponent;

use yew::prelude::*;

use controller::action::Action;

use ztrix::game::Game;
use ztrix::replay::Replay;

use user_prefs::UserPrefs;

use web_sys::UrlSearchParams;

use gloo_timers::callback::Interval;


pub struct GameInterface {
	replay: Replay,
	frozen: bool,
	input_handler: InputHandler<Action>,
	action_handler: ActionHandler,
	game_subcomp: GameSubcomponent,
	_interval: Interval,
}

impl Component for GameInterface {
    type Message = InputEvent;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
    	// get the url parameters
    	let window = web_sys::window()
			.expect("should have a window");
		let location = window.location();
		let search = location.search()
			.expect("location should have a search");
		let _params = UrlSearchParams::new_with_str(
				search.as_str())
			.expect("search should be valid parameters");
		let game_subcomp = GameSubcomponent::new();

		let link = ctx.link().clone();

		// create model
        Self {
        	replay: Replay::new(Game::new()),
        	frozen: true,
        	input_handler: InputHandler::new(),
        	action_handler: ActionHandler::new(),
        	game_subcomp: game_subcomp,
        	_interval: Interval::new(16, move ||
				link.send_message(InputEvent::TimePassed))
        }
    }

    fn update(&mut self, _ctx: &Context<Self>,
    		event: InputEvent) -> bool {
    	web_sys::console::log_1(&format!{"{:?}",event}.into());
    	let user_prefs = UserPrefs::get();
    	let input_bindings = user_prefs.get_input_bindings();
		let virtual_inputs = self.input_handler.update(
			event, input_bindings);
		let meta_actions = virtual_inputs.into_iter()
			.map(|e| {
				if matches! { e,
					VirtualInputEvent::Pressed(_)} {
					self.frozen = false;
				}
				if self.frozen && matches! { e,
					VirtualInputEvent::TimePassed(_)} {
					vec![]
				} else {
					self.action_handler.update(
						self.replay.get_game(), e)
				}})
    		.reduce(|mut l, mut r| {l.append(&mut r); l})
    		.unwrap_or(vec![]);
    	meta_actions.into_iter().map(|e| match e {
    			MetaAction::Action(action) => {
    				self.replay.add_action(action); true},
    			MetaAction::Undo => {
    				self.frozen = true;
    				self.replay.clear_or_undo()},
    			MetaAction::Redo => {
    				self.frozen = true;
    				self.replay.redo()},
    			MetaAction::Reroll(back) => {
    				self.frozen = true;
    				self.replay.reroll_prev_info(back)},
    			MetaAction::Restart => {
    				self.frozen = true;
    				self.replay = Replay::new(Game::new());
    				true},
    		}).fold(false, |l, r| l || r)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
    	let user_prefs = UserPrefs::get();
     	let input_bindings = user_prefs.get_input_bindings();
        html! {
        	<div class="interface"
            	tabindex=1
            	onkeydown={ctx.link().callback( move |e: KeyboardEvent|
            		InputEvent::KeyDown(e.code()))}
            	onkeyup={ctx.link().callback( move |e: KeyboardEvent|
            		InputEvent::KeyUp(e.code()))}
            	onfocusout={ctx.link().callback(move |_|
            		InputEvent::LostFocus)}
            	onfocusin={ctx.link().callback(move |_|
            		InputEvent::GainedFocus)}>
	        	{self.game_subcomp.view(ctx, game::Props{
		    		game: self.replay.get_game(),
		    		frame: self.replay.get_frame_num()
		    	})}
	            {button::view_button_grid(ctx,
	            	"BottomButtons".to_string(),
	            	&input_bindings.get_bottom_buttons())}
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first: bool) {
    	self.game_subcomp.rendered(ctx, game::Props{
    		game: self.replay.get_game(),
    		frame: self.replay.get_frame_num()
    	}, first);
    }
}

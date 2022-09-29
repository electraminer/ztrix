


use user_prefs::UserPrefs;
use wasm_bindgen::JsCast;
use component::subcomponent::Subcomponent;
use component::game_interface::GameInterface;
use ztrix::game::Game;
use web_sys::HtmlElement;

use yew::prelude::*;

use component::button;
use component::piece_box;
use component::piece_box::PieceBoxSubcomponent;

use component::queue::QueueSubcomponent;
use component::board;
use component::board::BoardSubcomponent;


pub struct GameSubcomponent {
	game_div: NodeRef,
	board: BoardSubcomponent,
	queue: QueueSubcomponent,
	hold: PieceBoxSubcomponent,
}

pub struct Props<'a> {
	pub game: &'a Game,
	pub frame: usize,
}

impl Subcomponent for GameSubcomponent {
	type Component = GameInterface;
	type Properties<'a> = Props<'a>;

	fn new() -> GameSubcomponent {
		GameSubcomponent{
			game_div: NodeRef::default(),
			board: BoardSubcomponent::new(),
			queue: QueueSubcomponent::new(),
			hold: PieceBoxSubcomponent::new(),
		}
	}

	fn view(&self, ctx: &Context<Self::Component>,
			props: Props) -> Html {
    	let user_prefs = UserPrefs::get();
     	let input_bindings = user_prefs.get_input_bindings();
		html! {
			<div class="game" ref={self.game_div.clone()}>
	        	<div class="side-column">
		            <button class="top-button"
		           		id="settings"></button>
	        		<p><strong>{"HOLD"}</strong></p>
		        		{self.hold.view(ctx, piece_box::Props{
							piece: props.game.get_hold(),
							grayed: props.game.has_held(),
						})}
	        		<p><strong>{"FRAME"}</strong></p>
	        		<p>{props.frame}</p>
		            {button::view_button_list(ctx,
		            	"LeftButtons".to_string(),
		            	&input_bindings.get_left_buttons())}
	            </div>
	            <div class={classes!(
	            		"board-column",
	            		props.game.is_in_zone()
	            			.then_some("in-zone"),
	            		props.game.is_over()
	            			.then_some("game-over"),
	            	)}>
		            {self.board.view(ctx, board::Props{
			     		board: props.game.get_board(),
			     		piece: props.game.get_piece(),
			     		current: props.game.get_current(),
					})}
				</div>
	        	<div class="side-column">
		            <button class="top-button"
		           		id="edit"></button>
	        		<p><strong>{"NEXT"}</strong></p>
		            {self.queue.view(ctx, props.game.get_queue())}
		            {button::view_button_list(ctx,
		            	"RightButtons".to_string(),
		            	&input_bindings.get_right_buttons())}
	            </div>
	        </div>
		}
	}

	fn rendered(&self, ctx: &Context<Self::Component>,
			props: Props, first: bool) {
     	// get game div
     	let game_div = self.game_div.cast::<HtmlElement>()
        	.expect("should be an html element");
		let width = game_div.client_width() as f64;
		let height = game_div.client_height() as f64;
		let matrix_aspect = 10.0 / 22.5;
		let matrix_width = height * matrix_aspect;
		let sidebar_width = (width - matrix_width) / 2.0;
		let flex = sidebar_width * 10.0 / matrix_width;
		let flex = flex.clamp(3.0, 6.0);
		let flex_str = format!("{}", flex);
		let left_sidebar = game_div.children().item(0)
			.expect("game div should have a left sidebar")
			.dyn_into::<HtmlElement>()
			.expect("should be an html element");
		left_sidebar.style().set_property("flex", &flex_str)
			.expect("style should be modifiable");
		let right_sidebar = game_div.children().item(2)
			.expect("game div should have a right sidebar")
			.dyn_into::<HtmlElement>()
			.expect("should be an html element");
		right_sidebar.style().set_property("flex", &flex_str)
			.expect("style should be modifiable");
     	self.board.rendered(ctx, board::Props{
     		board: props.game.get_board(),
     		piece: props.game.get_piece(),
			current: props.game.get_current(),
     	}, first);
		self.hold.rendered(ctx, piece_box::Props{
			piece: props.game.get_hold(),
			grayed: props.game.has_held(),
		}, first);
    	self.queue.rendered(ctx, props.game.get_queue(), first);
    }
}
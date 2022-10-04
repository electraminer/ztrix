
use controller::input_handler::ButtonEvent;
use component::piece_box::PieceBoxComponent;
use component::queue::QueueComponent;
use component::queue::QueueButton;
use component::board::BoardComponent;
use component::board::BoardMouseEvent;

use ztrix::game::Game;

use yew::prelude::*;

pub enum GameButton {
	Hold,
	Queue(QueueButton),
}

#[derive(Properties, PartialEq)]
pub struct Props {
	#[prop_or_default]
	pub game: Game,
	#[prop_or_default]
	pub top_left: Html,
	#[prop_or_default]
	pub bottom_left: Html,
	#[prop_or_default]
	pub top_right: Html,
	#[prop_or_default]
	pub bottom_right: Html,
	#[prop_or_default]
	pub onboardmouse: Callback<BoardMouseEvent>,
	#[prop_or_default]
	pub onbutton: Callback<ButtonEvent<GameButton>>,
}

#[function_component(GameComponent)]
pub fn game_component(props: &Props) -> Html {
	html! {
		<div class="game">
        	<div class="side-column">
        		<div class="top-left">
        			{props.top_left.clone()}
        		</div>
        		<p><strong>{"HOLD"}</strong></p>
        		<hr class="spacer"/>
        		<PieceBoxComponent
					piece={props.game.hold}
					grayed={props.game.has_held}
					onbutton={props.onbutton.reform(
						|e: ButtonEvent<()>| e.map(|_|
							GameButton::Hold))}/>
				<div class="bottom-left">
        			{props.bottom_left.clone()}
        		</div>
            </div>
            <div class={classes!(
            		"board-column",
            		props.game.in_zone.then_some("in-zone"),
            		props.game.over.then_some("game-over"),
            	)}>
            	<BoardComponent
            		board={props.game.board.clone()}
		     		piece={props.game.piece.clone()}
		     		onmouse={props.onboardmouse.clone()}/>
			</div>
        	<div class="side-column">
        		<div class="top-right">
        			{props.top_right.clone()}
        		</div>
        		<p><strong>{"NEXT"}</strong></p>
        		<QueueComponent
        			queue={props.game.queue.clone()}
					onbutton={props.onbutton.reform(
						|e: ButtonEvent<QueueButton>| e.map(|b|
							GameButton::Queue(b)))}/>
        		<div class="bottom-right">
        			{props.bottom_right.clone()}
        		</div>
            </div>
        </div>
	}
}
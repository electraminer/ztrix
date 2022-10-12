
use ztrix::game::game::Clear;
use controller::input_handler::ButtonEvent;
use component::piece_box::PieceBoxComponent;
use component::queue::QueueComponent;
use component::queue::QueueButton;
use component::board::BoardComponent;
use component::board::BoardMouseEvent;

use ztrix::game::Game;
use ztrix::game::Mino;

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
	pub num_revealed: usize,
	#[prop_or_default]
	pub last_clear: Option<Clear>,
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
        		<div class="middle-left">
	        		<p><strong>{"HOLD"}</strong></p>
	        		<hr class="spacer"/>
	        		<PieceBoxComponent
						piece={props.game.hold}
						grayed={props.game.has_held}
						onbutton={props.onbutton.reform(
							|e: ButtonEvent<()>| e.map(|_|
								GameButton::Hold))}/>
				</div>
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
					{if props.num_revealed
						> props.game.queue.length {
						html! {
							<img class="speculative"
								src="/assets/speculation.png"
								alt="🔁"/>
						}
					} else {
						html! { }
					}}
		     		<svg class="zone-lines"
		     			viewBox="0 0 100 20"
		     			style={{
		     				let h = if props.game.in_zone {
								let lines = props.game.board.matrix
								.iter()
								.filter(|r|
									**r == [Some(Mino::Gray); 10])
								.count();
								lines as f64 / 2.0
							} else {
								10.0
							}.clamp(1.0, 19.0);
		     				format!{"bottom: {}%;",
								(h - 1.0) / (26.0 - 2.0) * 100.0}
		     				}}>
						<text x="50%" y="60%">{
							if props.game.in_zone {
								let lines = props.game.board.matrix
									.iter()
									.filter(|r|
										**r == [Some(Mino::Gray); 10])
									.count();
								match lines {
									0 | 1 => "".to_string(),
									l => format!{"{} LINES", l},
								}
							} else {
								match props.last_clear {
									Some(Clear::ZoneClear(l)) =>
										match l {
											5 => "PENTRIX",
											6 => "HEXTRIX",
											7 => "SEPTRIX",
											8 => "OCTORIX",
											9 => "PENDECATRIX",
											10 => "DECATRIX",
											11 => "UNDECATRIX",
											12 => "DODECATRIX",
											13 => "TRIDECATRIX",
											14 => "QUADECATRIX",
											15 => "DECAPENTRIX",
											16 => "DECAHEXTRIX",
											17 => "DECASEPTRIX",
											18 => "PERFECTRIX",
											19 => "PENULTIMARIX",
											20 => "ULTIMATRIX",
											21 => "KIRBTRIX",
											22 => "IMPOSSITRIX",
											23 => "INFINITRIX",
											24 => "ELECTRIX",
											25 => "ELECTRIX+",
											26 => "ELECTRIX++",
											_ => ""
										},
									_ => ""
								}.to_string()
							}
						}</text>
					</svg>
			</div>
        	<div class="side-column">
        		<div class="top-right">
        			{props.top_right.clone()}
        		</div>
        		<div class="middle-right">
	        		<p><strong>{"NEXT"}</strong></p>
	        		<QueueComponent
	        			queue={props.game.queue.clone()}
	        			num_speculative={props.num_revealed}
						onbutton={props.onbutton.reform(
							|e: ButtonEvent<QueueButton>| e.map(|b|
								GameButton::Queue(b)))}/>
        		</div>
        		<div class="bottom-right">
        			{props.bottom_right.clone()}
        		</div>
            </div>
        </div>
	}
}
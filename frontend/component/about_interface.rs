use ztrix::game::Game;
use controller::input_handler::ButtonEvent;
use component::button::ButtonComponent;
use component::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
#[derive(Default)]
pub struct Props {
	#[prop_or_default]
	pub game: Game,
}

#[function_component(AboutInterface)]
pub fn about_interface(props: &Props) -> Html {
	let game = props.game.clone();
	let history = use_history()
		.expect("should be a history");
	html! {
		<div class="interface">
			<div class="logo-row">
				<ButtonComponent
					onbutton={Callback::from(
						move |e: ButtonEvent<()>|
							if let ButtonEvent::Press(_) = e {
								history.push(Route::GameGame{
									game: game.clone()
								});
							})}>
					<img src="/assets/play.png" alt="Play"/>
				</ButtonComponent>
				<img class="logo"
					src="/assets/logo.png" alt="ZTrix"/>
			</div>
			<div class="scrollable paragraphs">
				<p>{"ZTrix is a block stacking game created \
					by Electra with a focus on puzzles. Unlike \
					many similar games, which provide incentives \
					to play quickly, ZTrix is all about strategy. \
					There are no mistakes - use the built-in Undo \
					and Redo buttons to experiment with what \
					options are available. You can even use \
					Reroll to alter upcoming pieces if you \
					want - the future is yours to make!
				"}</p>
				<hr/>
				<p>{"Press Edit to create a copy of your game \
					that you have full control over. Draw on \
					the board, edit the next pieces, and more! \
					Then press Export and share the link with \
					your friends! You can use this to puzzle \
					them with challenges such as clearing the \
					full board or trying to maximize lines \
					cleared in the Zone.
				"}</p>
				<hr/>
				<p>{"Need people to share your puzzles and \
					strategies with? "}
					<a href="https://discord.gg/MGhqCBDGNH">
						{"Join our Discord!"}
					</a>
				</p>
			</div>
		</div>
	}
}
use component::about_interface::AboutInterface;
use component::config_interface::ConfigInterface;
use component::play_interface::PlayInterface;
use ztrix::game::Game;
use yew::prelude::*;
use yew_router::prelude::*;
use ztrix::puzzle::Puzzle;

use crate::component::edit_interface::EditInterface;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/play")]
    Play,
    #[at("/play/:game")]
    PlayGame { game: Game },
    #[at("/game")]
    Game,
    #[at("/game/:game")]
    GameGame { game: Game },

    #[at("/puzzle")]
    Puzzle,
    #[at("/puzzle/:puzzle")]
    PuzzlePuzzle { puzzle: Puzzle },

    #[at("/edit")]
    Edit,
    #[at("/edit/:puzzle")]
    EditPuzzle { puzzle: Puzzle },

    #[at("/config")]
    Config,
    #[at("/settings")]
    Settings,
    #[at("/controls")]
    Controls,

    #[at("/about")]
    About,
    #[at("/help")]
    Help,

    #[at("/random")]
    Random,
    #[at("/random/:difficulty")]
    RandomDifficulty{ difficulty: u32 },
}

fn switch(route: &Route) -> Html {
    match route {
    	Route::Home | Route::Play | Route::Game | Route::Puzzle => html! {
        	<PlayInterface/>
        },
        Route::PlayGame { game } => html! {
            <PlayInterface puzzle={Puzzle::new(game.clone())}/>
        },
        Route::GameGame { game } => html! {
            <PlayInterface puzzle={Puzzle::new(game.clone())}/>
        },

        Route::PuzzlePuzzle { puzzle } => html! {
            <PlayInterface puzzle={puzzle.clone()}/>
        },

        Route::Edit => html! {
            <EditInterface/>
        },
        Route::EditPuzzle { puzzle } => html! {
            <EditInterface puzzle={puzzle.clone()}/>
        },

        Route::Config => html! {
            <ConfigInterface/>
        },
        Route::Settings => html! {
            <ConfigInterface/>
        },
        Route::Controls => html! {
            <ConfigInterface/>
        },

        Route::About => html! {
            <AboutInterface/>
        },
        Route::Help => html! {
            <AboutInterface/>
        },

        Route::Random => html! {
            <PlayInterface puzzle={Puzzle::generate_kirb_puzzle(2)}/>
        },
        Route::RandomDifficulty { difficulty } => html! {
            <PlayInterface puzzle={Puzzle::generate_kirb_puzzle(*difficulty)}/>
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}
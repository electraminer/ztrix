use component::about_interface::AboutInterface;
use component::config_interface::ConfigInterface;
use component::play_interface::PlayInterface;
use ztrix::game::Game;
use yew::prelude::*;
use yew_router::prelude::*;

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
    #[at("/edit")]
    Edit,
    #[at("/edit/:game")]
    EditGame { game: Game },
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
}

fn switch(route: &Route) -> Html {
    match route {
    	Route::Home | Route::Play | Route::Game =>html! {
        	<PlayInterface/>
        },
        Route::PlayGame { game } => html! {
            <PlayInterface game={game.clone()}/>
        },
        Route::GameGame { game } => html! {
            <PlayInterface game={game.clone()}/>
        },
        Route::Edit => html! {
            <EditInterface/>
        },
        Route::EditGame { game } => html! {
            <EditInterface game={game.clone()}/>
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
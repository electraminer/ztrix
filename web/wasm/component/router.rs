use component::binding_interface::BindingInterface;
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
    #[at("/settings")]
    Settings,
    #[at("/controls")]
    Controls,
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
        Route::Settings => html! {
            <BindingInterface/>
        },
        Route::Controls => html! {
            <BindingInterface/>
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
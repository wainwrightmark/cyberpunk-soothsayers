use crate::web::opening_view::OpeningView;
use yew::prelude::*;
use yew_router::prelude::*;

use super::prelude::ShareCardView;
use super::spread_view::SpreadView;
use crate::web::question_view::QuestionView;
use crate::web::restart_view::RestartView;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Opening,
    #[at("/question")]
    Question,

    #[at("/spread")]
    Spread,

    #[at("/restart")]
    Restart,

    #[at("/share")]
    Share,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>

    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Opening => {
            html! {
               html!{
                   <OpeningView />
               }
            }
        }

        Route::Question {} => html! {
           <QuestionView  />
        },
        Route::Spread {} => html! {

        <SpreadView />

         },
        Route::Restart {} => html! {
            <RestartView  />
        },
        Route::Share => {
            html!(
                <ShareCardView />
            )
        }
    }
}

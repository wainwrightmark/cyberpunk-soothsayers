use std::str::FromStr;

use crate::data::image_data::{ImageData, ImageType};
use crate::data::prelude::ImageMeta;
use crate::state::logging::LoggableEvent;
use crate::state::preferences_state::DarkModeState;
use crate::state::prelude::{CustomSpread, DataState, SetCustomSpreadMessage, BackToTopMessage};
use crate::web::landing_view::LandingView;
use crate::web::particles::*;
use crate::web::preferences_view::PreferencesView;

use itertools::Itertools;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use yew_hooks::{use_effect_once, use_search_param};
use yew_router::prelude::*;
use yewdux::prelude::{Dispatch, use_store};

use super::prelude::{CheatView, ShareCardView};
use super::spread_view::SpreadView;
use crate::web::advanced_view::AdvancedView;
use crate::web::question_view::QuestionView;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    NoRoute,

    #[at("/landing")]
    Landing,
    #[at("/question")]
    Question,

    #[at("/spread")]
    Spread,

    #[at("/advanced")]
    Advanced,

    #[at("/share")]
    Share,

    #[at("/preferences")]
    Preferences,

    #[at("/custom/:cards")]
    Custom { cards: String },

    #[at("/cheat/:cards")]
    Cheat { cards: String },
}

fn android_show_status() {
    #[cfg(feature = "android")]
    {
        use capacitor_bindings::status_bar::*;
        crate::web::capacitor::do_or_report_error(|| async { StatusBar::show().await });
    }
}
fn android_hide_status() {
    #[cfg(feature = "android")]
    {
        use capacitor_bindings::status_bar::*;
        crate::web::capacitor::do_or_report_error(|| async { StatusBar::hide().await });
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let ref_param = use_search_param("ref".to_string());
    let gclid_param = use_search_param("gclid".to_string());

    //Load the dark mode state here to make sure dark mode is set correctly
    let _dms  = use_store::<DarkModeState>();

    use_effect_once(|| {
        spawn_local(crate::web::startup::setup(ref_param, gclid_param));
        || ()
    });

    html! {

        <>
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
        </>

    }
}

fn switch(routes: Route) -> Html {
    use Route::*;
    match routes {
        NoRoute | Landing | Question | Advanced => {
            android_show_status();
        }
        Share | Spread => {
            android_hide_status();
        }
        _ => {}
    }

    match routes {
        Landing => {
            html! {
               <LandingView />
            }
        }
        Preferences => {
            html!(<PreferencesView/>)
        }

        Question {} => html! {
           <QuestionView  />
        },
        Spread {} => html! {
        <>
            <ParticlesCanvas />
            <SpreadView />
        </>

         },
        NoRoute {} => html! {
            html! {
               <LandingView />
            }
        },
        Advanced {} => html! {
            <AdvancedView  />
        },
        Share => {
            html!(
                <ShareCardView />
            )
        }
        Cheat { cards } => {
            html!(<CheatView {cards} />)
        }
        Custom { cards } => {
            let custom_spread = get_custom_spread(&cards);

            Dispatch::<DataState>::new().apply(SetCustomSpreadMessage {
                custom: custom_spread.into(),
            });

            Dispatch::<DataState>::new().apply(BackToTopMessage);
            let event = LoggableEvent::Custom {
                cards: cards.clone(),
            };
            LoggableEvent::try_log(event);

            html!(<SpreadView />)
        }
    }
}

fn get_custom_spread(str: &str) -> CustomSpread {
    let cards = str
        .split_terminator(',')
        .map(|id| match ImageMeta::from_str(id) {
            Ok(im) => im.image_data,
            Err(_) => ImageData {
                id: id.to_string().into(),
                image_type: ImageType::Custom,
                alt: "Custom Image".to_string().into(),
            },
        })
        .rev()
        .collect_vec();
    CustomSpread { cards }
}

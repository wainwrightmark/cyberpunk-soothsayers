use itertools::Itertools;
use strum::{EnumCount, IntoEnumIterator};
use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};
use yew_router::prelude::use_navigator;
use yewdux::prelude::{ use_store_value};

use super::app::Route;
use crate::{
    data::prelude::{Soothsayer, StarSign},
    state::prelude::CardPageState,
};

#[derive(Properties, PartialEq)]
pub struct SoothsayerProps {
    pub sign: Option<StarSign>,
}

#[function_component(SoothsayerView)]
pub fn soothsayer_view(props: &SoothsayerProps) -> Html {

    let navigator = use_navigator().unwrap();
    let node = use_node_ref();
    let swipe_state = use_swipe(node.clone());
    let state = use_state(|| Soothsayer::EvelynMusgrave);

    let current_value = *state;
    let current_index = Soothsayer::iter()
        .position(|x| x == current_value)
        .expect("Selected value was not one of the possible values.");
    let previous = current_value.previous().unwrap_or(current_value);
    let next = current_value.next().unwrap_or(current_value);

    let select_previous = {
        let state = state.clone();
        Callback::from(move |_| state.set(previous))
    };

    let select_next = {
        let state = state.clone();
        Callback::from(move |_| state.set(next))
    };

    let can_select_previous = current_index != 0;
    let can_select_next = current_index + 1 < Soothsayer::COUNT;

    let items = Soothsayer::iter()
        .map(|soothsayer| {
            let selected = current_value == soothsayer;
            let classes = if selected {
                classes!("carousel-item", "carousel-item-visible")
            } else {
                classes!("carousel-item", "carousel-item-hidden")
            };

            let onclick = {
                let soothsayer = soothsayer.clone();
                let sign = props.sign.clone();
                let navigator = navigator.clone();
                Callback::from(move |_e: MouseEvent| {
                    navigator.push(&Route::Question { sign: sign.into(), soothsayer });
                })
            };

            let select_previous = select_previous.clone();
            let select_next = select_next.clone();
            html!(
                <div class={classes}  >
                    <h5 class="soothsayer-name" style="text-align: center;">{"Choose Soothsayer"}</h5>

                    
                    <div>
                    <img class="soothsayer-image"
                    onclick={onclick.clone()}
                    src={format!("https://drive.google.com/uc?export=view&id={}", soothsayer.image_id()) }
                         alt={soothsayer.name()} />
                         <div class="carousel-actions" style="pointer-events: none;">
            <button id="carousel-button-prev" aria-label="Previous" disabled={!can_select_previous} onclick={select_previous} style="pointer-events: auto;">{"❰"}</button>
            <button id="carousel-button-next" aria-label="Next" disabled={!can_select_next} onclick={select_next} style="pointer-events: auto;">{"❱"}</button>

            </div>
                    </div>
                    <h4 class="soothsayer-name" style="text-align: center; margin-top: 3vh; margin-bottom: 1vh;">{soothsayer.name()}</h4>
                        <p class="soothsayer-text" style="margin-top: 0vh;" >
                        {soothsayer.description()}
                        </p>
                        <button onclick={onclick} class="nice-button" style="margin: auto; display: block;">{"Choose"}</button>
                </div>
            )
        })
        .collect_vec();

    // You can depend on direction/swiping etc.
    {
        let swipe_state = swipe_state.clone();
        use_effect_with_deps(
            move |direction| {
                // Do something based on direction.
                match **direction {
                    UseSwipeDirection::Left => state.set(next),
                    UseSwipeDirection::Right => state.set(previous),
                    _ => (),
                }
                || ()
            },
            swipe_state.direction,
        );
    }

    html! {
        <>
        <div class="site" style="overflow: hidden ">
            <div class="container" >
        <div class="sm-4 col" style="margin: auto; width: 90em; padding:unset;">
        <div class="carousel" ref={node}>
            {items}

            
        </div>
        </div>
        </div>
        </div>
        </>
    }
}
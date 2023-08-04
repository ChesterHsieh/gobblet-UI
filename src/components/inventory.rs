use yew::prelude::*;
use yew::{function_component, html, Properties};

use crate::components::gobblet_card::Gobblet;
use crate::state::{Card, RawCard};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub cards: Vec<Card>,
    pub dupe_on_flip: Callback<RawCard>,
}

#[function_component]
pub fn Inventory(props: &Props) -> Html {
    html! {
        <div class="inventory">
        { for props.cards[..3].iter().map(|card|
            html! {
                <Gobblet card={card.clone()} on_flip={&props.dupe_on_flip} />
            }
        ) }
        </div>
    }
}
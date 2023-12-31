use yew::prelude::*;
use yew::{function_component, html, Properties};

use crate::components::gobblet_card::Gobblet;
use crate::state::{Card, RawCard};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub cards: Vec<Card>,
    pub on_flip: Callback<RawCard>,
}
#[function_component]
pub fn Chessboard(props: &Props) -> Html {
    html! {
        <div class="chess-board">
        { for props.cards[..9].iter().map(|card|
            html! {
                <Gobblet card={card.clone()} on_flip={&props.on_flip} />
            }
        ) }
        </div>
    }
}

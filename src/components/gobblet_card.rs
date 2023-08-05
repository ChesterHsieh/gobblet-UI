use web_sys::MouseEvent;
use yew::Properties;
use yew::prelude::*;

use crate::state::{Card, RawCard};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub card: Card,
    pub on_flip: Callback<RawCard>,
}

#[function_component]
pub fn Gobblet(props: &Props) -> Html {
    let Props { card, on_flip } = props.clone();
    let Card {name, id } = card;
    let get_link_by_cardname = {
        match name {
            CardName::RED_LARGE => "public/red-large.jpg",
            CardName::RED_MEDIUM => "public/red-medium.jpg",
            CardName::RED_TINY => "public/red-tiny.jpg",
            CardName::BLUE_TINY => "public/blue-tiny.jpg",
            CardName::BLUE_MEDIUM => "public/blue-medium.jpg",
            CardName::BLUE_LARGE => "public/blue-large.jpg",
        }
        .to_string()
    };

    // let onclick = move |e: MouseEvent| {
    //     e.stop_propagation();
    //     (!flipped).then(|| {
    //         on_flip.emit(RawCard {
    //             id: id.clone(),
    //             name,
    //         })
    //     });
    // };

    html! {
      <div class="chess-board-card-container">
          <div class={classes!("card", flipped.then_some("flipped"))} {onclick}>
              <img class="front" src={get_link_by_cardname} alt="card" />
              <img class="back" src="public/back.png" alt="card" />
          </div>
      </div>
    }
}

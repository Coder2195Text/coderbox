use dioxus::prelude::*;

use crate::components::global::player::Player;
use crate::Route;

#[component]
pub fn Layout() -> Element {
  rsx! {
    div { class: "h-screen w-screen flex flex-col justify-center",
      div { class: "flex-grow", Outlet::<Route> {} }
      Player {}
    }
  }
}

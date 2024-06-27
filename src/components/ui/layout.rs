use dioxus::prelude::*;

use crate::components::ui::nav::Nav;
use crate::components::ui::player::Player;
use crate::Route;

#[component]
pub fn Layout() -> Element {
  rsx! {
    div { class: "h-screen w-screen flex flex-col justify-center",
      div { class: "flex-grow flex flex-row",
        Nav {}
        div { class: "p-2", Outlet::<Route> {} }
      }
      Player {}
    }
  }
}

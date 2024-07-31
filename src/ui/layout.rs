use dioxus::prelude::*;

use super::nav::Nav;
use super::player::Player;
use crate::Route;

#[component]
pub fn Layout() -> Element {
  rsx! {
    div { class: "h-screen w-screen flex flex-col justify-center",
      div { class: "flex-grow flex flex-row",
        Nav {}
        div { class: "p-2 w-full md:w-3/4 pb-20 h-[calc(100dvh-128px)]  md:h-[calc(100dvh-88px)] overflow-auto",
          Outlet::<Route> {}
        }
      }
      Player {}
    }
  }
}

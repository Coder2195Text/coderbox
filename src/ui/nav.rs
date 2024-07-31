use dioxus::prelude::*;
use dioxus_free_icons::icons::md_action_icons::MdHome;
use dioxus_free_icons::Icon;

use crate::Route;

#[component]
pub fn Nav() -> Element {
  rsx! (
    div { class: "p-2 b:w-full md:w-1/4 b:fixed md:relative bottom-0 left-0",
      h3 { class: "gap-2 items-center justify-center h-20 hidden md:flex",
        img { class: "h-10 w-10", src: "./public/icon.png" }
        "Coderbox"
      }
      div { class: "gradient-bg w-full rounded-md p-2 mb-0 flex flex-row md:flex-col gap-2  text-xl",
        Link {
          to: Route::Home {},
          class: "flex items-center gap-2 justify-center w-full",
          Icon { height: 24, width: 24, icon: MdHome }
          span { class: "hidden md:block", "Home" }
        }
        Link {
          to: Route::Home {},
          class: "flex items-center gap-2 justify-center w-full",
          Icon { height: 24, width: 24, icon: MdHome }
          span { class: "hidden md:block", "Home 2" }
        }
        Link {
          to: Route::Home {},
          class: "flex items-center gap-2 justify-center w-full",
          Icon { height: 24, width: 24, icon: MdHome }
          span { class: "hidden md:block", "Home 3 " }
        }
      }
    }
  )
}

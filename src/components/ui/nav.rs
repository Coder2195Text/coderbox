use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Nav() -> Element {
  rsx! {
    div { class: "gradient-bg p-2 w-44 rounded-md m-2 mb-0",
      Link { to: Route::Home {}, class: "flex items-center gap-2", "Home" }
    }
  }
}

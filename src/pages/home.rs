use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
  rsx! {
    div {
      h1 { "Home" }
    }
  }
}

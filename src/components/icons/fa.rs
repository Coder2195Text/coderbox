use dioxus::prelude::*;

use super::IconProps;

#[component]
pub fn FaRegCirclePlay(props: IconProps) -> Element {
  let IconProps { color, size } = props;
  let color = color.unwrap_or("currentColor".to_string());
  let size = size.unwrap_or("16px".to_string());

  rsx! {
    svg {
      "fill": color.to_string(),
      "viewBox": "0 0 512 512",
      width: size.to_string(),
      "xmlns": "http://www.w3.org/2000/svg",
      height: size.to_string(),
      "stroke-width": "0",
      "stroke": color.to_string(),
      path { "d": "M464 256A208 208 0 1 0 48 256a208 208 0 1 0 416 0zM0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256zM188.3 147.1c7.6-4.2 16.8-4.1 24.3 .5l144 88c7.1 4.4 11.5 12.1 11.5 20.5s-4.4 16.1-11.5 20.5l-144 88c-7.4 4.5-16.7 4.7-24.3 .5s-12.3-12.2-12.3-20.9V168c0-8.7 4.7-16.7 12.3-20.9z" }
    }
  }
}

#[component]
pub fn FaRegCirclePause(props: IconProps) -> Element {
  let IconProps { color, size } = props;
  let color = color.unwrap_or("currentColor".to_string());
  let size = size.unwrap_or("16px".to_string());

  rsx! {
    svg {
      "fill": color.to_string(),
      "viewBox": "0 0 512 512",
      width: size.to_string(),
      "xmlns": "http://www.w3.org/2000/svg",
      height: size.to_string(),
      "stroke-width": "0",
      "stroke": color.to_string(),
      path { "d": "M464 256A208 208 0 1 0 48 256a208 208 0 1 0 416 0zM0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256zm224-72V328c0 13.3-10.7 24-24 24s-24-10.7-24-24V184c0-13.3 10.7-24 24-24s24 10.7 24 24zm112 0V328c0 13.3-10.7 24-24 24s-24-10.7-24-24V184c0-13.3 10.7-24 24-24s24 10.7 24 24z" }
    }
  }
}

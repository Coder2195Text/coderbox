use dioxus::prelude::*;

use super::IconProps;

#[component]
pub fn FaRegCirclePlay(props: IconProps) -> Element {
  let IconProps { color, size } = props;
  let color = color.unwrap_or("currentColor".to_string());
  let size = size.unwrap_or("18px".to_string());

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
  let size = size.unwrap_or("18px".to_string());

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

#[component]
pub fn FaMicrophone(props: IconProps) -> Element {
  let IconProps { color, size } = props;
  let color = color.unwrap_or("currentColor".to_string());
  let size = size.unwrap_or("18px".to_string());

  rsx! {
    svg {
      "fill": color.to_string(),
      "viewBox": "0 0 352 512",
      width: size.to_string(),
      "xmlns": "http://www.w3.org/2000/svg",
      height: size.to_string(),
      "stroke-width": "0",
      "stroke": color.to_string(),
      path { "d": "M176 352c53.02 0 96-42.98 96-96V96c0-53.02-42.98-96-96-96S80 42.98 80 96v160c0 53.02 42.98 96 96 96zm160-160h-16c-8.84 0-16 7.16-16 16v48c0 74.8-64.49 134.82-140.79 127.38C96.71 376.89 48 317.11 48 250.3V208c0-8.84-7.16-16-16-16H16c-8.84 0-16 7.16-16 16v40.16c0 89.64 63.97 169.55 152 181.69V464H96c-8.84 0-16 7.16-16 16v16c0 8.84 7.16 16 16 16h160c8.84 0 16-7.16 16-16v-16c0-8.84-7.16-16-16-16h-56v-33.77C285.71 418.47 352 344.9 352 256v-48c0-8.84-7.16-16-16-16z" }
    }
  }
}

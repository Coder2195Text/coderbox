use dioxus::prelude::*;

use super::IconProps;

#[component]
pub fn IoVolumeOffOutline(props: IconProps) -> Element {
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
      path {
        "stroke-width": "32",
        "stroke-linejoin": "round",
        "d": "M237.65 192H168a8 8 0 0 0-8 8v112a8 8 0 0 0 8 8h69.65a16 16 0 0 1 10.14 3.63l91.47 75a8 8 0 0 0 12.74-6.46V119.83a8 8 0 0 0-12.74-6.44l-91.47 75a16 16 0 0 1-10.14 3.61z",
        "fill": "none",
        "stroke-linecap": "round"
      }
    }
  }
}

#[component]
pub fn IoVolumeLowOutline(props: IconProps) -> Element {
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
      path {
        "fill": "none",
        "stroke-linecap": "round",
        "stroke-linejoin": "round",
        "stroke-width": "32",
        "d": "M189.65 192H120a8 8 0 0 0-8 8v112a8 8 0 0 0 8 8h69.65a16 16 0 0 1 10.14 3.63l91.47 75a8 8 0 0 0 12.74-6.46V119.83a8 8 0 0 0-12.74-6.44l-91.47 75a16 16 0 0 1-10.14 3.61zM384 320c9.74-19.41 16-40.81 16-64 0-23.51-6-44.4-16-64"
      }
    }
  }
}

#[component]
pub fn IoVolumeMediumOutline(props: IconProps) -> Element {
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
      path {
        "stroke-width": "32",
        "d": "M157.65 192H88a8 8 0 0 0-8 8v112a8 8 0 0 0 8 8h69.65a16 16 0 0 1 10.14 3.63l91.47 75a8 8 0 0 0 12.74-6.46V119.83a8 8 0 0 0-12.74-6.44l-91.47 75a16 16 0 0 1-10.14 3.61zM352 320c9.74-19.41 16-40.81 16-64 0-23.51-6-44.4-16-64m48 176c19.48-34 32-64 32-112s-12-77.7-32-112",
        "fill": "none",
        "stroke-linecap": "round",
        "stroke-linejoin": "round"
      }
    }
  }
}

#[component]
pub fn IoVolumeHighOutline(props: IconProps) -> Element {
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
      path {
        "stroke-linecap": "round",
        "fill": "none",
        "stroke-width": "32",
        "stroke-linejoin": "round",
        "d": "M126 192H56a8 8 0 0 0-8 8v112a8 8 0 0 0 8 8h69.65a15.93 15.93 0 0 1 10.14 3.54l91.47 74.89A8 8 0 0 0 240 392V120a8 8 0 0 0-12.74-6.43l-91.47 74.89A15 15 0 0 1 126 192zm194 128c9.74-19.38 16-40.84 16-64 0-23.48-6-44.42-16-64m48 176c19.48-33.92 32-64.06 32-112s-12-77.74-32-112m48 272c30-46 48-91.43 48-160s-18-113-48-160"
      }
    }
  }
}

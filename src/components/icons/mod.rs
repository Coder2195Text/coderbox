pub mod fa;
pub mod io;

use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Props)]
pub struct IconProps {
  size: Option<String>,
  color: Option<String>,
}

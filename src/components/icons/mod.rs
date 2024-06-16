pub mod fa;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Props)]
pub struct IconProps {
  size: Option<String>,
  color: Option<String>,
}

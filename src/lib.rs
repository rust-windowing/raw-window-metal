#![cfg(any(target_os = "macos", target_os = "ios"))]
#![allow(clippy::missing_safety_doc, clippy::let_unit_value)]

#[macro_use]
extern crate objc;

use objc::runtime::Object;

pub mod appkit;
pub mod uikit;

pub type CAMetalLayer = *mut Object;

pub enum Layer {
    Existing(CAMetalLayer),
    Allocated(CAMetalLayer),
}

#[macro_use]
extern crate objc;

use objc::runtime::Object;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "ios")]
pub mod ios;

pub type CAMetalLayer = *mut Object;

pub enum Layer {
    Existing(CAMetalLayer),
    Allocated(CAMetalLayer),
    None,
}
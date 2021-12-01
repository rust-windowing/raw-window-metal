use objc::runtime::Object;

#[cfg(target_os = "macos")]
pub mod appkit;
#[cfg(target_os = "ios")]
pub mod uikit;

pub type CAMetalLayer = *mut Object;

pub enum Layer {
    Existing(CAMetalLayer),
    Allocated(CAMetalLayer),
    None,
}

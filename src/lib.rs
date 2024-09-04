#![cfg(target_vendor = "apple")]
#![allow(clippy::missing_safety_doc)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg_hide), doc(cfg_hide(doc)))]
#![deny(unsafe_op_in_unsafe_fn)]

use objc2::rc::Retained;
use objc2_quartz_core::CAMetalLayer;
use std::ffi::c_void;

#[cfg(any(target_os = "macos", doc))]
pub mod appkit;

#[cfg(any(not(target_os = "macos"), doc))]
pub mod uikit;

/// A wrapper around [`CAMetalLayer`].
pub struct Layer {
    layer: Retained<CAMetalLayer>,
    pre_existing: bool,
}

impl Layer {
    /// Get a pointer to the underlying [`CAMetalLayer`]. The pointer is valid
    /// for at least as long as the [`Layer`] is valid, but can be extended by
    /// retaining it.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use objc2::rc::Retained;
    /// use objc2_quartz_core::CAMetalLayer;
    /// use raw_window_metal::Layer;
    ///
    /// let layer: Layer;
    /// # layer = unimplemented!();
    ///
    /// let layer: *mut CAMetalLayer = layer.as_ptr().cast();
    /// // SAFETY: The pointer is a valid `CAMetalLayer`.
    /// let layer = unsafe { Retained::retain(layer).unwrap() };
    ///
    /// // Use the `CAMetalLayer` here.
    /// ```
    #[inline]
    pub fn as_ptr(&self) -> *mut c_void {
        let ptr: *const CAMetalLayer = Retained::as_ptr(&self.layer);
        ptr as *mut _
    }

    /// Whether `raw-window-metal` created a new [`CAMetalLayer`] for you.
    #[inline]
    pub fn pre_existing(&self) -> bool {
        self.pre_existing
    }
}

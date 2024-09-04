#![cfg(target_vendor = "apple")]
#![allow(clippy::missing_safety_doc)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg_hide), doc(cfg_hide(doc)))]
#![deny(unsafe_op_in_unsafe_fn)]

use core::ffi::c_void;
use core::hash;
use core::panic::{RefUnwindSafe, UnwindSafe};
use objc2::rc::Retained;
use objc2_quartz_core::CAMetalLayer;

#[cfg(any(target_os = "macos", doc))]
pub mod appkit;

#[cfg(any(not(target_os = "macos"), doc))]
pub mod uikit;

/// A wrapper around [`CAMetalLayer`].
#[doc(alias = "CAMetalLayer")]
#[derive(Debug, Clone)]
pub struct Layer {
    layer: Retained<CAMetalLayer>,
    pre_existing: bool,
}

impl PartialEq for Layer {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.layer.eq(&other.layer)
    }
}

impl Eq for Layer {}

impl hash::Hash for Layer {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.layer.hash(state);
    }
}

// SAFETY: `CAMetalLayer` is thread safe, like most things in Core Animation, see:
// https://developer.apple.com/documentation/quartzcore/catransaction/1448267-lock?language=objc
// https://stackoverflow.com/questions/76250226/how-to-render-content-of-calayer-on-a-background-thread
//
// TODO(madsmtm): Move this to `objc2-quartz-core`.
unsafe impl Send for Layer {}
unsafe impl Sync for Layer {}

// Layer methods may panic, but that won't leave the layer in an invalid state.
//
// TODO(madsmtm): Move this to `objc2-quartz-core`.
impl UnwindSafe for Layer {}
impl RefUnwindSafe for Layer {}

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

    /// If `raw-window-metal` created a new [`CAMetalLayer`] for you, this returns `false`.
    ///
    /// This may be useful if you want to override some part of `raw-window-metal`'s behaviour, and
    /// need to do so based on whether it ended up creating a layer or not.
    ///
    /// You should try to avoid this, and instead:
    /// - Modify `CALayer` properties on the layer that you created this from.
    /// - Modify `CAMetalLayer` properties on the layer returned from `as_ptr`.
    #[inline]
    pub fn pre_existing(&self) -> bool {
        self.pre_existing
    }
}

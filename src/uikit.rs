use crate::Layer;
use objc2::rc::Retained;
use objc2_foundation::NSObjectProtocol;
use objc2_quartz_core::CAMetalLayer;
use raw_window_handle::UiKitWindowHandle;
use std::{ffi::c_void, ptr::NonNull};

/// Get or create a new [`Layer`] associated with the given
/// [`UiKitWindowHandle`].
///
/// # Safety
///
/// The handle must be valid.
pub unsafe fn metal_layer_from_handle(handle: UiKitWindowHandle) -> Layer {
    if let Some(_ui_view_controller) = handle.ui_view_controller {
        // TODO: ui_view_controller support
    }
    unsafe { metal_layer_from_ui_view(handle.ui_view) }
}

/// Get or create a new [`Layer`] associated with the given `UIView`.
///
/// # Safety
///
/// The view must be a valid instance of `UIView`.
pub unsafe fn metal_layer_from_ui_view(view: NonNull<c_void>) -> Layer {
    // SAFETY: Caller ensures that the view is a `UIView`.
    let view = unsafe { view.cast::<objc2_ui_kit::UIView>().as_ref() };

    let main_layer = view.layer();

    // Check if the view's layer is already a `CAMetalLayer`.
    let render_layer = if main_layer.is_kind_of::<CAMetalLayer>() {
        // SAFETY: Just checked that the layer is a `CAMetalLayer`.
        let layer = unsafe { Retained::cast::<CAMetalLayer>(main_layer) };
        Layer {
            layer,
            pre_existing: true,
        }
    } else {
        // If the main layer is not a `CAMetalLayer`, we create a
        // `CAMetalLayer` sublayer and use it instead.
        //
        // Unlike on macOS, we cannot replace the main view as `UIView` does
        // not allow it (when `NSView` does).
        let layer = unsafe { CAMetalLayer::new() };

        let bounds = main_layer.bounds();
        layer.setFrame(bounds);

        main_layer.addSublayer(&layer);

        Layer {
            layer,
            pre_existing: false,
        }
    };

    if let Some(window) = view.window() {
        view.setContentScaleFactor(window.screen().nativeScale());
    }

    render_layer
}

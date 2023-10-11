use crate::{CAMetalLayer, Layer};
use core_graphics::{base::CGFloat, geometry::CGRect};
use objc::{
    msg_send,
    runtime::{BOOL, YES},
};
use raw_window_handle::UiKitWindowHandle;
use std::{ffi::c_void, ptr::NonNull};

///
pub unsafe fn metal_layer_from_handle(handle: UiKitWindowHandle) -> Layer {
    if let Some(_ui_view_controller) = handle.ui_view_controller {
        // TODO: ui_view_controller support
    }
    metal_layer_from_ui_view(handle.ui_view)
}

///
pub unsafe fn metal_layer_from_ui_view(view: NonNull<c_void>) -> Layer {
    let view: cocoa::base::id = view.cast().as_ptr();
    let main_layer: CAMetalLayer = msg_send![view, layer];

    let class = class!(CAMetalLayer);
    let is_valid_layer: BOOL = msg_send![main_layer, isKindOfClass: class];
    let render_layer = if is_valid_layer == YES {
        Layer::Existing(main_layer)
    } else {
        // If the main layer is not a CAMetalLayer, we create a CAMetalLayer sublayer and use it instead.
        // Unlike on macOS, we cannot replace the main view as UIView does not allow it (when NSView does).
        let new_layer: CAMetalLayer = msg_send![class, new];

        let bounds: CGRect = msg_send![main_layer, bounds];
        let () = msg_send![new_layer, setFrame: bounds];

        let () = msg_send![main_layer, addSublayer: new_layer];
        Layer::Allocated(new_layer)
    };

    let window: cocoa::base::id = msg_send![view, window];
    if !window.is_null() {
        let screen: cocoa::base::id = msg_send![window, screen];
        assert!(!screen.is_null(), "window is not attached to a screen");

        let scale_factor: CGFloat = msg_send![screen, nativeScale];
        let () = msg_send![view, setContentScaleFactor: scale_factor];
    }

    render_layer
}

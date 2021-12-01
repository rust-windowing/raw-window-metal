use crate::{CAMetalLayer, Layer};
use core_graphics::{base::CGFloat, geometry::CGRect};
use objc::{
    msg_send,
    runtime::{Object, BOOL, YES},
};
use raw_window_handle::uikit::UiKitHandle;
use std::{ffi::c_void, mem};

///
pub unsafe fn metal_layer_from_handle(handle: UiKitHandle) -> Layer {
    if !handle.ui_view.is_null() {
        metal_layer_from_ui_view(handle.ui_view)
    } else if !handle.ui_window.is_null() {
        metal_layer_from_ui_window(handle.ui_window)
    } else {
        // TODO: ui_window & ui_view_controller support
        Layer::None
    }
}

///
pub unsafe fn metal_layer_from_ui_view(view: *mut c_void) -> Layer {
    let view: cocoa::base::id = mem::transmute(view);
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

///
pub unsafe fn metal_layer_from_ui_window(window: *mut c_void) -> Layer {
    let ui_window = window as *mut Object;
    let ui_view = msg_send![ui_window, contentView];
    metal_layer_from_ui_view(ui_view)
}

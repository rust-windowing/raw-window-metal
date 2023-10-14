use crate::{CAMetalLayer, Layer};
use core::ffi::c_void;
use core_graphics::{base::CGFloat, geometry::CGRect};
use objc::{
    msg_send,
    runtime::{BOOL, YES},
};
use raw_window_handle::AppKitWindowHandle;
use std::ptr::NonNull;

///
pub unsafe fn metal_layer_from_handle(handle: AppKitWindowHandle) -> Layer {
    metal_layer_from_ns_view(handle.ns_view)
}

///
pub unsafe fn metal_layer_from_ns_view(view: NonNull<c_void>) -> Layer {
    let view: cocoa::base::id = view.cast().as_ptr();

    // Check if the view is a CAMetalLayer
    let class = class!(CAMetalLayer);
    let is_actually_layer: BOOL = msg_send![view, isKindOfClass: class];
    if is_actually_layer == YES {
        return Layer::Existing(view);
    }

    // Check if the view contains a valid CAMetalLayer
    let existing: CAMetalLayer = msg_send![view, layer];
    let use_current = if existing.is_null() {
        false
    } else {
        let result: BOOL = msg_send![existing, isKindOfClass: class];
        result == YES
    };

    let render_layer = if use_current {
        Layer::Existing(existing)
    } else {
        // Allocate a new CAMetalLayer for the current view
        let layer: CAMetalLayer = msg_send![class, new];
        let () = msg_send![view, setLayer: layer];
        let () = msg_send![view, setWantsLayer: YES];
        let bounds: CGRect = msg_send![view, bounds];
        let () = msg_send![layer, setBounds: bounds];

        let window: cocoa::base::id = msg_send![view, window];
        if !window.is_null() {
            let scale_factor: CGFloat = msg_send![window, backingScaleFactor];
            let () = msg_send![layer, setContentsScale: scale_factor];
        }

        Layer::Allocated(layer)
    };

    let _: *mut c_void = msg_send![view, retain];
    render_layer
}

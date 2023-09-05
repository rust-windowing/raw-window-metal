use core::ffi::c_void;
use objc2::rc::Retained;
use objc2::ClassType;
use objc2_foundation::{NSObject, NSObjectProtocol};
use objc2_quartz_core::CAMetalLayer;
use raw_window_handle::AppKitWindowHandle;
use std::ptr::NonNull;

use crate::Layer;

///
pub unsafe fn metal_layer_from_handle(handle: AppKitWindowHandle) -> Layer {
    unsafe { metal_layer_from_ns_view(handle.ns_view) }
}

///
pub unsafe fn metal_layer_from_ns_view(view: NonNull<c_void>) -> Layer {
    // SAFETY: Caller ensures that the view is valid.
    let obj = unsafe { view.cast::<NSObject>().as_ref() };

    // Check if the view is a CAMetalLayer
    if obj.is_kind_of::<CAMetalLayer>() {
        // SAFETY: Just checked that the view is a `CAMetalLayer`.
        let layer = unsafe { view.cast::<CAMetalLayer>().as_ref() };
        return Layer {
            layer: layer.retain(),
            pre_existing: true,
        };
    }
    // Otherwise assume the view is `NSView`
    let view = unsafe { view.cast::<objc2_app_kit::NSView>().as_ref() };

    // Check if the view contains a valid CAMetalLayer
    let existing = unsafe { view.layer() };
    if let Some(existing) = existing {
        if existing.is_kind_of::<CAMetalLayer>() {
            // SAFETY: Just checked that the layer is a `CAMetalLayer`.
            let layer = unsafe { Retained::cast::<CAMetalLayer>(existing) };
            return Layer {
                layer,
                pre_existing: true,
            };
        }
    }

    // If the layer was not `CAMetalLayer`, allocate a new one for the view
    let layer = unsafe { CAMetalLayer::new() };
    unsafe { view.setLayer(Some(&layer)) };
    view.setWantsLayer(true);
    layer.setBounds(view.bounds());

    if let Some(window) = view.window() {
        let scale_factor = window.backingScaleFactor();
        layer.setContentsScale(scale_factor);
    }

    Layer {
        layer,
        pre_existing: false,
    }
}

use gpui::{prelude::*, App, Window};
use gpui_component::WindowExt as _;

pub fn open_about_dialog(window: &mut Window, cx: &mut App) {
    window.open_dialog(cx, |dialog, _, _| {
        dialog
            .title("About Component Playground")
            .child("A native desktop component gallery built with GPUI and Longbridge GPUI Kit.")
            .child("Explore interactive controls, forms, overlays, layouts, and visual components.")
            .child("Version 0.1.0")
    });
}

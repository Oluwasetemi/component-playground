use crate::StorySection;
use gpui::{
    actions, div, prelude::*, Bounds, Context, IntoElement, MouseButton, MouseDownEvent, Pixels,
    Point, Render, Window,
};
use gpui_component::{
    native_menu::NativeMenu, scroll::ScrollableElement as _, v_flex, ActiveTheme, ElementExt,
};

actions!(playground_native_menu, [MenuClicked]);

pub struct NativeMenuStory;

impl NativeMenuStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for NativeMenuStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let trigger = std::rc::Rc::new(std::cell::Cell::new(Bounds::<Pixels>::default()));
        let bounds = trigger.clone();
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(gpui::px(24.)).child("Native Menu"))
                    .child("Operating-system menus that can extend beyond the window bounds."),
            )
            .child(StorySection::new(
                "Native Menu Trigger",
                div()
                    .on_prepaint(move |current, _, _| bounds.set(current))
                    .child(
                        div()
                            .id("native-menu-trigger")
                            .w(gpui::px(360.))
                            .h(gpui::px(100.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .border_1()
                            .border_color(cx.theme().border)
                            .child("Right-click to open the native menu")
                            .on_mouse_down(
                                MouseButton::Right,
                                cx.listener(move |_, _: &MouseDownEvent, window, cx| {
                                    let position = Point {
                                        x: trigger.get().origin.x,
                                        y: trigger.get().origin.y + trigger.get().size.height,
                                    };
                                    NativeMenu::new()
                                        .menu("Copy", Box::new(MenuClicked))
                                        .menu("Paste", Box::new(MenuClicked))
                                        .separator()
                                        .menu_with_disabled(
                                            "Disabled item",
                                            true,
                                            Box::new(MenuClicked),
                                        )
                                        .show(position, window, cx);
                                }),
                            ),
                    ),
            ))
    }
}

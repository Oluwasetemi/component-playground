use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    menu::{ContextMenuExt, PopupMenuItem},
    scroll::ScrollableElement as _,
    v_flex, ActiveTheme,
};

pub struct ContextMenuStory;

impl ContextMenuStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for ContextMenuStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Context Menu"))
                    .child("Right-click surfaces to expose local actions."),
            )
            .child(StorySection::new(
                "Context Actions",
                div()
                    .w(px(420.))
                    .h(px(140.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .border_1()
                    .border_color(cx.theme().border)
                    .child("Right-click to open the context menu")
                    .context_menu(|menu, _, _| {
                        menu.item(PopupMenuItem::new("Cut"))
                            .item(PopupMenuItem::new("Copy"))
                            .item(PopupMenuItem::new("Paste"))
                            .separator()
                            .item(PopupMenuItem::new("Properties"))
                    }),
            ))
    }
}

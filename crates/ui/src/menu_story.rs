use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::Button,
    menu::{ContextMenuExt, DropdownMenu as _, PopupMenuItem},
    scroll::ScrollableElement as _,
    v_flex, ActiveTheme,
};

pub struct MenuStory;

impl MenuStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for MenuStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Menu"))
                    .child("Popup and context menus with actions, states, and submenus."),
            )
            .child(StorySection::new(
                "Popup Menu",
                Button::new("menu-popup")
                    .label("Open Menu")
                    .outline()
                    .dropdown_menu(|menu, _, _| {
                        menu.item(PopupMenuItem::new("New File"))
                            .item(PopupMenuItem::new("Open Folder"))
                            .separator()
                            .item(PopupMenuItem::new("Disabled Item").disabled(true))
                    }),
            ))
            .child(StorySection::new(
                "Context Menu",
                div()
                    .w(px(360.))
                    .h(px(100.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .border_1()
                    .border_color(cx.theme().border)
                    .child("Right-click this area")
                    .context_menu(|menu, _, _| {
                        menu.item(PopupMenuItem::new("Copy"))
                            .item(PopupMenuItem::new("Paste"))
                            .separator()
                            .item(PopupMenuItem::new("Inspect"))
                    }),
            ))
    }
}

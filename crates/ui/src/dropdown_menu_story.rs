use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button, ButtonVariants as _},
    menu::{DropdownMenu as _, PopupMenuItem},
    scroll::ScrollableElement as _,
    v_flex,
};

pub struct DropdownMenuStory;

impl DropdownMenuStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for DropdownMenuStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Dropdown Menu"))
                    .child("Menus anchored to buttons for compact command surfaces."),
            )
            .child(StorySection::new(
                "Actions",
                gpui_component::h_flex()
                    .gap_3()
                    .child(
                        Button::new("dropdown-actions")
                            .label("Actions")
                            .primary()
                            .dropdown_menu(|menu, _, _| {
                                menu.item(PopupMenuItem::new("Rename"))
                                    .item(PopupMenuItem::new("Duplicate"))
                                    .separator()
                                    .item(PopupMenuItem::new("Archive"))
                            }),
                    )
                    .child(
                        Button::new("dropdown-more")
                            .label("More")
                            .outline()
                            .dropdown_menu(|menu, _, _| {
                                menu.item(PopupMenuItem::new("Share"))
                                    .item(PopupMenuItem::new("Move to folder"))
                                    .item(PopupMenuItem::new("Download"))
                            }),
                    ),
            ))
    }
}

use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    input::{Textarea, TextareaState},
    scroll::ScrollableElement as _,
    v_flex,
};

pub struct TextareaStory {
    short: gpui::Entity<TextareaState>,
    growing: gpui::Entity<TextareaState>,
}

impl TextareaStory {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            short: cx.new(|cx| TextareaState::new(window, cx).placeholder("Write a short note")),
            growing: cx.new(|cx| {
                TextareaState::new(window, cx)
                    .auto_grow(3, 8)
                    .default_value("This textarea grows as more content is entered.")
            }),
        }
    }
}

impl Render for TextareaStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Textarea"))
                    .child("Multi-line input for longer text and notes."),
            )
            .child(StorySection::new(
                "Fixed Height",
                Textarea::new(&self.short).h(px(120.)).w(px(420.)),
            ))
            .child(StorySection::new(
                "Auto Grow",
                Textarea::new(&self.growing).w(px(420.)),
            ))
    }
}
